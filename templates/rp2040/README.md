# RP2040 (Raspberry Pi Pico) Template

Bu şablon, RP2040 mikrodenetleyici (Raspberry Pi Pico) için temel bare-metal Rust projesi yapılandırmasını içerir.

## Özellikler

- **CPU:** Dual ARM Cortex-M0+ @ 133 MHz
- **Flash:** 2 MB external QSPI flash (XIP via 0x10000000)
- **SRAM:** 264 KB (0x20000000)
- **Unique:** PIO (Programmable I/O) state machines

## Dosyalar

- `memory.x` - Linker script (RP2040 memory layout)
- `config.toml` - Cargo build configuration
- `Cargo.toml` - Temel dependencies

## Kullanım

1. Bu şablonu yeni projenize kopyalayın:
   ```bash
   cp -r templates/rp2040 my-project/
   ```

2. Target'ı yükleyin (eğer yüklü değilse):
   ```bash
   rustup target add thumbv6m-none-eabi
   ```

3. Build edin:
   ```bash
   cd my-project
   cargo build --release
   ```

4. Flash edin (probe-rs veya picotool ile):
   ```bash
   cargo run --release
   # veya UF2 bootloader ile:
   # elf2uf2-rs target/thumbv6m-none-eabi/release/my-project my-project.uf2
   ```

## probe-rs Chip ID

Flash/run için kullanılacak chip ID:
- `RP2040`

## UF2 Bootloader

RP2040'ı BOOTSEL butonu ile başlatarak UF2 modunda flash edebilirsiniz:
1. BOOTSEL basılı tutarak Pico'yu USB'ye takın
2. Disk olarak mount olacak
3. `.uf2` dosyasını sürükle-bırak ile kopyalayın
