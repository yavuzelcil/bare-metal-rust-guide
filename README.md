# Bare-Metal Rust Guide

Bu repo, bare-metal (işletim sistemi olmadan) Rust programlama için adım adım rehber ve örnek projeler içerir. Her `step-N-*` dizini, belirli bir konuyu öğretmek için tasarlanmış bağımsız bir projedir.

## 📚 İçerik

### Adım Adım Öğrenme

| Adım | Konu | Açıklama |
|------|------|----------|
| [step-0-minimal](./step-0-minimal/) | **Minimal Program** | En basit bare-metal Rust programı: `#![no_std]`, `#![no_main]`, panic handler ve reset vektörü |
| [step-1-blinky](./step-1-blinky/) | **LED Blink** | GPIO register'larına doğrudan erişim, LED kontrol |
| [step-2-systick](./step-2-systick/) | **SysTick Timer** | ARM Cortex-M SysTick timer, interrupt handler, hassas zamanlama |
| [step-3-uart](./step-3-uart/) | **UART Serial** | UART yapılandırması, seri port iletişimi, echo programı |
| [step-4-printf](./step-4-printf/) | **Printf** | Formatlı çıktı, `core::fmt` trait'leri, debug altyapısı |
| [step-5-storage](./step-5-storage/) | **Key-Value Storage** | Flash emulation, key-value storage, set/get/delete operasyonları |
| [step-6-webserver](./step-6-webserver/) | **Web Server** | HTTP server, LED kontrolü, web UI, REST API |

### Platform Şablonları (Templates)

`templates/` dizini, farklı mikrodenetleyiciler için hazır yapılandırma şablonları içerir:

- **[stm32f429zi](./templates/stm32f429zi/)** - STM32F429ZI (ARM Cortex-M4F, 180 MHz, 2MB Flash)
- **[rp2040](./templates/rp2040/)** - Raspberry Pi Pico (Dual Cortex-M0+, 133 MHz, 264KB RAM)
- **[esp32-c3](./templates/esp32-c3/)** - ESP32-C3 (RISC-V, 160 MHz, WiFi/BLE)

Her şablon şunları içerir:
- `memory.x` - Linker script (memory layout)
- `config.toml` - Cargo build yapılandırması
- `Cargo.toml` - Bağımlılıklar ve profile ayarları
- `README.md` - Platforma özel kullanım talimatları

## 🚀 Hızlı Başlangıç

### Gereksinimler

1. **Rust toolchain** yükleyin:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Target architecture** ekleyin:
   ```bash
   # STM32F429ZI için (ARM Cortex-M4F with FPU)
   rustup target add thumbv7em-none-eabihf
   
   # RP2040 için (ARM Cortex-M0+)
   rustup target add thumbv6m-none-eabi
   
   # ESP32-C3 için (RISC-V)
   rustup target add riscv32imc-unknown-none-elf
   ```

3. **Flash tool** yükleyin:
   ```bash
   # probe-rs (önerilen, çoklu platform desteği)
   curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
   
   # veya ESP32 için espflash
   cargo install espflash
   ```

### İlk Projeyi Çalıştırma

```bash
# Minimal programı dene
cd step-0-minimal

# Derleme
cargo build --release

# Flash et (kartınız bağlıysa)
cargo run --release

# veya build script ile
chmod +x build.sh
./build.sh
```

## 📖 Nasıl Kullanılır?

### 1. Sırayla İlerleyin

En basit projeden (step-0-minimal) başlayarak adım adım ilerleyin. Her step, bir önceki bilgiye dayanır.

### 2. Kodu İnceleyin

Her dizinde:
- `src/main.rs` - Ana kaynak kod
- `memory.x` - Memory layout tanımı
- `Cargo.toml` - Proje yapılandırması
- `.cargo/config.toml` - Build ayarları
- `README.md` - Detaylı açıklama

### 3. Deneyip Öğrenin

- Kodu değiştirin, yeniden derleyin
- Farklı register değerleri deneyin
- Kendi özelliklerinizi ekleyin

### 4. Farklı Platform Deneyin

`templates/` dizinindeki şablonları kullanarak farklı mikrodenetleyicilere adapte edin:

```bash
# Örnek: RP2040 için yeni proje
cp -r templates/rp2040 my-rp2040-project
cd my-rp2040-project
# Kendi kodunuzu ekleyin
cargo build --release
```

## 🛠️ Araçlar ve Yardımcı Programlar

### Debugging

```bash
# probe-rs ile debug
probe-rs attach --chip STM32F429ZITx

# GDB ile debug
cargo run --release
# Ayrı terminalde:
arm-none-eabi-gdb target/thumbv7em-none-eabihf/release/binary
```

### Serial Monitor

```bash
# macOS/Linux
screen /dev/tty.usbmodem* 115200

# probe-rs ile
probe-rs run --chip STM32F429ZITx

# ESP32 için
espflash monitor
```

### Binary Analizi

```bash
# Binary boyutunu görüntüle
cargo size --release -- -A

# Disassembly
cargo objdump --release -- -d

# Symbol tablosu
cargo nm --release
```

## 📝 Proje Yapısı

```
bare-metal-rust/
├── README.md                  # Bu dosya
├── step-0-minimal/            # Minimal bare-metal program
│   ├── src/main.rs
│   ├── memory.x
│   ├── Cargo.toml
│   ├── .cargo/config.toml
│   └── build.sh
├── step-1-blinky/             # GPIO ve LED kontrol
├── step-2-systick/            # Timer ve interrupt
├── step-3-uart/               # Serial iletişim
├── step-4-printf/             # Formatlı çıktı
├── step-5-storage/            # Key-value storage
├── step-6-webserver/          # HTTP server ve web UI
└── templates/                 # Platform şablonları
    ├── stm32f429zi/
    ├── rp2040/
    └── esp32-c3/
```

## 🎯 Öğrenme Hedefleri

Bu repo ile şunları öğreneceksiniz:

- ✅ `#![no_std]` Rust programlama
- ✅ Memory layout ve linker script'ler
- ✅ Hardware register'larına doğrudan erişim
- ✅ Interrupt handler'lar
- ✅ Peripheral driver geliştirme
- ✅ Embedded sistem optimizasyonları
- ✅ Cross-compilation ve deployment

## 🔗 Kaynaklar

### Resmi Dökümanlar
- [The Embedded Rust Book](https://rust-embedded.github.io/book/)
- [Embedonomicon](https://docs.rust-embedded.org/embedonomicon/)
- [probe-rs Documentation](https://probe.rs/)

### Crate'ler
- [cortex-m](https://crates.io/crates/cortex-m) - Cortex-M processor support
- [cortex-m-rt](https://crates.io/crates/cortex-m-rt) - Runtime support
- [embedded-hal](https://crates.io/crates/embedded-hal) - Hardware abstraction traits

### Datasheets
- [STM32F429 Reference Manual](https://www.st.com/resource/en/reference_manual/dm00031020.pdf)
- [RP2040 Datasheet](https://datasheets.raspberrypi.com/rp2040/rp2040-datasheet.pdf)
- [ESP32-C3 Technical Reference](https://www.espressif.com/sites/default/files/documentation/esp32-c3_technical_reference_manual_en.pdf)

## 🤝 Katkıda Bulunma

Katkılarınızı bekliyoruz! Lütfen:
1. Fork yapın
2. Feature branch oluşturun (`git checkout -b feature/yeni-ozellik`)
3. Commit edin (`git commit -am 'Yeni özellik: ...'`)
4. Push edin (`git push origin feature/yeni-ozellik`)
5. Pull Request açın

## 📄 Lisans

Bu proje MIT lisansı altında yayınlanmıştır.

## 💡 İpuçları

- **Optimizasyon:** Release build'lerde `-Oz` veya `-Os` kullanarak binary boyutunu küçültün
- **Debug:** `probe-rs` ile canlı debug yaparken RTT (Real-Time Transfer) kullanın
- **Safety:** `unsafe` kullanırken dikkatli olun, register erişimleri volatile olmalı
- **Documentation:** Datasheet'leri yanınızda bulundurun, register adresleri ve bit tanımları için

---

**Mutlu kodlamalar! 🦀**
