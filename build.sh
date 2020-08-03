#!/bin/sh

rustup target add "${TARGET}"

# Download the Raspberry Pi cross-compilation toolchain if needed
if [ "${TARGET}" = "arm-unknown-linux-gnueabihf" ]
then
  git clone --depth=1 https://github.com/raspberrypi/tools.git /tmp/tools;
  export PATH=/tmp/tools/arm-bcm2708/arm-linux-gnueabihf/bin:${PATH};
  export CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc
fi

cargo build --target="${TARGET}" --release

ls -al ./target/"${TARGET}"/release/

if [ "${TARGET}" = "x86_64-pc-windows-gnu" ]
then
  export OSCAR_NAME="oscar.exe"
  mv ./target/"${TARGET}"/release/oscar "${OSCAR_NAME}"
fi

if [ "${TARGET}" != "x86_64-pc-windows-gnu" ]
then
  export OSCAR_NAME="oscar-${TARGET}"
  mv ./target/"${TARGET}"/release/oscar "${OSCAR_NAME}"
fi

ls -al .
