#!/bin/sh

curl -L https://www.libsdl.org/release/SDL2-2.0.9.tar.gz | tar xz
cd SDL2-2.0.9
CFLAGS="-fPIC" ./configure --enable-shared --enable-static
make
make install
cd ..
