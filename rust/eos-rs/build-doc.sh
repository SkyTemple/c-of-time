#!/usr/bin/env bash
# Generates the documentation at target/armv5te-none-ndseoseabi-na/doc using `cargo doc` -> `rustdoc`.
# The NA version is used as a target, but generally this applies to all regions (though some `ffi` functions
# may be unavailable in some regions).
cargo rustdoc \
  --package eos-rs \
  -Zbuild-std=core,alloc \
  --target ../armv5te-none-ndseoseabi-na.json \
  --target-dir ./target \
  --all-features \
  -- \
  --cfg docsrs \
