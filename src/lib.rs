#![no_std]
#![allow(unused_imports)]

use fermium;

#[allow(unused_macros)]
macro_rules! submodule {
  ($v:vis $name:ident) => {
    mod $name;
    $v use $name::*;
  };
  ($v:vis $name:ident { $($content:tt)* }) => {
    mod $name { $($content)* }
    $v use $name::*;
  };
}

mod event;
