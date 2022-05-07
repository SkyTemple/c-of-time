#!/usr/bin/env bash
# Generates the bindings for the pmdsky-debug headers.
# Note that rust-bindgen needs to be in the path and pmdsky-debug needs to be checkout out.

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

bindgen \
  --raw-line '//! This module contains the "raw" functions and types from the game. They are generatd by bindgen. It is generally recommended to try and use [`crate::api`] instead.' \
  --raw-line "//!" \
  --raw-line "//! For the documentation, see the symbol YAML files of pmdsky-debug at <https://github.com/UsernameFodder/pmdsky-debug/tree/master/symbols>." \
  --raw-line "#![allow(non_upper_case_globals)]" \
  --raw-line "#![allow(non_camel_case_types)]" \
  --raw-line "#![allow(non_snake_case)]" \
  --ctypes-prefix "crate::ctypes" \
  --use-core \
  --no-layout-tests \
  --no-derive-copy \
  --no-derive-debug \
  --default-enum-style moduleconsts \
  $SCRIPT_DIR/include.h \
  -- \
  -target armv5te-none-eabi \
  > $SCRIPT_DIR/src/ffi.rs
