# Step 0: Minimal Bare-Metal Rust

Bu adım, en minimal bare-metal Rust programını içerir:
- `#![no_std]` ve `#![no_main]` ile standart kütüphane kullanmadan
- `panic_handler` ile panic durumları ele alınır
- `Reset` fonksiyonu ile MCU reset vektörü tanımlanır
- Sonsuz döngü ile program çalışır durumda kalır

## Derleme

```bash
cargo build --release
```

veya

```bash
./build.sh
```

## Flash

```bash
cargo run --release
```

## Gereksinimler

- `thumbv7em-none-eabihf` target yüklü olmalı:
  ```bash
  rustup target add thumbv7em-none-eabihf
  ```
- `probe-rs` kurulu olmalı (flash için)

## Hedef Kart

- **STM32F439ZI** (veya F429ZI)
- Flash: 2MB (0x08000000)
- RAM: 192KB (0x20000000)
