#![no_std]
#![no_main]

use core::fmt::{self, Write};
use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{AtomicBool, Ordering};

// Hardware addresses
const RCC_BASE: usize = 0x4002_3800;
const RCC_AHB1ENR: *mut u32 = (RCC_BASE + 0x30) as *mut u32;

const GPIOD_BASE: usize = 0x4002_0C00;
const GPIOD_MODER: *mut u32 = (GPIOD_BASE + 0x00) as *mut u32;
const GPIOD_AFRL: *mut u32 = (GPIOD_BASE + 0x20) as *mut u32;
const GPIOD_AFRH: *mut u32 = (GPIOD_BASE + 0x24) as *mut u32;

const USART3_BASE: usize = 0x4000_4800;
const USART3_SR: *mut u32 = (USART3_BASE + 0x00) as *mut u32;
const USART3_DR: *mut u32 = (USART3_BASE + 0x04) as *mut u32;
const USART3_BRR: *mut u32 = (USART3_BASE + 0x08) as *mut u32;
const USART3_CR1: *mut u32 = (USART3_BASE + 0x0C) as *mut u32;

const RCC_APB1ENR: *mut u32 = (RCC_BASE + 0x40) as *mut u32;
const USART_SR_TXE: u32 = 1 << 7;

// Flash emulation (in RAM for demo)
const FLASH_SIZE: usize = 4096; // 4KB
static mut FLASH_MEMORY: [u8; FLASH_SIZE] = [0xFF; FLASH_SIZE];

// Key-Value storage structure
const MAX_ENTRIES: usize = 16;
const MAX_KEY_LEN: usize = 16;
const MAX_VALUE_LEN: usize = 64;

#[repr(C)]
#[derive(Clone, Copy)]
struct Entry {
    key: [u8; MAX_KEY_LEN],
    key_len: u8,
    value: [u8; MAX_VALUE_LEN],
    value_len: u8,
    valid: u8, // 0xFF = valid, 0x00 = deleted
}

impl Entry {
    const fn new() -> Self {
        Entry {
            key: [0; MAX_KEY_LEN],
            key_len: 0,
            value: [0; MAX_VALUE_LEN],
            value_len: 0,
            valid: 0x00,
        }
    }
}

struct Storage {
    flash: &'static mut [u8; FLASH_SIZE],
}

impl Storage {
    fn new() -> Self {
        Storage {
            flash: unsafe { &mut FLASH_MEMORY },
        }
    }

    fn format(&mut self) {
        // Erase all flash (set to 0xFF)
        for byte in self.flash.iter_mut() {
            *byte = 0xFF;
        }
    }

    fn write_entry(&mut self, index: usize, entry: &Entry) -> Result<(), &'static str> {
        if index >= MAX_ENTRIES {
            return Err("Index out of bounds");
        }

        let offset = index * core::mem::size_of::<Entry>();
        let entry_bytes = unsafe {
            core::slice::from_raw_parts(
                entry as *const Entry as *const u8,
                core::mem::size_of::<Entry>(),
            )
        };

        self.flash[offset..offset + entry_bytes.len()].copy_from_slice(entry_bytes);
        Ok(())
    }

    fn read_entry(&self, index: usize) -> Result<Entry, &'static str> {
        if index >= MAX_ENTRIES {
            return Err("Index out of bounds");
        }

        let offset = index * core::mem::size_of::<Entry>();
        let mut entry = Entry::new();
        let entry_bytes = unsafe {
            core::slice::from_raw_parts_mut(
                &mut entry as *mut Entry as *mut u8,
                core::mem::size_of::<Entry>(),
            )
        };

        entry_bytes.copy_from_slice(&self.flash[offset..offset + entry_bytes.len()]);
        Ok(entry)
    }

    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), &'static str> {
        if key.len() > MAX_KEY_LEN || value.len() > MAX_VALUE_LEN {
            return Err("Key or value too long");
        }

        // Find existing entry or empty slot
        let mut slot = None;
        for i in 0..MAX_ENTRIES {
            let entry = self.read_entry(i)?;
            if entry.valid == 0xFF
                && entry.key_len == key.len() as u8
                && &entry.key[..key.len()] == key
            {
                slot = Some(i);
                break;
            } else if entry.valid == 0x00 && slot.is_none() {
                slot = Some(i);
            }
        }

        let slot = slot.ok_or("Storage full")?;

        let mut entry = Entry::new();
        entry.key[..key.len()].copy_from_slice(key);
        entry.key_len = key.len() as u8;
        entry.value[..value.len()].copy_from_slice(value);
        entry.value_len = value.len() as u8;
        entry.valid = 0xFF;

        self.write_entry(slot, &entry)
    }

    fn get(&self, key: &[u8]) -> Option<&[u8]> {
        for i in 0..MAX_ENTRIES {
            if let Ok(entry) = self.read_entry(i) {
                if entry.valid == 0xFF
                    && entry.key_len == key.len() as u8
                    && &entry.key[..key.len()] == key
                {
                    // Return value from flash directly
                    let offset = i * core::mem::size_of::<Entry>()
                        + MAX_KEY_LEN
                        + 1; // skip key and key_len
                    return Some(&self.flash[offset..offset + entry.value_len as usize]);
                }
            }
        }
        None
    }

    fn delete(&mut self, key: &[u8]) -> Result<(), &'static str> {
        for i in 0..MAX_ENTRIES {
            let mut entry = self.read_entry(i)?;
            if entry.valid == 0xFF
                && entry.key_len == key.len() as u8
                && &entry.key[..key.len()] == key
            {
                entry.valid = 0x00;
                self.write_entry(i, &entry)?;
                return Ok(());
            }
        }
        Err("Key not found")
    }

    fn list(&self) -> [(Option<&[u8]>, Option<&[u8]>); MAX_ENTRIES] {
        let mut result = [(None, None); MAX_ENTRIES];
        
        for i in 0..MAX_ENTRIES {
            if let Ok(entry) = self.read_entry(i) {
                if entry.valid == 0xFF {
                    let offset = i * core::mem::size_of::<Entry>();
                    let key = &self.flash[offset..offset + entry.key_len as usize];
                    let value = &self.flash[offset + MAX_KEY_LEN + 1..offset + MAX_KEY_LEN + 1 + entry.value_len as usize];
                    result[i] = (Some(key), Some(value));
                }
            }
        }
        
        result
    }
}

// Global UART instance
struct Uart;
static mut UART: Uart = Uart;

impl Uart {
    fn putc(&self, c: u8) {
        unsafe {
            while (read_volatile(USART3_SR) & USART_SR_TXE) == 0 {}
            write_volatile(USART3_DR, c as u32);
        }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.putc(byte);
        }
        Ok(())
    }
}

// Critical section implementation
static IN_CRITICAL: AtomicBool = AtomicBool::new(false);

struct CriticalSection;

impl CriticalSection {
    fn new() -> Self {
        unsafe {
            core::arch::asm!("cpsid i");
        }
        IN_CRITICAL.store(true, Ordering::SeqCst);
        CriticalSection
    }
}

impl Drop for CriticalSection {
    fn drop(&mut self) {
        IN_CRITICAL.store(false, Ordering::SeqCst);
        unsafe {
            core::arch::asm!("cpsie i");
        }
    }
}

macro_rules! print {
    ($($arg:tt)*) => {
        {
            let _cs = CriticalSection::new();
            unsafe {
                use core::fmt::Write;
                let _ = write!(&mut UART, $($arg)*);
            }
        }
    };
}

macro_rules! println {
    () => { print!("\r\n") };
    ($($arg:tt)*) => {
        {
            let _cs = CriticalSection::new();
            unsafe {
                use core::fmt::Write;
                let _ = write!(&mut UART, $($arg)*);
                let _ = write!(&mut UART, "\r\n");
            }
        }
    };
}

fn print_bytes(data: &[u8]) {
    for &byte in data {
        unsafe {
            UART.putc(byte);
        }
    }
}

#[no_mangle]
pub extern "C" fn Reset_Handler() -> ! {
    // Enable GPIOD and USART3 clocks
    unsafe {
        let mut rcc_ahb1enr = read_volatile(RCC_AHB1ENR);
        rcc_ahb1enr |= 1 << 3; // GPIODEN
        write_volatile(RCC_AHB1ENR, rcc_ahb1enr);

        let mut rcc_apb1enr = read_volatile(RCC_APB1ENR);
        rcc_apb1enr |= 1 << 18; // USART3EN
        write_volatile(RCC_APB1ENR, rcc_apb1enr);
    }

    // Configure PD8 (TX) and PD9 (RX) as AF7
    unsafe {
        let mut moder = read_volatile(GPIOD_MODER);
        moder &= !(0b11 << (8 * 2)) | !(0b11 << (9 * 2));
        moder |= (0b10 << (8 * 2)) | (0b10 << (9 * 2));
        write_volatile(GPIOD_MODER, moder);

        let mut afrh = read_volatile(GPIOD_AFRH);
        afrh &= !(0xF << ((8 - 8) * 4)) | !(0xF << ((9 - 8) * 4));
        afrh |= (7 << ((8 - 8) * 4)) | (7 << ((9 - 8) * 4));
        write_volatile(GPIOD_AFRH, afrh);
    }

    // Configure USART3: 115200 baud, 8N1
    unsafe {
        write_volatile(USART3_BRR, 139); // 16MHz / 115200
        write_volatile(USART3_CR1, (1 << 13) | (1 << 3) | (1 << 2)); // UE, TE, RE
    }

    println!("\r\n=== Key-Value Storage Example ===\r\n");

    // Initialize storage
    let mut storage = Storage::new();
    
    println!("Formatting storage...");
    storage.format();
    println!("Format complete!\n");

    // Write some key-value pairs
    println!("Writing entries...");
    let _ = storage.set(b"ssid", b"MyWiFiNetwork");
    println!("  ssid = MyWiFiNetwork");
    
    let _ = storage.set(b"password", b"secret123");
    println!("  password = secret123");
    
    let _ = storage.set(b"port", b"8080");
    println!("  port = 8080");
    
    let _ = storage.set(b"timeout", b"30");
    println!("  timeout = 30\n");

    // Read values back
    println!("Reading entries...");
    if let Some(value) = storage.get(b"ssid") {
        print!("  ssid = ");
        print_bytes(value);
        println!("");
    }
    
    if let Some(value) = storage.get(b"password") {
        print!("  password = ");
        print_bytes(value);
        println!("");
    }
    
    if let Some(value) = storage.get(b"port") {
        print!("  port = ");
        print_bytes(value);
        println!("");
    }

    // List all entries
    println!("\nListing all entries:");
    for (key, value) in storage.list().iter() {
        if let (Some(k), Some(v)) = (key, value) {
            print!("  ");
            print_bytes(k);
            print!(" = ");
            print_bytes(v);
            println!("");
        }
    }

    // Update an entry
    println!("\nUpdating 'port' to 9000...");
    let _ = storage.set(b"port", b"9000");
    
    if let Some(value) = storage.get(b"port") {
        print!("  port = ");
        print_bytes(value);
        println!("");
    }

    // Delete an entry
    println!("\nDeleting 'timeout'...");
    let _ = storage.delete(b"timeout");
    
    println!("Remaining entries:");
    for (key, value) in storage.list().iter() {
        if let (Some(k), Some(v)) = (key, value) {
            print!("  ");
            print_bytes(k);
            print!(" = ");
            print_bytes(v);
            println!("");
        }
    }

    println!("\nDone!");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
