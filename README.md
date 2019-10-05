[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
[![AppVeyor](https://ci.appveyor.com/api/projects/status/4m1hm02p9kjf1dyc/branch/master?svg=true)](https://ci.appveyor.com/project/Lokathor/beryllium/branch/master)
[![TravisCI](https://travis-ci.org/Lokathor/beryllium.svg?branch=master)](https://travis-ci.org/Lokathor/beryllium)
[![crates.io](https://img.shields.io/crates/v/beryllium.svg)](https://crates.io/crates/beryllium)
[![docs.rs](https://docs.rs/beryllium/badge.svg)](https://docs.rs/beryllium/)

# beryllium
An opinionated set of "high level" wrappers for the
[fermium](https://docs.rs/fermium) SDL2 bindings.

Compared to the usual [sdl2](https://docs.rs/sdl2), this is much closer to the
actual SDL2 C API. It is, however, a lot less complete. If you want something
added in just file and issue and we can probably work to get it done fairly
easily. If you don't want to wait on me, just call into `fermium` directly.

There are [examples](/examples/) available if you want to see it in action.

## Building

Obviously this uses SDL2. The [fermium](https://docs.rs/fermium) crate is used
as the bindings crate. You will need version 2.0.8 or later:

* On Windows all necessary files are provided automatically by `fermium`, you
  don't need to do _any_ special setup at all.
* On non-Windows you need to have installed SDL2 yourself ahead of time.

### Window Subsystem

This isn't SDL2 specific, it's just a general Win32 thing, but you'll probably
want to set the
[window_subsystem](https://doc.rust-lang.org/reference/attributes.html#crate-only-attributes)
to "windows" in your programs. Just add this to the top of any binary or
example:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
```

We only want it enabled when `debug_assertions` are _not_ active, because it
suppresses all default console output when it is enabled.

This line won't have any effect on the build outside Windows, so no worries
about needing to check for the build being on windows.

## License

This crate uses the Zlib license, the same license that SDL2 itself uses.

## Nifty Project Logo

![99% beryllium](https://upload.wikimedia.org/wikipedia/commons/0/0c/Be-140g.jpg)
