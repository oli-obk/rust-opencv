#![crate_name = "opencv"]
#![crate_type = "lib"]


#![feature(globs, unsafe_destructor)]
#![deny(unused_imports)]

extern crate libc;

pub mod core;
pub mod highgui;
pub mod image;
pub mod objdetect;
pub mod video;
mod ffi;
