# Step 3: UART Serial Haberleşme

Bu adımda UART üzerinden seri port haberleşmesi yapıyoruz.

## ✨ Özellikler

- ✅ **USART3** yapılandırması (115200 baud, 8N1)
- ✅ GPIO **Alternate Function** (AF7) ayarları
- ✅ `uart_putc()` - Tek karakter gönderme
- ✅ `uart_puts()` - String gönderme
- ✅ `print_u32()` - Sayı yazdırma helper
- ✅ **"Hello, World!"** mesajı
- ✅ LED blink + serial log kombinasyonu

## 🎯 Step 2'den Farklar

| Özellik | Step 2 (SysTick) | Step 3 (UART) |
|---------|------------------|---------------|
| İletişim | ❌ | ✅ Serial port |
| Debug | LED blink | **Mesaj yazdırma** |
| Peripheral | SysTick | USART3 + GPIO AF |
| API | `delay_ms()` | `uart_puts()` |
| Pin Config | Output mode | **Alternate function** |

## 📡 UART Nedir?

**UART** (Universal Asynchronous Receiver/Transmitter), seri port haberleşmesi için kullanılan bir protokoldür:

- **Asenkron:** Ayrı clock hattı gerekmez
- **Full-duplex:** Aynı anda TX ve RX
- **8N1:** 8 data bit, No parity, 1 stop bit
- **Baud rate:** İletişim hızı (115200 bps kullanıyoruz)

### STM32F439'da USART3

```
USART3 Pinleri:
  PD8 -> TX (Transmit)
  PD9 -> RX (Receive)
  AF7 -> Alternate Function 7 (USART3)
```

## 🔧 Hardware Yapılandırması

### 1. Clock Enable

```rust
// GPIOD ve USART3 clock'larını aç
RCC_AHB1ENR |= RCC_AHB1ENR_GPIODEN;   // GPIOD
RCC_APB1ENR |= RCC_APB1ENR_USART3EN;  // USART3 (APB1 bus'ta)
```

### 2. GPIO Alternate Function

```rust
// PD8, PD9 -> Alternate Function mode (0b10)
GPIOD_MODER |= (0b1010 << (8 * 2));

// AF7 (USART3) seç
GPIOD_AFRH |= (0x77 << 0);  // AF7 = 0b0111
```

### 3. Baud Rate

```rust
// 115200 @ 16 MHz APB1 clock
// BRR = F_APB1 / Baud = 16_000_000 / 115200 ≈ 139
USART3_BRR = 139;
```

### 4. USART Enable

```rust
// UE (USART Enable) | TE (TX Enable) | RE (RX Enable)
USART3_CR1 = USART_CR1_UE | USART_CR1_TE | USART_CR1_RE;
```

## 🚀 Derleme ve Çalıştırma

### Build

```bash
cargo build --release
```

veya:
```bash
chmod +x build.sh
./build.sh
```

### Flash

```bash
cargo run --release
```

### Serial Monitor Bağlantısı

**macOS/Linux:**
```bash
# screen ile
screen /dev/tty.usbmodem* 115200

# Çıkmak için: Ctrl+A sonra K

# veya minicom
minicom -D /dev/tty.usbmodem* -b 115200
```

**Windows:**
```bash
# PuTTY veya TeraTerm
# Port: COMx, Baud: 115200
```

**probe-rs ile (önerilen):**
```bash
# probe-rs hem flash hem de serial monitor sağlar
cargo run --release
```

## 📺 Beklenen Çıktı

Terminal'de göreceğiniz mesajlar:

```
===============================
  Bare-Metal Rust - Step 3
  UART Serial Communication
===============================
Hello, World from STM32F439ZI!

LED Toggle #0
LED Toggle #1
LED Toggle #2
LED Toggle #3
...
```

## 🔍 Kod Detayları

### 1. UART Initialization

```rust
unsafe fn uart_init() {
    // 1. Clock enable
    RCC_AHB1ENR |= RCC_AHB1ENR_GPIODEN;
    RCC_APB1ENR |= RCC_APB1ENR_USART3EN;
    
    // 2. GPIO AF mode
    GPIOD_MODER &= !(0b1111 << (8 * 2));
    GPIOD_MODER |= (0b1010 << (8 * 2));
    
    // 3. AF7 select
    GPIOD_AFRH |= (0x77 << 0);
    
    // 4. Baud rate
    USART3_BRR = 139;  // 115200 @ 16 MHz
    
    // 5. Enable
    USART3_CR1 = USART_CR1_UE | USART_CR1_TE | USART_CR1_RE;
}
```

### 2. Character Transmit

```rust
unsafe fn uart_putc(c: u8) {
    // TXE (TX Empty) flag'ini bekle
    while (read_volatile(USART3_SR) & USART_SR_TXE) == 0 {}
    
    // Karakteri gönder
    write_volatile(USART3_DR, c as u32);
}
```

### 3. String Transmit

```rust
unsafe fn uart_puts(s: &str) {
    for byte in s.bytes() {
        uart_putc(byte);
    }
}
```

### 4. Number Print

```rust
unsafe fn print_u32(n: u32) {
    // Decimal basamaklara ayır
    let mut buffer = [0u8; 10];
    let mut i = 0;
    
    while n > 0 {
        buffer[i] = b'0' + (n % 10) as u8;
        n /= 10;
        i += 1;
    }
    
    // Ters sırayla yazdır
    while i > 0 {
        uart_putc(buffer[i - 1]);
    }
}
```

## 📋 USART Register'ları

```
USART3_SR (Status Register):
  Bit 7: TXE  - Transmit data register empty
  Bit 6: TC   - Transmission complete
  Bit 5: RXNE - Read data register not empty

USART3_DR (Data Register):
  Bit 8-0: Data - TX/RX data (9 bit mode varsa bit 8 de kullanılır)

USART3_BRR (Baud Rate Register):
  Bit 15-0: BRR - Baud rate divider

USART3_CR1 (Control Register 1):
  Bit 13: UE - USART enable
  Bit 3:  TE - Transmitter enable
  Bit 2:  RE - Receiver enable
```

## 🎓 Öğrenilen Kavramlar

1. **UART/USART Protokolü**
   - Asenkron seri haberleşme
   - Baud rate hesaplama
   - 8N1 frame format

2. **GPIO Alternate Function**
   - MODER register (mode selection)
   - AFRL/AFRH register (AF selection)
   - AF7 = USART3 mapping

3. **APB1 Peripheral**
   - USART3, APB1 bus'ta
   - APB1ENR ile clock enable
   - Clock domain farkı (AHB1 vs APB1)

4. **Status Flag Polling**
   - TXE flag'ini beklemek
   - Busy-wait TX transmission
   - Non-blocking alternatifler

## ⚙️ Konfigürasyon

### Farklı Baud Rate

```rust
// 9600 baud
USART3_BRR = 16_000_000 / 9600;  // = 1667

// 921600 baud (yüksek hız)
USART3_BRR = 16_000_000 / 921600;  // = 17
```

### Farklı USART

**USART2 kullanmak için (PA2=TX, PA3=RX):**
```rust
const USART2_BASE: u32 = 0x4000_4400;
// RCC_APB1ENR |= (1 << 17);  // USART2EN
// AF7 for USART2 on PA2/PA3
```

**USART1 kullanmak için (PA9=TX, PA10=RX):**
```rust
const USART1_BASE: u32 = 0x4001_1000;
// RCC_APB2ENR |= (1 << 4);  // USART1EN (APB2'de!)
// AF7 for USART1
```

## 🔜 Sıradaki Adım

**step-4-printf** - Formatlı çıktı desteği
- `core::fmt` trait implementasyonu
- `write!` ve `writeln!` makroları
- `println!("Counter: {}", n)` syntax
- Debug/Display trait'leri

## 📚 Referanslar

- [STM32F439 Reference Manual](https://www.st.com/resource/en/reference_manual/dm00031020.pdf)
  - Chapter 30: USART
  - Section 30.6: USART registers
- [UART Protocol](https://en.wikipedia.org/wiki/Universal_asynchronous_receiver-transmitter)

## ⚠️ Troubleshooting

**Serial çıktı yok?**
1. Doğru COM port'u seçtiğinizden emin olun
2. Baud rate 115200 olmalı
3. probe-rs ile flash ettiyseniz RTT yerine UART kullanıldığından emin olun
4. USART3 pinleri: PD8 (TX), PD9 (RX)

**Garbled characters (bozuk karakterler)?**
1. Baud rate hesaplamasını kontrol edin
2. APB1 clock hızını doğrulayın (16 MHz varsayıyoruz)
3. Kablo bağlantılarını kontrol edin

**Derleme hatası?**
```bash
cargo clean
cargo build --release
```

## 💡 İpuçları

- **CRLF:** Windows terminal için `\r\n` kullanın
- **Buffer:** Büyük mesajlar için buffer eklenebilir
- **Interrupt:** RX için interrupt handler eklemek daha verimli
- **DMA:** Yüksek hızda DMA ile UART kullanılabilir
- **printf:** Bir sonraki step'te `core::fmt` ile gerçek printf gelecek!
