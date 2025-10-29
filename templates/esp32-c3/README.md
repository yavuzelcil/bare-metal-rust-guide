# ESP32-C3 Template

Bu şablon, ESP32-C3 mikrodenetleyici için temel bare-metal Rust projesi yapılandırmasını içerir.

## Özellikler

- **CPU:** RISC-V single-core @ 160 MHz
- **Flash:** 4 MB embedded flash
- **SRAM:** 400 KB (DRAM)
- **WiFi:** 2.4 GHz 802.11 b/g/n
- **Bluetooth:** BLE 5.0

## Dosyalar

- `memory.x` - Linker script (ESP32-C3 memory layout)
- `config.toml` - Cargo build configuration
- `Cargo.toml` - Temel dependencies

## Kullanım

1. Bu şablonu yeni projenize kopyalayın:
   ```bash
   cp -r templates/esp32-c3 my-project/
   ```

2. Target'ı yükleyin (eğer yüklü değilse):
   ```bash
   rustup target add riscv32imc-unknown-none-elf
   ```

3. Build edin:
   ```bash
   cd my-project
   cargo build --release
   ```

4. Flash edin (espflash veya probe-rs ile):
   ```bash
   cargo run --release
   # veya
   # espflash flash target/riscv32imc-unknown-none-elf/release/my-project
   ```

## Tool Installation

ESP32-C3 için `espflash` kurulumu:
```bash
cargo install espflash
```

veya probe-rs (ESP32-C3 desteği ile):
```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

## Serial Monitor

```bash
espflash monitor
# veya
screen /dev/tty.usbserial-* 115200
```

## Önemli Notlar

- ESP32-C3, RISC-V mimarisi kullanır (ARM değil)
- WiFi/BLE kullanımı için ek crate'ler gerekir (esp-hal, esp-wifi)
- Flash layout dikkatli yapılandırılmalıdır (bootloader, partition table)
