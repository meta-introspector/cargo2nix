#!/bin/bash

# Set dummy environment variables for rustc build scripts when mocking
export RUSTC_BOOTSTRAP=1
export CFG_CARGO_FEATURES=""
export CFG_OPT_LEVEL="0"
export CFG_TARGET_FEATURES=""
export CFG_DISABLE_UNSTABLE_FEATURES="0"
export CFG_RELEASE="1.70.0 (000000000 2025-11-30)" # Match version_check format
export CFG_RELEASE_CHANNEL="nightly"
export CFG_VIRTUAL_RUST_SOURCE_BASE_DIR="/dummy/src"
export CFG_VIRTUAL_RUSTC_DEV_SOURCE_BASE_DIR="/dummy/dev_src"
export CFG_DEFAULT_CODEGEN_BACKEND="llvm"
export CFG_VERSION="1.70.0" # Match CFG_RELEASE version part
export CFG_VER_HASH="0000000000000000000000000000000000000000" # 40 zeroes
export CFG_VER_DATE="2025-11-30"
export CFG_LIBDIR_RELATIVE="0"
export CFG_DEFAULT_LINKER="cc"
export CFG_USE_SELF_CONTAINED_LINKER="0"
export CFG_COMPILER_HOST_TRIPLE="x86_64-unknown-linux-gnu" # Adjust to your host triple if necessary

# Set RUSTC_BUILD_SYSROOT to prevent build scripts from trying to find a real sysroot
export RUSTC_BUILD_SYSROOT="/dummy/sysroot"

# Execute the command passed to the script
exec "$@"
