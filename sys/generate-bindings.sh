#!/usr/bin/env bash

set -eu

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

BINDGEN_VERSION="$(bindgen --version)"
RPI_WS281X_VERSION="$(cd rpi_ws281x/; git rev-parse --verify HEAD)"

bindgen \
    --no-doc-comments \
    --use-core \
    --no-layout-tests \
    --raw-line "// Generated using $BINDGEN_VERSION" \
    --raw-line "// Generated against rpi_ws281x $RPI_WS281X_VERSION" \
    --raw-line "" \
    --raw-line "#![allow(non_camel_case_types, dead_code)]" \
    --whitelist-function "ws2811_(init|render|fini|get_return_t_str)" \
    --whitelist-var "(WS2811|SK6812)_STRIP_.*" \
    --whitelist-var "WS2811_TARGET_FREQ" \
    --whitelist-var "RPI_PWM_CHANNELS" \
    --rustified-enum "ws2811_return_t" \
    --blacklist-type "__.*" \
    -o "src/lib.rs" \
    "./ws281x.h" \
    -- -I/usr/include
