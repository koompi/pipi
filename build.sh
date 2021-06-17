#!/bin/sh

cargo build --release

sudo install -Dm755 target/release/pipi /usr/bin/pipi


echo "Finished install pipi"
