#!/bin/bash

source "${ENVIRONMENT_SCRIPT}"

pushd "${SRC_DIR}/" || exit

export CLANG_PATH="${OECORE_NATIVE_SYSROOT}/usr/bin/clang"
export LIBCLANG_PATH="${OECORE_NATIVE_SYSROOT}/usr/lib/"
export LLVM_CONFIG_PATH="${OECORE_NATIVE_SYSROOT}/usr/bin/llvm-config"
export BINDGEN_EXTRA_CLANG_ARGS="--sysroot=${SDKTARGETSYSROOT}/"
RUSTFLAGS="$RUSTFLAGS -L /usr/lib/rustlib/${RUST_TARGET}/lib -C link-arg=-Wl,-dynamic-linker=/lib/${DYNAMIC_LINKER} -C linker=${LINKER}" cargo build
