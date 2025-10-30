#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};
use core::fmt::{self, Write};
use core::cell::RefCell;
use core::ops::DerefMut;

// ============================================================================
// Critical Section (basit interrupt disable/enable)
// ============================================================================
struct CriticalSection;

impl CriticalSection {
    fn new() -> Self {
        unsafe {
            // PRIMASK register'ı set et (interrupt'ları disable et)
            core::arch::asm!("cpsid i");
        }
        CriticalSection
    }
}

impl Drop for CriticalSection {
    fn drop(&mut self) {
        unsafe {
            // PRIMASK register'ı clear et (interrupt'ları enable et)
            core::arch::asm!("cpsie i");
        }
    }
}

// ============================================================================
// USART3 Register Adresleri
// ============================================================================
const USART3_BASE: u32 = 0x4000_4800;
const USART3_SR: *mut u32 = (USART3_BASE + 0x00) as *mut u32;
const USART3_DR: *mut u32 = (USART3_BASE + 0x04) as *mut u32;
const USART3_BRR: *mut u32 = (USART3_BASE + 0x08) as *mut u32;
const USART3_CR1: *mut u32 = (USART3_BASE + 0x0C) as *mut u32;

const USART_SR_TXE: u32 = 1 << 7;
const USART_CR1_UE: u32 = 1 << 13;
const USART_CR1_TE: u32 = 1 << 3;
const USART_CR1_RE: u32 = 1 << 2;

// ============================================================================
// RCC Register Adresleri
// ============================================================================
const RCC_BASE: u32 = 0x4002_3800;
const RCC_AHB1ENR: *mut u32 = (RCC_BASE + 0x30) as *mut u32;
const RCC_APB1ENR: *mut u32 = (RCC_BASE + 0x40) as *mut u32;

const RCC_AHB1ENR_GPIOBEN: u32 = 1 << 1;
const RCC_AHB1ENR_GPIODEN: u32 = 1 << 3;
const RCC_APB1ENR_USART3EN: u32 = 1 << 18;

// ============================================================================
// GPIO Register Adresleri
// ============================================================================
const GPIOD_BASE: u32 = 0x4002_0C00;
const GPIOD_MODER: *mut u32 = (GPIOD_BASE + 0x00) as *mut u32;
const GPIOD_AFRH: *mut u32 = (GPIOD_BASE + 0x24) as *mut u32;

const GPIOB_BASE: u32 = 0x4002_0400;
const GPIOB_MODER: *mut u32 = (GPIOB_BASE + 0x00) as *mut u32;
const GPIOB_ODR: *mut u32 = (GPIOB_BASE + 0x14) as *mut u32;
const LED_PIN: u32 = 7;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// ============================================================================
// UART Wrapper Struct
// ============================================================================
struct Uart;

impl Uart {
    /// USART3'ü başlat (115200 baud, 8N1)
    unsafe fn init() {
        // Clock enable
        let rcc_ahb1enr = read_volatile(RCC_AHB1ENR);
        write_volatile(RCC_AHB1ENR, rcc_ahb1enr | RCC_AHB1ENR_GPIODEN);
        
        let rcc_apb1enr = read_volatile(RCC_APB1ENR);
        write_volatile(RCC_APB1ENR, rcc_apb1enr | RCC_APB1ENR_USART3EN);
        
        // GPIO AF mode (PD8, PD9)
        let moder = read_volatile(GPIOD_MODER);
        let moder = moder & !(0b1111 << (8 * 2));
        let moder = moder | (0b1010 << (8 * 2));
        write_volatile(GPIOD_MODER, moder);
        
        // AF7 (USART3)
        let afrh = read_volatile(GPIOD_AFRH);
        let afrh = afrh & !(0xFF << 0);
        let afrh = afrh | (0x77 << 0);
        write_volatile(GPIOD_AFRH, afrh);
        
        // Baud rate: 115200 @ 16 MHz
        write_volatile(USART3_BRR, 139);
        
        // Enable USART
        write_volatile(USART3_CR1, USART_CR1_UE | USART_CR1_TE | USART_CR1_RE);
    }
    
    /// Tek karakter gönder
    fn putc(&mut self, c: u8) {
        unsafe {
            while (read_volatile(USART3_SR) & USART_SR_TXE) == 0 {}
            write_volatile(USART3_DR, c as u32);
        }
    }
}

// ============================================================================
// fmt::Write Trait Implementation
// ============================================================================
impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.putc(byte);
        }
        Ok(())
    }
}

// ============================================================================
// Global UART Instance (RefCell wrapped)
// ============================================================================
static mut UART: Option<RefCell<Uart>> = None;

/// Global UART instance'ı başlat
unsafe fn init_global_uart() {
    Uart::init();
    UART = Some(RefCell::new(Uart));
}

/// Global UART instance'a erişim
fn with_uart<F, R>(f: F) -> R
where
    F: FnOnce(&mut Uart) -> R,
{
    let _cs = CriticalSection::new();  // Interrupt-safe
    unsafe {
        let uart_ref = UART.as_ref().unwrap();
        let mut uart = uart_ref.borrow_mut();
        f(uart.deref_mut())
    }
}

// ============================================================================
// print! ve println! Macro'ları
// ============================================================================
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        with_uart(|uart| {
            use core::fmt::Write;
            let _ = write!(uart, $($arg)*);
        })
    };
}

#[macro_export]
macro_rules! println {
    () => { print!("\r\n") };
    ($($arg:tt)*) => {
        with_uart(|uart| {
            use core::fmt::Write;
            let _ = writeln!(uart, $($arg)*);
        })
    };
}

// ============================================================================
// LED Fonksiyonları
// ============================================================================
unsafe fn led_init() {
    let rcc_ahb1enr = read_volatile(RCC_AHB1ENR);
    write_volatile(RCC_AHB1ENR, rcc_ahb1enr | RCC_AHB1ENR_GPIOBEN);
    
    let moder = read_volatile(GPIOB_MODER);
    let moder = moder & !(0b11 << (LED_PIN * 2));
    let moder = moder | (0b01 << (LED_PIN * 2));
    write_volatile(GPIOB_MODER, moder);
}

unsafe fn led_toggle() {
    let odr = read_volatile(GPIOB_ODR);
    write_volatile(GPIOB_ODR, odr ^ (1 << LED_PIN));
}

fn delay(count: u32) {
    for _ in 0..count {
        unsafe {
            read_volatile(&count);
        }
    }
}

// ============================================================================
// Main Entry Point
// ============================================================================
#[no_mangle]
pub extern "C" fn Reset() -> ! {
    unsafe {
        init_global_uart();
        led_init();
        
        // Başlangıç mesajı (formatlanmış!)
        println!();
        println!("====================================");
        println!("  Bare-Metal Rust - Step 4");
        println!("  Printf Implementation");
        println!("====================================");
        println!("Rust version: {}", env!("CARGO_PKG_VERSION"));
        println!();
        
        let mut counter: u32 = 0;
        let mut total_toggles: u32 = 0;
        
        loop {
            led_toggle();
            total_toggles += 1;
            
            // Formatlanmış çıktı - HEX, decimal, padding
            println!("Toggle #{:03} | Total: {} | Hex: 0x{:04X}", 
                     counter, total_toggles, counter);
            
            // Matematiksel işlemler
            if counter % 5 == 0 {
                println!("  -> Counter is divisible by 5!");
            }
            
            if counter == 10 {
                println!("  => Reached 10 toggles!");
            }
            
            counter += 1;
            
            // Her 20 toggle'da özet
            if counter % 20 == 0 {
                println!();
                println!("=== Summary after {} toggles ===", counter);
                println!("  LED state changed {} times", total_toggles);
                println!();
            }
            
            delay(2_000_000);
        }
    }
}
