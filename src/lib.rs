#![no_std]
#![allow(unused_imports)]

use fermium;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowID(u32);

mod event;
