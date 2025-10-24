# Release Process

This document describes the release process for dprint-plugin-tailwindcss.

## Prerequisites

- [ ] All tests passing (`cargo test --lib --target x86_64-unknown-linux-gnu`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Code formatted (`cargo fmt`)
- [ ] Documentation up to date
- [ ] CHANGELOG.md updated
- [ ] On `main` branch
- [ ] No uncommitted changes

## Automated Release (Recommended)

### 1. Prepare Release

```bash
# Run the release script
./scripts/release.sh <version>

# Example:
./scripts/release.sh 0.1.0
```

This script will:
- ✅ Check pre-flight conditions (branch, uncommitted changes)
- ✅ Update version in `Cargo.toml`
- ✅ Run all tests
- ✅ Build optimized WASM
- ✅ Create release artifacts in `release/v<version>/`
- ✅ Generate `plugin-info.json`
- ✅ Commit version bump
- ✅ Create git tag

### 2. Push Release

```bash
# Push main branch
git push origin main

# Push tag
git push origin v<version>
```

### 3. GitHub Actions

GitHub Actions will automatically:
- Create GitHub release
- Upload WASM binary
- Upload plugin-info.json

## Manual Release

### 1. Update Version

Edit `Cargo.toml`:
```toml
[package]
version = "0.1.0"  # Update version here
```

### 2. Update CHANGELOG

Edit `CHANGELOG.md`:
```markdown
## [0.1.0] - 2025-10-24

### Added
- Feature 1
- Feature 2

### Fixed
- Bug 1
```

### 3. Run Tests

```bash
cargo test --lib --target x86_64-unknown-linux-gnu
```

### 4. Build WASM

```bash
cargo build --release --target wasm32-unknown-unknown
```

### 5. Verify Build

```bash
# Check file size
ls -lh target/wasm32-unknown-unknown/release/dprint_plugin_tailwindcss.wasm

# Verify it's a valid WASM file
file target/wasm32-unknown-unknown/release/dprint_plugin_tailwindcss.wasm
```

### 6. Create Git Tag

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to <version>"
git tag -a "v<version>" -m "Release v<version>"
git push origin main
git push origin v<version>
```

### 7. Create GitHub Release

1. Go to https://github.com/friedjoff/dprint-plugin-tailwindcss/releases/new
2. Select the tag `v<version>`
3. Set release title: "Release v<version>"
4. Copy content from CHANGELOG.md
5. Upload `dprint_plugin_tailwindcss.wasm`
6. Publish release

## Post-Release

### 1. Test Installation

```bash
# Create test project
mkdir test-install
cd test-install

# Create dprint.json
cat > dprint.json << EOF
{
  "plugins": [
    "https://github.com/friedjoff/dprint-plugin-tailwindcss/releases/download/v<version>/dprint_plugin_tailwindcss.wasm"
  ]
}
EOF

# Install plugin
dprint config update

# Test formatting
echo '<div class="p-4 mt-2">Test</div>' > test.html
dprint fmt test.html
cat test.html  # Should show sorted classes
```

### 2. Update Documentation

- Update main README.md with new version number
- Update installation examples
- Announce on social media/Discord/forums

### 3. Monitor Issues

Watch for any issues reported by early adopters.

## Version Numbering

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR** version: Incompatible API changes
- **MINOR** version: New functionality (backwards-compatible)
- **PATCH** version: Bug fixes (backwards-compatible)

Examples:
- `0.1.0` - Initial release
- `0.1.1` - Bug fix
- `0.2.0` - New feature
- `1.0.0` - Stable API

## Release Checklist

### Pre-Release
- [ ] All tests passing
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Version bumped in Cargo.toml
- [ ] No compiler warnings

### Release
- [ ] Git tag created
- [ ] GitHub release created
- [ ] WASM binary uploaded
- [ ] plugin-info.json uploaded

### Post-Release
- [ ] Installation tested
- [ ] Documentation links updated
- [ ] Announcement published
- [ ] Issues monitored

## Rollback Procedure

If a release needs to be rolled back:

1. **Delete GitHub release**:
   - Go to Releases page
   - Delete problematic release
   - Delete git tag: `git push --delete origin v<version>`

2. **Fix issues**:
   - Create fix branch
   - Make corrections
   - Test thoroughly

3. **Re-release**:
   - Bump patch version (e.g., 0.1.0 → 0.1.1)
   - Follow normal release process

## Troubleshooting

### WASM Build Fails

```bash
# Clean build
cargo clean

# Rebuild
cargo build --release --target wasm32-unknown-unknown
```

### Tests Fail

```bash
# Run specific failing test
cargo test --lib --target x86_64-unknown-linux-gnu <test_name> -- --nocapture

# Check for recent changes
git diff HEAD~1
```

### Version Mismatch

```bash
# Check Cargo.toml version
grep "^version" Cargo.toml

# Check git tag
git describe --tags
```

## Scripts Reference

### `scripts/build.sh`

```bash
# Debug build
./scripts/build.sh

# Release build
./scripts/build.sh --release
```

### `scripts/release.sh`

```bash
# Create release
./scripts/release.sh <version>

# Example
./scripts/release.sh 0.1.0
```

## GitHub Actions Workflows

### `.github/workflows/ci.yml`
- Runs on: Push to main/develop, PRs
- Actions: Test, format check, clippy, build WASM

### `.github/workflows/release.yml`
- Runs on: Push tag `v*.*.*`
- Actions: Create release, build WASM, upload artifacts

## Support

For release questions:
- GitHub Issues: https://github.com/friedjoff/dprint-plugin-tailwindcss/issues
- Discussions: https://github.com/friedjoff/dprint-plugin-tailwindcss/discussions

---

**Last Updated**: Step 9 implementation  
**Maintained By**: @friedjoff
