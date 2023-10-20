# Odroid Build

```
. /home/guster32/odroidxu4_sdk/environment-setup-cortexa15t2hf-neon-vfpv4-arcadia-linux-gnueabi
ln -s /etc/ssl /home/guster32/odroidxu4_sdk/sysroots/x86_64-arcadiasdk-linux/etc/ssl
ln -s /home/guster32/odroidxu4_sdk/sysroots/x86_64-arcadiasdk-linux/usr/lib/libclang-cpp.so.15 /home/guster32/odroidxu4_sdk/sysroots/x86_64-arcadiasdk-linux/usr/lib/libclang.so.15
cp /home/guster32/git/guster32/scripts/.build/buildArcadiaDevOdroidxu4/build/tmp-glibc/sysroots-components/x86_64-nativesdk/nativesdk-clang/usr/local/oe-sdk-hardcoded-buildpath/sysroots/x86_64-arcadiasdk-linux/usr/lib/libclang.so.15.0.7 /home/guster32/odroidxu4_sdk/sysroots/x86_64-arcadiasdk-linux/usr/lib/

export CLANG_PATH=/home/guster32/odroidxu4_sdk/sysroots/x86_64-arcadiasdk-linux/usr/bin/clang
export LIBCLANG_PATH=/home/guster32/odroidxu4_sdk/sysroots/x86_64-arcadiasdk-linux/usr/lib/
export LLVM_CONFIG_PATH=/home/guster32/odroidxu4_sdk/sysroots/x86_64-arcadiasdk-linux/usr/bin/llvm-config
export BINDGEN_EXTRA_CLANG_ARGS="--sysroot=/home/guster32/odroidxu4_sdk/sysroots/cortexa15t2hf-neon-vfpv4-arcadia-linux-gnueabi/"
RUSTFLAGS="$RUSTFLAGS -C link-arg=-Wl,-dynamic-linker=/lib/ld-linux-armhf.so.3 -C linker=arm-arcadia-linux-gnueabi-gcc" cargo build --target=armv7-arcadia-linux-gnueabihf
```

# Qemu Build

```
. "$HOME/qemux86-64_sdk/environment-setup-core2-64-arcadia-linux"
ln -s /etc/ssl "$HOME/qemux86-64_sdk/sysroots/x86_64-arcadiasdk-linux/etc/ssl"
ln -s "$HOME/qemux86-64_sdk/sysroots/x86_64-arcadiasdk-linux/usr/lib/libclang-cpp.so.15" "$HOME/qemux86-64_sdk/sysroots/x86_64-arcadiasdk-linux/usr/lib/libclang.so.15"
cp "$HOME/git/guster32/scripts/.build/buildArcadiaDevQemu/build/tmp-glibc/sysroots-components/x86_64-nativesdk/nativesdk-clang/usr/local/oe-sdk-hardcoded-buildpath/sysroots/x86_64-arcadiasdk-linux/usr/lib/libclang.so.15.0.7" "$HOME/qemux86-64_sdk/sysroots/x86_64-arcadiasdk-linux/usr/lib/"

export CLANG_PATH="$HOME/qemux86-64_sdk/sysroots/x86_64-arcadiasdk-linux/usr/bin/clang"
export LIBCLANG_PATH="$HOME/qemux86-64_sdk/sysroots/x86_64-arcadiasdk-linux/usr/lib/"
export LLVM_CONFIG_PATH="$HOME/qemux86-64_sdk/sysroots/x86_64-arcadiasdk-linux/usr/bin/llvm-config"
export BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$HOME/qemux86-64_sdk/sysroots/core2-64-arcadia-linux/"
RUSTFLAGS="$RUSTFLAGS -C link-arg=-Wl,-dynamic-linker=/lib/ld-linux-x86-64.so.2 -C linker=x86_64-arcadia-linux-gcc" cargo build --target=x86_64-unknown-linux-gnu

```