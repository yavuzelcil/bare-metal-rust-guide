#!/usr/bin/env bash
set -e

echo "Building step-1-blinky (GPIO LED Blink)..."

# Release build için optimize edilmiş derleme
cargo build --release

echo ""
echo "✅ Build complete!"
echo "Binary: target/thumbv7em-none-eabihf/release/bare-metal-rust-step1-blinky"
echo ""
echo "📦 Binary size:"
ls -lh target/thumbv7em-none-eabihf/release/bare-metal-rust-step1-blinky | awk '{print $5}'
echo ""
echo "🚀 Flash to board:"
echo "  cargo run --release"
echo "or"
echo "  probe-rs run --chip STM32F439ZI target/thumbv7em-none-eabihf/release/bare-metal-rust-step1-blinky"
