#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};

// ============================================================================
// USART3 Register Adresleri (APB1 bus'ta)
// ============================================================================
const USART3_BASE: u32 = 0x4000_4800;
const USART3_SR: *mut u32 = (USART3_BASE + 0x00) as *mut u32;   // Status register
const USART3_DR: *mut u32 = (USART3_BASE + 0x04) as *mut u32;   // Data register
const USART3_BRR: *mut u32 = (USART3_BASE + 0x08) as *mut u32;  // Baud rate register
const USART3_CR1: *mut u32 = (USART3_BASE + 0x0C) as *mut u32;  // Control register 1

// USART Status Register (SR) bit masks
const USART_SR_TXE: u32 = 1 << 7;   // Transmit data register empty
const USART_SR_TC: u32 = 1 << 6;    // Transmission complete
const USART_SR_RXNE: u32 = 1 << 5;  // Read data register not empty

// USART Control Register 1 (CR1) bit masks
const USART_CR1_UE: u32 = 1 << 13;   // USART enable
const USART_CR1_TE: u32 = 1 << 3;    // Transmitter enable
const USART_CR1_RE: u32 = 1 << 2;    // Receiver enable

// ============================================================================
// RCC (Clock Control) Register Adresleri
// ============================================================================
const RCC_BASE: u32 = 0x4002_3800;
const RCC_AHB1ENR: *mut u32 = (RCC_BASE + 0x30) as *mut u32;  // AHB1 peripheral clock enable
const RCC_APB1ENR: *mut u32 = (RCC_BASE + 0x40) as *mut u32;  // APB1 peripheral clock enable

const RCC_AHB1ENR_GPIOBEN: u32 = 1 << 1;  // GPIOB clock enable
const RCC_AHB1ENR_GPIODEN: u32 = 1 << 3;  // GPIOD clock enable
const RCC_APB1ENR_USART3EN: u32 = 1 << 18; // USART3 clock enable

// ============================================================================
// GPIO Register Adresleri (USART3: PD8=TX, PD9=RX)
// ============================================================================
const GPIOD_BASE: u32 = 0x4002_0C00;
const GPIOD_MODER: *mut u32 = (GPIOD_BASE + 0x00) as *mut u32;   // Mode register
const GPIOD_AFRL: *mut u32 = (GPIOD_BASE + 0x20) as *mut u32;    // Alternate function low (pins 0-7)
const GPIOD_AFRH: *mut u32 = (GPIOD_BASE + 0x24) as *mut u32;    // Alternate function high (pins 8-15)

// LED (GPIOB)
const GPIOB_BASE: u32 = 0x4002_0400;
const GPIOB_MODER: *mut u32 = (GPIOB_BASE + 0x00) as *mut u32;
const GPIOB_ODR: *mut u32 = (GPIOB_BASE + 0x14) as *mut u32;
const LED_PIN: u32 = 7;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// ============================================================================
// UART Fonksiyonları
// ============================================================================

/// USART3'ü başlat (115200 baud, 8N1)
/// PD8 = TX, PD9 = RX, AF7 (Alternate Function 7)
unsafe fn uart_init() {
    // 1. GPIOD ve USART3 clock'larını etkinleştir
    let rcc_ahb1enr = read_volatile(RCC_AHB1ENR);
    write_volatile(RCC_AHB1ENR, rcc_ahb1enr | RCC_AHB1ENR_GPIODEN);
    
    let rcc_apb1enr = read_volatile(RCC_APB1ENR);
    write_volatile(RCC_APB1ENR, rcc_apb1enr | RCC_APB1ENR_USART3EN);
    
    // 2. PD8 ve PD9'u alternate function mode'a ayarla (mode = 0b10)
    let moder = read_volatile(GPIOD_MODER);
    let moder = moder & !(0b1111 << (8 * 2));  // PD8 ve PD9'u temizle
    let moder = moder | (0b1010 << (8 * 2));   // PD8=AF, PD9=AF
    write_volatile(GPIOD_MODER, moder);
    
    // 3. PD8 ve PD9 için AF7 (USART3) seç
    let afrh = read_volatile(GPIOD_AFRH);
    let afrh = afrh & !(0xFF << 0);      // PD8(bit 0-3) ve PD9(bit 4-7) temizle
    let afrh = afrh | (0x77 << 0);       // AF7 = 0b0111
    write_volatile(GPIOD_AFRH, afrh);
    
    // 4. Baud rate ayarla: 115200 @ 16 MHz APB1 clock
    // BRR = clock / baud = 16_000_000 / 115200 ≈ 139 (0x8B)
    write_volatile(USART3_BRR, 139);
    
    // 5. USART3'ü etkinleştir: UE | TE | RE
    write_volatile(USART3_CR1, USART_CR1_UE | USART_CR1_TE | USART_CR1_RE);
}

/// Tek bir karakter gönder
unsafe fn uart_putc(c: u8) {
    // TXE (Transmit Data Register Empty) flag'ini bekle
    while (read_volatile(USART3_SR) & USART_SR_TXE) == 0 {}
    
    // Karakteri data register'a yaz
    write_volatile(USART3_DR, c as u32);
}

/// String gönder
unsafe fn uart_puts(s: &str) {
    for byte in s.bytes() {
        uart_putc(byte);
    }
}

// ============================================================================
// LED Fonksiyonları (step-1'den)
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

/// Basit delay (step-1'den)
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
        // Peripheral'leri başlat
        uart_init();
        led_init();
        
        // Başlangıç mesajı
        uart_puts("\r\n");
        uart_puts("===============================\r\n");
        uart_puts("  Bare-Metal Rust - Step 3\r\n");
        uart_puts("  UART Serial Communication\r\n");
        uart_puts("===============================\r\n");
        uart_puts("Hello, World from STM32F439ZI!\r\n");
        uart_puts("\r\n");
        
        let mut counter: u32 = 0;
        
        // Ana loop: LED blink + serial log
        loop {
            led_toggle();
            
            // Counter mesajını yazdir
            //uart_puts("LED Toggle #");
            //print_u32(counter);
            //uart_puts("\r\n");
            
            //counter += 1;
            
            delay(2_000_000);  // ~2 saniye
        }
    }
}

// ============================================================================
// Helper: u32 sayıyı decimal olarak yazdır
// ============================================================================
unsafe fn print_u32(mut n: u32) {
    if n == 0 {
        uart_putc(b'0');
        return;
    }
    
    // Basamakları buffer'a yaz (ters sırada)
    let mut buffer = [0u8; 10];
    let mut i = 0;
    
    while n > 0 {
        buffer[i] = (b'0' + (n % 10) as u8);
        n /= 10;
        i += 1;
    }
    
    // Buffer'ı ters çevirerek yazdır
    while i > 0 {
        i -= 1;
        uart_putc(buffer[i]);
    }
}
