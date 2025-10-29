#!/usr/bin/env bash
set -e

echo "Building step-2-systick (SysTick Timer)..."

# Release build için optimize edilmiş derleme
cargo build --release

echo ""
echo "✅ Build complete!"
echo "Binary: target/thumbv7em-none-eabihf/release/bare-metal-rust-step2-systick"
echo ""
echo "📦 Binary size:"
ls -lh target/thumbv7em-none-eabihf/release/bare-metal-rust-step2-systick | awk '{print $5}'
echo ""
echo "🚀 Flash to board:"
echo "  cargo run --release"
echo "or"
echo "  probe-rs run --chip STM32F439ZI target/thumbv7em-none-eabihf/release/bare-metal-rust-step2-systick"
