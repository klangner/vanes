#!/usr/bin/env bash
cargo build --target armv7-unknown-linux-gnueabihf
rsync target/armv7-unknown-linux-gnueabihf/debug/vanes root@192.168.7.2:~/vanes
ssh root@192.168.7.2 ./vanes
ssh root@192.168.7.2 pkill -9 vanes