#!/bin/sh

sudo apt-get update
sudo apt-get install lld clang -y
cargo install --locked trunk