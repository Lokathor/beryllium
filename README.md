[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
[![AppVeyor](https://ci.appveyor.com/api/projects/status/4m1hm02p9kjf1dyc/branch/master?svg=true)](https://ci.appveyor.com/project/Lokathor/beryllium/branch/master)
[![TravisCI](https://travis-ci.org/Lokathor/beryllium.svg?branch=master)](https://travis-ci.org/Lokathor/beryllium)
[![crates.io](https://img.shields.io/crates/v/beryllium.svg)](https://crates.io/crates/beryllium)
[![docs.rs](https://docs.rs/beryllium/badge.svg)](https://docs.rs/beryllium/)

# THIS CRATE IS REVOKED AND BANNED YOU CANNOT USE IT BECAUSE C IS TERRIBLE

please have a good day

# beryllium
An opinionated set of "high level" wrappers for the
[fermium](https://github.com/Lokathor/fermium) SDL2 bindings.

There are [examples](/examples/) available if you want to see it in action.

## Mindset

* Correctness is evaluated at compile time through lifetime tracking. Many
  people don't like having said lifetimes because it makes it hard to put stuff
  together into the same struct. I'm trying to keep it to a minimum without
  being unsafe. If you'd like to take the wheel yourself you're invited to. The
  [glium example](/examples/extern_crate_glium.rs) has some code that
  (carefully) uses a bunch of transmutes to keep lifetime trouble to a minimum
  while also mixing several things up into one big struct. Going forward, I'll
  try to improve in this area as much as possible.
* Sometimes you just gotta write some `unsafe` code, and that's all there is to
  it. I can't cover up every dangerous possibility for you. I don't really
  consider this a major drawback, but you might think differently. I still try
  to keep the `unsafe` to a minimum of course.

## Building

Obviously this uses SDL2. The [fermium](https://docs.rs/fermium) crate is used
as the bindings crate. You will need version 2.0.9 or later:

* On Windows all necessary files are provided automatically by `fermium`, you
  don't need to do _any_ special setup at all.
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
* In both cases, the default linking is a static linking, so your program won't
  need to also ship any developer files once the compilation is completed.

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

### Contribution

**PLEASE DO NOT PR TO MASTER**

All new development goes to a
[dev](https://github.com/Lokathor/beryllium/tree/dev) branch. Updates are only
transferred to the `master` branch when it's time to do a new release to
crates.io.

Some call it odd, but I just like having the `master` branch always reflect the
latest available on crates.io whenever possible to keep confusion to a minimum
when jumping between the two websites.
