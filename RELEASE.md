# Release Process

This document describes how to create releases for tempo-proxy.

## Automatic Releases

Releases are automatically created when a tag is pushed to the repository.

### Creating a Release

1. **Make sure all changes are merged to `main`**
2. **Create and push a tag:**

```bash
# Create a new version tag
git tag v1.0.0

# Push the tag to trigger the release workflow
git push origin v1.0.0
```

3. **Monitor the GitHub Actions workflow** at `https://github.com/your-username/tempo-proxy/actions`

4. **The release will be automatically created** with binaries for all supported platforms

### Supported Platforms

The release workflow builds binaries for:

- **Linux**:
  - `tempo-proxy-linux-amd64` (x86_64, glibc)
  - `tempo-proxy-linux-amd64-musl` (x86_64, musl - static)
  - `tempo-proxy-linux-arm64` (aarch64)

- **macOS**:
  - `tempo-proxy-macos-amd64` (Intel Mac)
  - `tempo-proxy-macos-arm64` (Apple Silicon Mac)

- **Windows**:
  - `tempo-proxy-windows-amd64.exe` (x86_64)

## Local Testing

Before creating a release, you can test the build process locally:

```bash
# Build and test locally
./scripts/build-release.sh

# Test the binary
./target/release/tempo-proxy serve --interface 127.0.0.1:8080
```

## Version Naming

Use semantic versioning:

- `v1.0.0` - Major release
- `v1.1.0` - Minor release (new features)
- `v1.0.1` - Patch release (bug fixes)
- `v1.0.0-alpha.1` - Pre-release (will be marked as pre-release)

## Manual Release

If you need to create a release manually:

1. Go to GitHub Releases page
2. Click "Create a new release"
3. Create a new tag (e.g., `v1.0.0`)
4. Fill in the release title and description
5. The workflow will automatically build and attach binaries

## Troubleshooting

### Build Failures

If the build fails:

1. Check the Actions tab for detailed logs
2. Verify that the web build completes successfully
3. Ensure all Rust code compiles without warnings
4. Test locally with `./scripts/build-release.sh`

### Missing Binaries

If some binaries are missing from the release:

1. Check if the specific target failed in the Actions log
2. The workflow may need updates for cross-compilation
3. Re-run the workflow if it was a temporary issue

## Release Notes

The workflow automatically generates release notes based on:

- Commit messages since the last release
- Pull request titles and descriptions

For better release notes, use clear commit messages and PR descriptions.