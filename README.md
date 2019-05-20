[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
[![AppVeyor](https://ci.appveyor.com/api/projects/status/4m1hm02p9kjf1dyc?svg=true)](https://ci.appveyor.com/project/Lokathor/beryllium)
[![TravisCI](https://travis-ci.org/Lokathor/beryllium.svg?branch=master)](https://travis-ci.org/Lokathor/beryllium)
[![crates.io](https://img.shields.io/crates/v/beryllium.svg)](https://crates.io/crates/beryllium)
[![docs.rs](https://docs.rs/beryllium/badge.svg)](https://docs.rs/beryllium/)

# beryllium
An opinionated set of "high level" wrappers for the
[fermium](https://github.com/Lokathor/fermium) SDL2 bindings.

crate squatted, more to come soon!

## Mindset

I attempt to make things as safe as is _reasonable_, without actually doing
anything that would hurt the API design.

## Building

Obviously this uses SDL2. You need version 2.0.9 or later:

* On Windows the necessary files are provided automatically, and you don't need
  to do any special setup at all.
* On non-Windows you need to have installed SDL2 yourself ahead of time:
  * You'll need the `-fPIC` flag enabled in your SDL2 install! This is necessary
    because the build will static link to SDL2 by default.
  * On macOS you should simply [use
    homebrew](https://formulae.brew.sh/formula/sdl2), it does everything
    correctly with no fuss.
  * On Linux you can use the [installer script](install-sdl2.sh) in this repo.
    Either run it as is with `sudo` for a default install (to `/usr/local/lib`)
    or adjust it to fit your needs. Linux programmer are all pros, right?

## License

This crate uses the Zlib license, the same license that SDL2 itself uses.
