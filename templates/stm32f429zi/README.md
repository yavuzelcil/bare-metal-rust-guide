# STM32F429ZI Template

Bu şablon, STM32F429ZI mikrodenetleyici için temel bare-metal Rust projesi yapılandırmasını içerir.

## Özellikler

- **CPU:** ARM Cortex-M4F @ 180 MHz
- **Flash:** 2048 KB (0x08000000)
- **RAM:** 192 KB (0x20000000) + 64 KB CCM (0x10000000)
- **FPU:** Single precision floating point unit

## Dosyalar

- `memory.x` - Linker script (memory layout)
- `config.toml` - Cargo build configuration
- `Cargo.toml` - Temel dependencies

## Kullanım

1. Bu şablonu yeni projenize kopyalayın:
   ```bash
   cp -r templates/stm32f429zi my-project/
   ```

2. Target'ı yükleyin (eğer yüklü değilse):
   ```bash
   rustup target add thumbv7em-none-eabihf
   ```

3. Build edin:
   ```bash
   cd my-project
   cargo build --release
   ```

4. Flash edin (probe-rs ile):
   ```bash
   cargo run --release
   ```

## probe-rs Chip ID

Flash/run için kullanılacak chip ID:
- `STM32F429ZITx`
- `STM32F429ZI`
- `STM32F439ZI` (variant)
