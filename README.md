[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
[![AppVeyor](https://ci.appveyor.com/api/projects/status/4m1hm02p9kjf1dyc?svg=true)](https://ci.appveyor.com/project/Lokathor/beryllium)
[![TravisCI](https://travis-ci.org/Lokathor/beryllium.svg?branch=master)](https://travis-ci.org/Lokathor/beryllium)
[![crates.io](https://img.shields.io/crates/v/beryllium.svg)](https://crates.io/crates/beryllium)
[![docs.rs](https://docs.rs/beryllium/badge.svg)](https://docs.rs/beryllium/)

# beryllium
An opinionated set of "high level" wrappers for the
[fermium](https://github.com/Lokathor/fermium) SDL2 bindings.

## Mindset

I attempt to make things as safe as is _reasonable_, without actually doing
anything that would hurt the API design.

There are [examples](/examples/) available if you want a sample of what things
look like in practice.

## Building

Obviously this uses SDL2. You need version 2.0.9 or later:

* On Windows the necessary files are provided automatically, and you don't need
  to do _any_ special setup at all.
* On non-Windows you need to have installed SDL2 yourself ahead of time:
  * You'll need the `-fPIC` flag enabled in your SDL2 install! This is necessary
    because the build will static link to SDL2 by default.
  * On macOS you should simply [use
    homebrew](https://formulae.brew.sh/formula/sdl2), it does everything
    correctly with no fuss.
  * On Linux you can use the [installer script](install-sdl2.sh) in this repo.
    Either run it as is with `sudo` for a default install (to `/usr/local/lib`)
    or adjust it to fit your needs. Linux programmer are all pros, right?
  * If you attempt to build the lib and it fails because SDL2 isn't installed
    you'll have to run `cargo clean` to make the `build.rs` work.

The `fermium` bindings do a static compilation by default, so once you build
your program it won't need to be shipped with any extra files.

### Window Subsystem

You'll probably want to set the
[window_subsystem](https://doc.rust-lang.org/reference/attributes.html#crate-only-attributes)
to "windows" in your beryllium programs. Just add this to the top of the main
file of any binary or example:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
```

Note that we only want it enabled when `debug_assertions` are not active. If
it's configured you don't have a default console _at all_ so you can't print
debug messages. We only need to set it with the version we plan to ship to
users.

This line won't have any effect on the build outside Windows, so no worries.

## License

This crate uses the Zlib license, the same license that SDL2 itself uses.
