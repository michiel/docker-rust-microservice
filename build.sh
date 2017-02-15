#!/bin/sh

cargo build --target x86_64-unknown-linux-musl --release 
docker build -t echo-rs .
