use ffi::core::*;
use ffi::types::{CvSeq, CvRect};

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
  pub x: int,
  pub y: int,
}

impl Point {
  pub fn new(x: int, y: int) -> Point {
    Point { x: x, y: y }
  }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Rect {
  pub x: int,
  pub y: int,
  pub width: int,
  pub height: int,
}

impl Rect {
  pub fn new(x: int, y: int, width: int, height: int) -> Rect {
    Rect { x: x, y: y, width: width, height: height }
  }
}

pub type Scalar = [f64;4];

pub struct Seq {
  pub raw: *mut CvSeq,
  pub curr: uint,
}

impl Iterator for Seq {
  type Item = Rect;
  fn next(&mut self) -> Option<Rect> {
    unsafe {
      if self.curr.lt(&self.len()) {
        match cvGetSeqElem(&*self.raw, self.curr as int) {
          c if !c.is_null() => {
            let ref rect = *(c as *mut CvRect);
            self.curr += 1;
            Some(Rect::new(rect.x as int, rect.y as int, rect.width as int, rect.height as int))
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
  fn len(&self) -> uint {
    unsafe {
      let total = (*self.raw).total;
      if total.gt(&0) { total as uint } else { 0u }
    }
  }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Size {
  pub width: int,
  pub height: int,
}

impl Size {
  pub fn new(width: int, height: int) -> Size {
    Size { width: width, height: height }
  }
}
