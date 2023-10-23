# Odroid Build

```
ln -s /etc/ssl "$HOME/odroidxu4_sdk/sysroots/x86_64-arcadiasdk-linux/etc/ssl"
cp "$HOME/git/guster32/scripts/.build/buildArcadiaDevOdroidxu4/build/tmp-glibc/sysroots-components/x86_64-nativesdk/nativesdk-clang/usr/local/oe-sdk-hardcoded-buildpath/sysroots/x86_64-arcadiasdk-linux/usr/lib/libclang.so.15.0.7" "$HOME/odroidxu4_sdk/sysroots/x86_64-arcadiasdk-linux/usr/lib/"

export RUST_ARCADIA_TARGET_LIBS=$HOME/odroidxu4_sdk/sysroots/cortexa15t2hf-neon-vfpv4-arcadia-linux-gnueabi/usr/lib/rustlib/armv7-arcadia-linux-gnueabihf/lib
mv $RUST_ARCADIA_TARGET_LIBS/libcore-a6490b301bea3f41.rlib $RUST_ARCADIA_TARGET_LIBS/_libcore-a6490b301bea3f41.rlib
mv $RUST_ARCADIA_TARGET_LIBS/libcompiler_builtins-2cdc1e73a0a6ad22.rlib $RUST_ARCADIA_TARGET_LIBS/_libcompiler_builtins-2cdc1e73a0a6ad22.rlib
mv $RUST_ARCADIA_TARGET_LIBS/libstd-07d3c55a6b8c0863.so $RUST_ARCADIA_TARGET_LIBS/_libstd-07d3c55a6b8c0863.so
mv $RUST_ARCADIA_TARGET_LIBS/libstd-07d3c55a6b8c0863.rlib $RUST_ARCADIA_TARGET_LIBS/_libstd-07d3c55a6b8c0863.rlib
mv $RUST_ARCADIA_TARGET_LIBS/libstd_detect-b052216a048b8881.rlib $RUST_ARCADIA_TARGET_LIBS/_libstd_detect-b052216a048b8881.rlib

. "$HOME/odroidxu4_sdk/environment-setup-cortexa15t2hf-neon-vfpv4-arcadia-linux-gnueabi"
export CLANG_PATH="$HOME/odroidxu4_sdk/sysroots/x86_64-arcadiasdk-linux/usr/bin/clang"
export LIBCLANG_PATH="$HOME/odroidxu4_sdk/sysroots/x86_64-arcadiasdk-linux/usr/lib/"
export LLVM_CONFIG_PATH="$HOME/odroidxu4_sdk/sysroots/x86_64-arcadiasdk-linux/usr/bin/llvm-config"
export BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$HOME/odroidxu4_sdk/sysroots/cortexa15t2hf-neon-vfpv4-arcadia-linux-gnueabi/"
RUSTFLAGS="$RUSTFLAGS -L /usr/lib/rustlib/armv7-arcadia-linux-gnueabihf/lib -C link-arg=-Wl,-dynamic-linker=/lib/ld-linux-armhf.so.3 -C linker=arm-arcadia-linux-gnueabi-gcc" cargo build
```

# Qemu Build

```
ln -s /etc/ssl "$HOME/qemux86-64_sdk/sysroots/x86_64-arcadiasdk-linux/etc/ssl"
cp "$HOME/git/guster32/scripts/.build/buildArcadiaDevQemu/build/tmp-glibc/sysroots-components/x86_64-nativesdk/nativesdk-clang/usr/local/oe-sdk-hardcoded-buildpath/sysroots/x86_64-arcadiasdk-linux/usr/lib/libclang.so.15.0.7" "$HOME/qemux86-64_sdk/sysroots/x86_64-arcadiasdk-linux/usr/lib/"

export RUST_ARCADIA_TARGET_LIBS=$HOME/qemux86-64_sdk/sysroots/core2-64-arcadia-linux/usr/lib/rustlib/x86_64-arcadia-linux-gnu/lib
mv $RUST_ARCADIA_TARGET_LIBS/libstd-f1beb7fa065ddb8a.rlib $RUST_ARCADIA_TARGET_LIBS/_libstd-f1beb7fa065ddb8a.rlib
mv $RUST_ARCADIA_TARGET_LIBS/libstd-f1beb7fa065ddb8a.so $RUST_ARCADIA_TARGET_LIBS/_libstd-f1beb7fa065ddb8a.so
mv $RUST_ARCADIA_TARGET_LIBS/libcore-529b855680093334.rlib $RUST_ARCADIA_TARGET_LIBS/_libcore-529b855680093334.rlib
mv $RUST_ARCADIA_TARGET_LIBS/libcompiler_builtins-f4bf402e27e86439.rlib $RUST_ARCADIA_TARGET_LIBS/_libcompiler_builtins-f4bf402e27e86439.rlib


. "$HOME/qemux86-64_sdk/environment-setup-core2-64-arcadia-linux"
export CLANG_PATH="$HOME/qemux86-64_sdk/sysroots/x86_64-arcadiasdk-linux/usr/bin/clang"
export LIBCLANG_PATH="$HOME/qemux86-64_sdk/sysroots/x86_64-arcadiasdk-linux/usr/lib/"
export LLVM_CONFIG_PATH="$HOME/qemux86-64_sdk/sysroots/x86_64-arcadiasdk-linux/usr/bin/llvm-config"
export BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$HOME/qemux86-64_sdk/sysroots/core2-64-arcadia-linux/"
RUSTFLAGS="$RUSTFLAGS -L /usr/lib/rustlib/x86_64-arcadiasdk-linux-gnu/lib -C link-arg=-Wl,-dynamic-linker=/lib/ld-linux-x86-64.so.2 -C linker=x86_64-arcadia-linux-gcc" cargo build

```