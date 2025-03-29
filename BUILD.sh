#!/bin/bash

git pull;

trunk clean;

trunk build --release;

arch=$(uname -m)

if [ "$arch" = "aarch64" ]; then
    docker build -t mtvmovslep:arm64 -f ./arm64/Dockerfile .
    docker run -d -p 9092:80 mtvmovslep:arm64
fi

if [ "$arch" = "arm32v7" ]; then
    docker build -t mtvmovslep:arm32 -f ./arm32/Dockerfile .
    docker run -d -p 9092:80 mtvmovslep:arm32
fi

if [ "$arch" = "x86_64" ]; then
    docker build -t mtvmovslep:amd64 -f ./amd64/Dockerfile .
    docker run -d -p 9092:80 mtvmovslep:amd64
fi