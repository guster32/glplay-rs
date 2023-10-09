. /home/guster32/odroidxu4_sdk/environment-setup-cortexa15t2hf-neon-vfpv4-arcadia-linux-gnueabi
ln -s /etc/ssl /home/guster32/odroidxu4_sdk/sysroots/x86_64-arcadiasdk-linux/etc/ssl
ln -s /home/guster32/odroidxu4_sdk/sysroots/x86_64-arcadiasdk-linux/usr/lib/libclang-cpp.so.15 /home/guster32/odroidxu4_sdk/sysroots/x86_64-arcadiasdk-linux/usr/lib/libclang.so.15
cp /home/guster32/git/guster32/scripts/.build/buildArcadiaDevOdroidxu4/build/tmp-glibc/sysroots-components/x86_64-nativesdk/nativesdk-clang/usr/local/oe-sdk-hardcoded-buildpath/sysroots/x86_64-arcadiasdk-linux/usr/lib/libclang.so.15.0.7 /home/guster32/odroidxu4_sdk/sysroots/x86_64-arcadiasdk-linux/usr/lib/

export CLANG_PATH=/home/guster32/odroidxu4_sdk/sysroots/x86_64-arcadiasdk-linux/usr/bin/clang
export LIBCLANG_PATH=/home/guster32/odroidxu4_sdk/sysroots/x86_64-arcadiasdk-linux/usr/lib/
export LLVM_CONFIG_PATH=/home/guster32/odroidxu4_sdk/sysroots/x86_64-arcadiasdk-linux/usr/bin/llvm-config
export BINDGEN_EXTRA_CLANG_ARGS="--sysroot=/home/guster32/odroidxu4_sdk/sysroots/cortexa15t2hf-neon-vfpv4-arcadia-linux-gnueabi/"
RUSTFLAGS="$RUSTFLAGS -C link-arg=-Wl,-dynamic-linker=/lib/ld-linux-armhf.so.3 -C linker=arm-arcadia-linux-gnueabi-gcc" cargo build --target=armv7-arcadia-linux-gnueabihf
