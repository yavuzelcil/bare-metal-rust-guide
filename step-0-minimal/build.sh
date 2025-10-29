#!/usr/bin/env bash
set -e

echo "Building step-0-minimal (bare-metal Rust)..."

# Release build için optimize edilmiş derleme
cargo build --release

echo "Build complete! Binary: target/thumbv7em-none-eabihf/release/bare-metal-rust-step0"
echo ""
echo "Flash to board:"
echo "  cargo run --release"
echo "or"
echo "  probe-rs run --chip STM32F439ZI target/thumbv7em-none-eabihf/release/bare-metal-rust-step0"
