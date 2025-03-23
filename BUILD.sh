#!/bin/bash
cd /home/whitepi/mtv-tv-leptos/

git pull;
trunk build --release;

arch=$(uname -m)
if [ "$arch" = "aarch64" ]; then
    docker build -t mtvmovslep:arm64 .
    docker run -d -p 9092:80 mtvmovslep:arm64
fi

if [ "$arch" = "arm32v7" ]; then
    docker build -t mtvmovslep:arm32 .
    docker run -d -p 9092:80 mtvmovslep:arm32
fi