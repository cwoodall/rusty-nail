#!/usr/bin/bash

for i in examples/*; do
  cargo build --target=arm-unknown-linux-gnueabihf --verbose --example $(basename $i .rs);
done
