#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{AtomicU32, Ordering};

// ============================================================================
// SysTick Register Adresleri (ARM Cortex-M System Control Space)
// ============================================================================
const SYSTICK_BASE: u32 = 0xE000_E010;
const SYST_CSR: *mut u32 = (SYSTICK_BASE + 0x00) as *mut u32;  // Control and Status
const SYST_RVR: *mut u32 = (SYSTICK_BASE + 0x04) as *mut u32;  // Reload Value
const SYST_CVR: *mut u32 = (SYSTICK_BASE + 0x08) as *mut u32;  // Current Value

// SysTick Control/Status Register bit masks
const SYST_CSR_ENABLE: u32 = 1 << 0;     // Counter enable
const SYST_CSR_TICKINT: u32 = 1 << 1;    // Interrupt enable
const SYST_CSR_CLKSOURCE: u32 = 1 << 2;  // Clock source (1 = processor clock)

// ============================================================================
// RCC ve GPIO Register Adresleri
// ============================================================================
const RCC_BASE: u32 = 0x4002_3800;
const RCC_AHB1ENR: *mut u32 = (RCC_BASE + 0x30) as *mut u32;

const GPIOB_BASE: u32 = 0x4002_0400;
const GPIOB_MODER: *mut u32 = (GPIOB_BASE + 0x00) as *mut u32;
const GPIOB_ODR: *mut u32 = (GPIOB_BASE + 0x14) as *mut u32;

const RCC_AHB1ENR_GPIOBEN: u32 = 1 << 1;
const LED_PIN: u32 = 7;

// ============================================================================
// Global Milisaniye Sayacı (Atomic - interrupt-safe)
// ============================================================================
static MILLIS: AtomicU32 = AtomicU32::new(0);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// ============================================================================
// SysTick Interrupt Handler
// ============================================================================
#[no_mangle]
pub extern "C" fn SysTick_Handler() {
    // Her SysTick interrupt'ında milisaniye sayacını artır
    MILLIS.fetch_add(1, Ordering::Relaxed);
}

// ============================================================================
// Public API
// ============================================================================

/// Mevcut milisaniye değerini döndür
fn millis() -> u32 {
    MILLIS.load(Ordering::Relaxed)
}

/// Belirtilen milisaniye kadar bekle (hassas delay)
fn delay_ms(ms: u32) {
    let start = millis();
    while millis().wrapping_sub(start) < ms {
        // Busy-wait, ama çok daha az CPU kullanır
        // (interrupt her 1ms'de tetiklenir)
    }
}

/// SysTick timer'ı başlat (1ms tick)
/// Varsayım: System clock = 16 MHz (HSI default clock)
unsafe fn systick_init() {
    // SysTick'i durdur
    write_volatile(SYST_CSR, 0);
    
    // 1ms için reload value hesapla
    // 16 MHz / 1000 = 16000 ticks per millisecond
    // Reload value = ticks - 1
    let reload_value = 16_000 - 1;
    write_volatile(SYST_RVR, reload_value);
    
    // Current value'yu sıfırla
    write_volatile(SYST_CVR, 0);
    
    // SysTick'i başlat:
    // - ENABLE: Counter'ı aktif et
    // - TICKINT: Interrupt'ı etkinleştir
    // - CLKSOURCE: Processor clock kullan
    write_volatile(SYST_CSR, SYST_CSR_ENABLE | SYST_CSR_TICKINT | SYST_CSR_CLKSOURCE);
}

/// GPIO'yu başlat (LED için)
unsafe fn gpio_init() {
    // GPIOB clock'unu etkinleştir
    let rcc_ahb1enr = read_volatile(RCC_AHB1ENR);
    write_volatile(RCC_AHB1ENR, rcc_ahb1enr | RCC_AHB1ENR_GPIOBEN);

    // PB7'yi output mode'a ayarla
    let moder = read_volatile(GPIOB_MODER);
    let moder = moder & !(0b11 << (LED_PIN * 2));
    let moder = moder | (0b01 << (LED_PIN * 2));
    write_volatile(GPIOB_MODER, moder);
}

/// LED'i aç
unsafe fn led_on() {
    let odr = read_volatile(GPIOB_ODR);
    write_volatile(GPIOB_ODR, odr | (1 << LED_PIN));
}

/// LED'i kapat
unsafe fn led_off() {
    let odr = read_volatile(GPIOB_ODR);
    write_volatile(GPIOB_ODR, odr & !(1 << LED_PIN));
}

/// LED'i toggle et
unsafe fn led_toggle() {
    let odr = read_volatile(GPIOB_ODR);
    write_volatile(GPIOB_ODR, odr ^ (1 << LED_PIN));
}

// ============================================================================
// Main Entry Point
// ============================================================================
#[no_mangle]
pub extern "C" fn Reset() -> ! {
    unsafe {
        // Peripheral'leri başlat
        gpio_init();
        systick_init();
        
        // Ana loop: LED'i 1 saniye aralıklarla yak/söndür
        loop {
            led_on();
            delay_ms(3000);  // 1 saniye bekle
            
            led_off();
            delay_ms(1000);  // 1 saniye bekle
        }
    }
}
