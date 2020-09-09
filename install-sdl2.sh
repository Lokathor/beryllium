#!/bin/sh

curl -L https://www.libsdl.org/release/SDL2-2.0.12.tar.gz | tar xz
cd SDL2-2.0.12
CFLAGS="-fPIC" ./configure --enable-shared --enable-static
make
make install
cd ..
