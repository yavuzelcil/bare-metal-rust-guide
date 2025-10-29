#!/usr/bin/env bash
set -e

echo "Building step-3-uart (UART Serial Communication)..."

# Release build
cargo build --release

echo ""
echo "✅ Build complete!"
echo "Binary: target/thumbv7em-none-eabihf/release/bare-metal-rust-step3-uart"
echo ""
echo "📦 Binary size:"
ls -lh target/thumbv7em-none-eabihf/release/bare-metal-rust-step3-uart | awk '{print $5}'
echo ""
echo "🚀 Flash to board:"
echo "  cargo run --release"
echo ""
echo "📡 Monitor serial output:"
echo "  screen /dev/tty.usbmodem* 115200"
echo "  # or: minicom -D /dev/tty.usbmodem* -b 115200"
echo "  # Exit screen: Ctrl+A then K"
