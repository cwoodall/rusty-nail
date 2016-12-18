#!/usr/bin/bash

TARGET=arm-unknown-linux-gnueabihf

for i in examples/*; do
  cargo build --target=$TARGET --verbose --example $(basename $i .rs);
done
