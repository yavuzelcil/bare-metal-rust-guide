#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};

// RCC (Reset and Clock Control) register base address
const RCC_BASE: u32 = 0x4002_3800;
const RCC_AHB1ENR: *mut u32 = (RCC_BASE + 0x30) as *mut u32;

// GPIOB base address (user LED genellikle PB0, PB7, veya PB14'te olur)
const GPIOB_BASE: u32 = 0x4002_0400;
const GPIOB_MODER: *mut u32 = (GPIOB_BASE + 0x00) as *mut u32;
const GPIOB_ODR: *mut u32 = (GPIOB_BASE + 0x14) as *mut u32;

// GPIOB clock enable bit (bit 1 in AHB1ENR)
const RCC_AHB1ENR_GPIOBEN: u32 = 1 << 1;

// LED pin (PB7 kullanıyoruz - birçok Nucleo/Discovery board'da user LED bu pinde)
const LED_PIN: u32 = 7;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Basit delay fonksiyonu (busy-wait loop)
/// NOT: Bu çok kaba bir delay, gerçek projede SysTick kullanılmalı
fn delay(count: u32) {
    for _ in 0..count {
        // Compiler'ın optimize edip çıkarmaması için volatile read
        unsafe {
            read_volatile(&count);
        }
    }
}

/// GPIO pinini output mode'a ayarla
unsafe fn gpio_init() {
    // 1. GPIOB clock'unu etkinleştir
    let rcc_ahb1enr = read_volatile(RCC_AHB1ENR);
    write_volatile(RCC_AHB1ENR, rcc_ahb1enr | RCC_AHB1ENR_GPIOBEN);

    // 2. PB7'yi output mode'a ayarla (MODER register'da 2 bit per pin)
    // MODER[15:14] = 01 (output mode) for pin 7
    let moder = read_volatile(GPIOB_MODER);
    // Önce pin 7'nin bitlerini temizle (mask: ~(0b11 << (LED_PIN * 2)))
    let moder = moder & !(0b11 << (LED_PIN * 2));
    // Output mode ayarla (01)
    let moder = moder | (0b01 << (LED_PIN * 2));
    write_volatile(GPIOB_MODER, moder);
}

/// LED'i toggle et (aç/kapat)
unsafe fn led_toggle() {
    let odr = read_volatile(GPIOB_ODR);
    write_volatile(GPIOB_ODR, odr ^ (1 << LED_PIN));
}

#[no_mangle]
pub extern "C" fn Reset() -> ! {
    unsafe {
        // GPIO'yu başlat
        gpio_init();

        // Sonsuz blink döngüsü
        loop {
            led_toggle();
            delay(500_000); // ~500ms (yaklaşık, clock hızına bağlı)
        }
    }
}
