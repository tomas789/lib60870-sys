# Release Process

This document describes how to release a new version of the `lib60870-sys` crate.

## Release Steps

### 1. Update the version

Edit `Cargo.toml` and update the version number:

```toml
[package]
version = "X.Y.Z"
```

### 2. Regenerate pre-generated bindings (if needed)

If you've updated the lib60870 C library version, regenerate the bindings:

```bash
rm -rf target
LIB60870_SYS_UPDATE_PREGENERATED_BINDINGS=1 cargo build
```

This is required for docs.rs to build documentation (it has no network access).

### 3. Commit and push

```bash
git add -A
git commit -m "Release vX.Y.Z"
git push origin main
```

### 4. Create a GitHub release

1. Go to **Releases → Draft a new release**
2. Create a new tag: `vX.Y.Z` (must match `Cargo.toml` version with `v` prefix)
3. Set the release title (e.g., `v0.2.0`)
4. Add release notes describing the changes
5. Click **Publish release**

## What happens automatically

When you publish a GitHub release:

1. The `release.yml` workflow triggers
2. It verifies the tag version matches `Cargo.toml`
3. Builds and tests the crate
4. Publishes to [crates.io](https://crates.io/crates/lib60870-sys)
5. docs.rs automatically builds documentation using the pre-generated bindings
