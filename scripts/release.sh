#!/bin/bash
set -e

# Build and release script for dprint-plugin-tailwindcss
# Usage: ./scripts/release.sh [version]

VERSION=${1:-"0.1.0"}

echo "🚀 Building dprint-plugin-tailwindcss v${VERSION}"
echo ""

# Check if we're on main branch
BRANCH=$(git branch --show-current)
if [ "$BRANCH" != "main" ]; then
    echo "❌ Error: Must be on main branch to release"
    exit 1
fi

# Check for uncommitted changes
if [ -n "$(git status --porcelain)" ]; then
    echo "❌ Error: There are uncommitted changes"
    exit 1
fi

echo "✅ Pre-flight checks passed"
echo ""

# Update version in Cargo.toml
echo "📝 Updating version in Cargo.toml..."
sed -i "s/^version = .*/version = \"${VERSION}\"/" Cargo.toml

# Run tests
echo "🧪 Running tests..."
cargo test --lib --target x86_64-unknown-linux-gnu --quiet

echo "✅ Tests passed"
echo ""

# Build WASM
echo "🔨 Building WASM (release)..."
cargo build --release --target wasm32-unknown-unknown

WASM_FILE="target/wasm32-unknown-unknown/release/dprint_plugin_tailwindcss.wasm"
WASM_SIZE=$(du -h "$WASM_FILE" | cut -f1)

echo "✅ WASM built: ${WASM_SIZE}"
echo ""

# Verify WASM
echo "🔍 Verifying WASM..."
file "$WASM_FILE"
echo ""

# Create release directory
RELEASE_DIR="release/v${VERSION}"
mkdir -p "$RELEASE_DIR"

# Copy WASM
cp "$WASM_FILE" "$RELEASE_DIR/"

echo "✅ Release artifacts created in ${RELEASE_DIR}/"
echo ""

# Create plugin info JSON
cat > "$RELEASE_DIR/plugin-info.json" << EOF
{
  "schemaVersion": 4,
  "name": "dprint-plugin-tailwindcss",
  "version": "${VERSION}",
  "linux-x86_64": {
    "reference": "https://github.com/friedjoff/dprint-plugin-tailwindcss/releases/download/v${VERSION}/dprint_plugin_tailwindcss.wasm",
    "checksum": ""
  },
  "mac-x86_64": {
    "reference": "https://github.com/friedjoff/dprint-plugin-tailwindcss/releases/download/v${VERSION}/dprint_plugin_tailwindcss.wasm",
    "checksum": ""
  },
  "mac-aarch64": {
    "reference": "https://github.com/friedjoff/dprint-plugin-tailwindcss/releases/download/v${VERSION}/dprint_plugin_tailwindcss.wasm",
    "checksum": ""
  },
  "windows-x86_64": {
    "reference": "https://github.com/friedjoff/dprint-plugin-tailwindcss/releases/download/v${VERSION}/dprint_plugin_tailwindcss.wasm",
    "checksum": ""
  }
}
EOF

echo "✅ plugin-info.json created"
echo ""

# Commit version bump
echo "📝 Committing version bump..."
git add Cargo.toml
git commit -m "chore: bump version to ${VERSION}"

# Create git tag
echo "🏷️  Creating git tag v${VERSION}..."
git tag -a "v${VERSION}" -m "Release v${VERSION}"

echo ""
echo "✅ Release preparation complete!"
echo ""
echo "📦 Next steps:"
echo "   1. Review changes: git show"
echo "   2. Push changes: git push origin main"
echo "   3. Push tag: git push origin v${VERSION}"
echo "   4. GitHub Actions will create the release automatically"
echo ""
echo "   Or manually upload from: ${RELEASE_DIR}/"
echo ""
