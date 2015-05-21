#![crate_name = "opencv"]
#![crate_type = "lib"]

#![deny(unused_imports)]

#[macro_use] extern crate enum_primitive;
extern crate num;

extern crate libc;

pub mod core;
pub mod highgui;
pub mod image;
pub mod objdetect;
pub mod video;
mod ffi;
