#!/bin/bash
set -e

# Build script for local development
# Usage: ./scripts/build.sh [--release]

RELEASE_FLAG=""
if [ "$1" == "--release" ]; then
    RELEASE_FLAG="--release"
    echo "🔨 Building in RELEASE mode..."
else
    echo "🔨 Building in DEBUG mode..."
fi

echo ""

# Build for testing (native)
echo "📦 Building for native target (tests)..."
cargo build ${RELEASE_FLAG} --target x86_64-unknown-linux-gnu

echo "✅ Native build complete"
echo ""

# Build WASM
echo "📦 Building WASM target..."
cargo build ${RELEASE_FLAG} --target wasm32-unknown-unknown

WASM_PATH="target/wasm32-unknown-unknown"
if [ "$1" == "--release" ]; then
    WASM_FILE="${WASM_PATH}/release/dprint_plugin_tailwindcss.wasm"
else
    WASM_FILE="${WASM_PATH}/debug/dprint_plugin_tailwindcss.wasm"
fi

if [ -f "$WASM_FILE" ]; then
    WASM_SIZE=$(du -h "$WASM_FILE" | cut -f1)
    echo "✅ WASM build complete: ${WASM_SIZE}"
    echo "   Location: ${WASM_FILE}"
else
    echo "❌ WASM file not found!"
    exit 1
fi

echo ""
echo "✅ Build complete!"
echo ""
