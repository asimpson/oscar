#!/bin/sh

rustup target add "${TARGET}"

# Download the Raspberry Pi cross-compilation toolchain if needed
if [ "${TARGET}" = "arm-unknown-linux-gnueabihf" ]
then
  git clone --depth=1 https://github.com/raspberrypi/tools.git /tmp/tools;
  export PATH=/tmp/tools/arm-bcm2708/arm-linux-gnueabihf/bin:${PATH};
fi

cargo build --target="${TARGET}" --release

mv ./target/"${TARGET}"/release/oscar "${OSCAR_NAME}"
