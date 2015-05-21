use ffi::core::*;
use ffi::types::{CvSeq, CvRect};


use std::path::Path;
use std::ffi::CString;

pub fn as_c_str(path: &Path) -> CString {
    CString::new(path.as_os_str().to_str().unwrap()).unwrap()
}

#[derive(Clone, PartialEq, Debug)]
pub struct Color {
  red: u8,
  green: u8,
  blue: u8,
  alpha: u8,
}

impl Color {
  pub fn from_rgb(red: u8, green: u8, blue: u8) -> Color {
    Color { red: red, green: green, blue: blue, alpha: 0 }
  }

  pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
    Color { red: red, green: green, blue: blue, alpha: alpha }
  }

  pub fn as_scalar(&self) -> Scalar {
    [self.blue as f64, self.green as f64, self.red as f64, self.alpha as f64]
  }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

impl Point {
  pub fn new(x: i32, y: i32) -> Point {
    Point { x: x, y: y }
  }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Rect {
  pub x: i32,
  pub y: i32,
  pub width: i32,
  pub height: i32,
}

impl Rect {
  pub fn new(x: i32, y: i32, width: i32, height: i32) -> Rect {
    Rect { x: x, y: y, width: width, height: height }
  }
}

pub type Scalar = [f64;4];

pub struct Seq {
  pub raw: *mut CvSeq,
  pub curr: usize,
}

impl Iterator for Seq {
  type Item = Rect;
  fn next(&mut self) -> Option<Rect> {
    unsafe {
      if self.curr.lt(&self.len()) {
        match cvGetSeqElem(&*self.raw, self.curr as i32) {
          c if !c.is_null() => {
            let ref rect = *(c as *mut CvRect);
            self.curr += 1;
            Some(Rect::new(rect.x as i32, rect.y as i32, rect.width as i32, rect.height as i32))
          },
          _ => None
        }
      } else {
        None
      }
    }
  }
}

impl Seq {
  fn len(&self) -> usize {
    unsafe {
      let total = (*self.raw).total;
      if total.gt(&0) { total as usize } else { 0 }
    }
  }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Size {
  pub width: i32,
  pub height: i32,
}

impl Size {
  pub fn new(width: i32, height: i32) -> Size {
    Size { width: width, height: height }
  }
}
