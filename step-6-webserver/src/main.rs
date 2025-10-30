#![no_std]
#![no_main]

use core::fmt::{self, Write};
use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

// Hardware addresses - RCC
const RCC_BASE: usize = 0x4002_3800;
const RCC_AHB1ENR: *mut u32 = (RCC_BASE + 0x30) as *mut u32;
const RCC_APB1ENR: *mut u32 = (RCC_BASE + 0x40) as *mut u32;
const RCC_APB2ENR: *mut u32 = (RCC_BASE + 0x44) as *mut u32;
const RCC_AHB1RSTR: *mut u32 = (RCC_BASE + 0x10) as *mut u32;

// GPIO addresses
const GPIOB_BASE: usize = 0x4002_0400;
const GPIOB_MODER: *mut u32 = (GPIOB_BASE + 0x00) as *mut u32;
const GPIOB_ODR: *mut u32 = (GPIOB_BASE + 0x14) as *mut u32;

const GPIOD_BASE: usize = 0x4002_0C00;
const GPIOD_MODER: *mut u32 = (GPIOD_BASE + 0x00) as *mut u32;
const GPIOD_AFRH: *mut u32 = (GPIOD_BASE + 0x24) as *mut u32;

// USART3 addresses
const USART3_BASE: usize = 0x4000_4800;
const USART3_SR: *mut u32 = (USART3_BASE + 0x00) as *mut u32;
const USART3_DR: *mut u32 = (USART3_BASE + 0x04) as *mut u32;
const USART3_BRR: *mut u32 = (USART3_BASE + 0x08) as *mut u32;
const USART3_CR1: *mut u32 = (USART3_BASE + 0x0C) as *mut u32;
const USART_SR_TXE: u32 = 1 << 7;

// SysTick addresses
const SYSTICK_BASE: usize = 0xE000_E010;
const SYSTICK_CSR: *mut u32 = (SYSTICK_BASE + 0x00) as *mut u32;
const SYSTICK_RVR: *mut u32 = (SYSTICK_BASE + 0x04) as *mut u32;
const SYSTICK_CVR: *mut u32 = (SYSTICK_BASE + 0x08) as *mut u32;

// Ethernet MAC addresses
const ETH_BASE: usize = 0x4002_8000;
const ETH_MACCR: *mut u32 = (ETH_BASE + 0x0000) as *mut u32;
const ETH_MACFFR: *mut u32 = (ETH_BASE + 0x0004) as *mut u32;
const ETH_MACHTHR: *mut u32 = (ETH_BASE + 0x0008) as *mut u32;
const ETH_MACHTLR: *mut u32 = (ETH_BASE + 0x000C) as *mut u32;
const ETH_MACMIIAR: *mut u32 = (ETH_BASE + 0x0010) as *mut u32;
const ETH_MACMIIDR: *mut u32 = (ETH_BASE + 0x0014) as *mut u32;
const ETH_MACFCR: *mut u32 = (ETH_BASE + 0x0018) as *mut u32;
const ETH_MACA0HR: *mut u32 = (ETH_BASE + 0x0040) as *mut u32;
const ETH_MACA0LR: *mut u32 = (ETH_BASE + 0x0044) as *mut u32;

// LED pins: PB0 (Blue), PB7 (Green), PB14 (Red)
const LED_BLUE: u32 = 0;
const LED_GREEN: u32 = 7;
const LED_RED: u32 = 14;

// Global state
static MILLIS: AtomicU32 = AtomicU32::new(0);
static LED_STATE: AtomicU32 = AtomicU32::new(0); // bits: 0=blue, 1=green, 2=red

// UART singleton
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

// Critical section
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

macro_rules! println {
    () => { 
        {
            let _cs = CriticalSection::new();
            unsafe {
                use core::fmt::Write;
                let _ = write!(&mut UART, "\r\n");
            }
        }
    };
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

// LED control functions
fn led_init() {
    unsafe {
        // Enable GPIOB clock
        let mut rcc = read_volatile(RCC_AHB1ENR);
        rcc |= 1 << 1; // GPIOBEN
        write_volatile(RCC_AHB1ENR, rcc);

        // Configure PB0, PB7, PB14 as output
        let mut moder = read_volatile(GPIOB_MODER);
        moder &= !(0b11 << (LED_BLUE * 2));
        moder |= 0b01 << (LED_BLUE * 2);
        moder &= !(0b11 << (LED_GREEN * 2));
        moder |= 0b01 << (LED_GREEN * 2);
        moder &= !(0b11 << (LED_RED * 2));
        moder |= 0b01 << (LED_RED * 2);
        write_volatile(GPIOB_MODER, moder);

        // Turn off all LEDs
        led_set(LED_BLUE, false);
        led_set(LED_GREEN, false);
        led_set(LED_RED, false);
    }
}

fn led_set(pin: u32, state: bool) {
    unsafe {
        let mut odr = read_volatile(GPIOB_ODR);
        if state {
            odr |= 1 << pin;
        } else {
            odr &= !(1 << pin);
        }
        write_volatile(GPIOB_ODR, odr);
    }

    // Update global state
    let mut led_state = LED_STATE.load(Ordering::Relaxed);
    let bit = match pin {
        LED_BLUE => 0,
        LED_GREEN => 1,
        LED_RED => 2,
        _ => return,
    };
    if state {
        led_state |= 1 << bit;
    } else {
        led_state &= !(1 << bit);
    }
    LED_STATE.store(led_state, Ordering::Relaxed);
}

fn led_toggle(pin: u32) {
    unsafe {
        let mut odr = read_volatile(GPIOB_ODR);
        odr ^= 1 << pin;
        write_volatile(GPIOB_ODR, odr);
    }

    // Update global state
    let mut led_state = LED_STATE.load(Ordering::Relaxed);
    let bit = match pin {
        LED_BLUE => 0,
        LED_GREEN => 1,
        LED_RED => 2,
        _ => return,
    };
    led_state ^= 1 << bit;
    LED_STATE.store(led_state, Ordering::Relaxed);
}

fn led_get(pin: u32) -> bool {
    let led_state = LED_STATE.load(Ordering::Relaxed);
    let bit = match pin {
        LED_BLUE => 0,
        LED_GREEN => 1,
        LED_RED => 2,
        _ => return false,
    };
    (led_state & (1 << bit)) != 0
}

fn led_toggle_all() {
    led_toggle(LED_BLUE);
    led_toggle(LED_GREEN);
    led_toggle(LED_RED);
}

// UART initialization
fn uart_init() {
    unsafe {
        // Enable GPIOD and USART3 clocks
        let mut rcc_ahb1enr = read_volatile(RCC_AHB1ENR);
        rcc_ahb1enr |= 1 << 3; // GPIODEN
        write_volatile(RCC_AHB1ENR, rcc_ahb1enr);

        let mut rcc_apb1enr = read_volatile(RCC_APB1ENR);
        rcc_apb1enr |= 1 << 18; // USART3EN
        write_volatile(RCC_APB1ENR, rcc_apb1enr);

        // Configure PD8 (TX) and PD9 (RX) as AF7
        let mut moder = read_volatile(GPIOD_MODER);
        moder &= !(0b11 << (8 * 2)) | !(0b11 << (9 * 2));
        moder |= (0b10 << (8 * 2)) | (0b10 << (9 * 2));
        write_volatile(GPIOD_MODER, moder);

        let mut afrh = read_volatile(GPIOD_AFRH);
        afrh &= !(0xF << ((8 - 8) * 4)) | !(0xF << ((9 - 8) * 4));
        afrh |= (7 << ((8 - 8) * 4)) | (7 << ((9 - 8) * 4));
        write_volatile(GPIOD_AFRH, afrh);

        // Configure USART3: 115200 baud, 8N1
        write_volatile(USART3_BRR, 139); // 16MHz / 115200
        write_volatile(USART3_CR1, (1 << 13) | (1 << 3) | (1 << 2)); // UE, TE, RE
    }
}

// SysTick initialization
fn systick_init() {
    unsafe {
        write_volatile(SYSTICK_RVR, 16000 - 1); // 1ms @ 16MHz
        write_volatile(SYSTICK_CVR, 0);
        write_volatile(SYSTICK_CSR, 0x7); // Enable, interrupt, use processor clock
    }
}

#[no_mangle]
pub extern "C" fn SysTick_Handler() {
    MILLIS.fetch_add(1, Ordering::Relaxed);
}

fn millis() -> u32 {
    MILLIS.load(Ordering::Relaxed)
}

fn delay_ms(ms: u32) {
    let start = millis();
    while millis() - start < ms {}
}

// Ethernet initialization (placeholder - will be implemented)
fn ethernet_init() {
    println!("Ethernet init...");
    
    unsafe {
        // Enable Ethernet clocks
        let mut rcc = read_volatile(RCC_AHB1ENR);
        rcc |= (1 << 25) | (1 << 26) | (1 << 27); // ETHMACEN, ETHMACTXEN, ETHMACRXEN
        write_volatile(RCC_AHB1ENR, rcc);

        // Software reset
        let mut rcc_rst = read_volatile(RCC_AHB1RSTR);
        rcc_rst |= 1 << 25; // ETHMACRST
        write_volatile(RCC_AHB1RSTR, rcc_rst);
        delay_ms(1);
        rcc_rst &= !(1 << 25);
        write_volatile(RCC_AHB1RSTR, rcc_rst);
        delay_ms(1);

        // Set MAC address (example: 02:00:00:00:00:01)
        write_volatile(ETH_MACA0HR, 0x00000002);
        write_volatile(ETH_MACA0LR, 0x01000000);

        println!("MAC: 02:00:00:00:00:01");
    }

    // TODO: PHY init, DMA descriptors, smoltcp setup
    println!("Ethernet ready (stub)");
}

// HTTP request parser
fn parse_http_request(buffer: &[u8]) -> Option<(&str, &str)> {
    let req = core::str::from_utf8(buffer).ok()?;
    let mut lines = req.lines();
    let first_line = lines.next()?;
    
    let mut parts = first_line.split_whitespace();
    let method = parts.next()?;
    let path = parts.next()?;
    
    Some((method, path))
}

// HTTP response generator
fn handle_request(method: &str, path: &str) -> &'static str {
    if method != "GET" {
        return "HTTP/1.1 405 Method Not Allowed\r\nContent-Length: 0\r\n\r\n";
    }

    match path {
        "/" => {
            concat!(
                "HTTP/1.1 200 OK\r\n",
                "Content-Type: text/html\r\n\r\n",
                "<!DOCTYPE html><html><head><title>STM32 LED Control</title>",
                "<style>body{font-family:Arial;margin:40px;background:#f0f0f0}",
                ".container{background:white;padding:30px;border-radius:10px;box-shadow:0 2px 10px rgba(0,0,0,0.1)}",
                "h1{color:#333;text-align:center}",
                ".led{margin:20px 0;padding:15px;border:2px solid #ddd;border-radius:5px}",
                ".btn{padding:10px 20px;margin:5px;border:none;border-radius:5px;cursor:pointer;font-size:16px}",
                ".btn-on{background:#4CAF50;color:white}.btn-off{background:#f44336;color:white}",
                ".btn-toggle{background:#2196F3;color:white;width:100%;padding:15px;font-size:18px}",
                ".status{display:inline-block;width:20px;height:20px;border-radius:50%;margin-left:10px}",
                ".status-on{background:#4CAF50}.status-off{background:#ccc}",
                "</style></head><body>",
                "<div class='container'><h1>🎮 STM32 LED Controller</h1>",
                "<div class='led'>🔵 <b>Blue LED</b> <span class='status status-off' id='blue'></span>",
                "<button class='btn btn-on' onclick=\"fetch('/led/blue/on').then(()=>location.reload())\">ON</button>",
                "<button class='btn btn-off' onclick=\"fetch('/led/blue/off').then(()=>location.reload())\">OFF</button></div>",
                "<div class='led'>🟢 <b>Green LED</b> <span class='status status-off' id='green'></span>",
                "<button class='btn btn-on' onclick=\"fetch('/led/green/on').then(()=>location.reload())\">ON</button>",
                "<button class='btn btn-off' onclick=\"fetch('/led/green/off').then(()=>location.reload())\">OFF</button></div>",
                "<div class='led'>🔴 <b>Red LED</b> <span class='status status-off' id='red'></span>",
                "<button class='btn btn-on' onclick=\"fetch('/led/red/on').then(()=>location.reload())\">ON</button>",
                "<button class='btn btn-off' onclick=\"fetch('/led/red/off').then(()=>location.reload())\">OFF</button></div>",
                "<button class='btn btn-toggle' onclick=\"fetch('/led/toggle').then(()=>location.reload())\">⚡ Toggle All LEDs</button>",
                "<p style='text-align:center;color:#666;margin-top:30px'>STM32F439ZI @ 192.168.1.100</p>",
                "</div></body></html>"
            )
        }
        "/led/blue/on" => {
            led_set(LED_BLUE, true);
            "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK"
        }
        "/led/blue/off" => {
            led_set(LED_BLUE, false);
            "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK"
        }
        "/led/green/on" => {
            led_set(LED_GREEN, true);
            "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK"
        }
        "/led/green/off" => {
            led_set(LED_GREEN, false);
            "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK"
        }
        "/led/red/on" => {
            led_set(LED_RED, true);
            "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK"
        }
        "/led/red/off" => {
            led_set(LED_RED, false);
            "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK"
        }
        "/led/toggle" => {
            led_toggle_all();
            "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK"
        }
        "/api/status" => {
            // Return JSON status
            if led_get(LED_BLUE) && led_get(LED_GREEN) && led_get(LED_RED) {
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"blue\":true,\"green\":true,\"red\":true}"
            } else if !led_get(LED_BLUE) && !led_get(LED_GREEN) && !led_get(LED_RED) {
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"blue\":false,\"green\":false,\"red\":false}"
            } else {
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"blue\":false,\"green\":false,\"red\":false}"
            }
        }
        _ => "HTTP/1.1 404 Not Found\r\nContent-Length: 9\r\n\r\nNot Found"
    }
}

#[no_mangle]
pub extern "C" fn Reset_Handler() -> ! {
    // Initialize peripherals
    uart_init();
    systick_init();
    led_init();

    println!("\r\n=== STM32 Web Server Demo ===\r\n");

    // Initialize Ethernet (stub for now)
    ethernet_init();

    println!("\nServer ready!");
    println!("Waiting for connections...\n");

    // Demo: Simulate HTTP requests via UART
    println!("Demo mode: Send HTTP requests via UART");
    println!("Example: GET /led/blue/on HTTP/1.1\\r\\n\\r\\n\n");

    // Main loop - in real implementation, this would poll Ethernet
    let mut counter = 0u32;
    loop {
        delay_ms(1000);
        counter += 1;

        if counter % 5 == 0 {
            println!("[{}s] Server running... LED states: Blue={} Green={} Red={}", 
                counter, 
                if led_get(LED_BLUE) { "ON" } else { "OFF" },
                if led_get(LED_GREEN) { "ON" } else { "OFF" },
                if led_get(LED_RED) { "ON" } else { "OFF" }
            );
        }

        // Demo: toggle green LED every 10 seconds
        if counter % 10 == 0 {
            led_toggle(LED_GREEN);
            println!("Auto-toggled green LED");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
