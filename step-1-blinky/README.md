# Step 1: Blinky (LED Yakıp Söndürme)

Bu adımda GPIO kullanarak bir LED'i yakıp söndürmeyi öğreniyoruz.

## ✨ Özellikler

- ✅ GPIO register'larına **doğrudan erişim** (HAL yok)
- ✅ RCC clock enable (GPIOB peripheral'i aktif etme)
- ✅ Pin konfigürasyonu (output mode)
- ✅ Basit delay döngüsü ile LED toggle
- ✅ Volatile pointer kullanımı

## 🎯 Öğrenilen Kavramlar

1. **Memory-Mapped I/O:** Hardware register'lara pointer ile erişim
2. **RCC (Reset and Clock Control):** Peripheral clock'larını etkinleştirme
3. **GPIO MODER:** Pin mode ayarları (input/output/alternate/analog)
4. **GPIO ODR:** Output data register (pin açma/kapama)
5. **Volatile:** Compiler optimizasyonlarını engelleme

## 📋 Hardware Detayları

### Kullanılan Pin
- **PB7** (GPIOB Pin 7) - Birçok STM32 board'da user LED bu pinde

### Register Adresleri
```rust
RCC_BASE:     0x4002_3800
RCC_AHB1ENR:  RCC_BASE + 0x30  // Peripheral clock enable
GPIOB_BASE:   0x4002_0400
GPIOB_MODER:  GPIOB_BASE + 0x00 // Mode register
GPIOB_ODR:    GPIOB_BASE + 0x14 // Output data register
```

### Pin Mode Ayarı
MODER register'da her pin için 2 bit:
- `00` = Input mode
- `01` = Output mode ✅ (bizim kullandığımız)
- `10` = Alternate function
- `11` = Analog mode

## 🚀 Derleme ve Çalıştırma

### Build

```bash
cargo build --release
```

veya build script ile:
```bash
chmod +x build.sh
./build.sh
```

### Flash

```bash
# cargo run ile (probe-rs kullanır)
cargo run --release

# veya doğrudan probe-rs
probe-rs run --chip STM32F439ZI target/thumbv7em-none-eabihf/release/bare-metal-rust-step1-blinky
```

## 🔍 Kod Açıklaması

### 1. GPIO Initialization
```rust
unsafe fn gpio_init() {
    // GPIOB clock'unu aç
    write_volatile(RCC_AHB1ENR, read_volatile(RCC_AHB1ENR) | RCC_AHB1ENR_GPIOBEN);
    
    // PB7'yi output mode'a ayarla
    let moder = read_volatile(GPIOB_MODER);
    let moder = moder & !(0b11 << (7 * 2));  // Önce temizle
    let moder = moder | (0b01 << (7 * 2));   // Output mode
    write_volatile(GPIOB_MODER, moder);
}
```

### 2. LED Toggle
```rust
unsafe fn led_toggle() {
    let odr = read_volatile(GPIOB_ODR);
    write_volatile(GPIOB_ODR, odr ^ (1 << 7)); // XOR ile toggle
}
```

### 3. Delay (Busy-Wait)
```rust
fn delay(count: u32) {
    for _ in 0..count {
        unsafe { read_volatile(&count); }  // Compiler optimize etmesin
    }
}
```

## 📝 Notlar

### Farklı LED Pinleri
Kartınızda LED farklı bir pindeyse, şu satırları değiştirin:
```rust
const LED_PIN: u32 = 7;  // PB7 yerine istediğiniz pin numarası
// ve gerekirse GPIOB yerine GPIOA, GPIOC vb. kullanın
```

### GPIOA Kullanımı
GPIOA için:
```rust
const GPIOA_BASE: u32 = 0x4002_0000;
const RCC_AHB1ENR_GPIOAEN: u32 = 1 << 0;
```

### Delay Ayarı
Delay süresi MCU clock hızına bağlıdır. Daha hızlı/yavaş yanıp sönme için:
```rust
delay(500_000);  // Değeri artır/azalt
```

## 🎓 Sıradaki Adım

**step-2-systick** - Doğru zamanlama için SysTick timer kullanımı
- Hassas delay fonksiyonları
- Interrupt tabanlı zamanlama
- `delay(500_000)` yerine gerçek milisaniye delay

## 📚 Referanslar

- [STM32F439 Reference Manual (RM0090)](https://www.st.com/resource/en/reference_manual/dm00031020.pdf)
  - Chapter 8: RCC (Reset and Clock Control)
  - Chapter 11: GPIO (General Purpose I/O)
- GPIO MODER register: Section 11.4.1
- GPIO ODR register: Section 11.4.6

## ⚠️ Troubleshooting

**LED yanmıyor mu?**
1. Doğru pin numarasını kullandığınızdan emin olun
2. Kartınızın şemasına bakın (LED hangi GPIO pininde?)
3. Bazı kartlarda LED aktif-low olabilir (logic tersine çevirmeniz gerekebilir)
4. `probe-rs run` ile flash edip, seri çıktı var mı kontrol edin

**Derleme hatası?**
```bash
# Target yüklü mü kontrol edin
rustup target add thumbv7em-none-eabihf

# Clean build deneyin
cargo clean && cargo build --release
```
