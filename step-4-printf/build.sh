#!/usr/bin/env bash
set -e

echo "Building step-4-printf (Formatted Output / Printf)..."

# Release build
cargo build --release

echo ""
echo "✅ Build complete!"
echo "Binary: target/thumbv7em-none-eabihf/release/bare-metal-rust-step4-printf"
echo ""
echo "📦 Binary size:"
ls -lh target/thumbv7em-none-eabihf/release/bare-metal-rust-step4-printf | awk '{print $5}'
echo ""
echo "🚀 Flash to board:"
echo "  cargo run --release"
echo ""
echo "📡 Monitor serial output (115200 baud):"
echo "  screen /dev/tty.usbmodem* 115200"
