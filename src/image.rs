use std::ptr;
use ffi::core::*;
use ffi::highgui::*;
use ffi::imgproc::*;
use ffi::types::{CvArr, CvPoint, CvRect, CvSize, IplImage};
use core::{Color, Point, Rect, Size, as_c_str};

use std::path::Path;

pub struct Image {
  pub raw: *const IplImage,
  pub is_owned: bool,
}

impl Image {
  pub fn load(path: &Path) -> Result<Image, String> {
    let path_c_str = as_c_str(path);
    unsafe {
      match cvLoadImage(path_c_str.as_ptr(), 1) { // CV_LOAD_IMAGE_COLOR
        p if !p.is_null() => Ok(Image { raw: p, is_owned: true }),
        _ => Err(String::from_utf8_lossy(path_c_str.to_bytes()).into_owned()),
      }
    }
  }

  pub fn save(&self, path: &Path) -> bool {
    let path_c_str = as_c_str(path);
    unsafe {
      cvSaveImage(path_c_str.as_ptr(), self.raw, ptr::null()) == 0
    }
  }

  pub fn size(&self) -> Size {
    unsafe {
      let size = cvGetSize(self.raw as *const CvArr);
      Size { width: size.width as i32, height: size.height as i32 }
    }
  }

  pub fn width(&self) -> i32 { self.size().width }
  pub fn height(&self) -> i32 { self.size().height }

  pub fn add_line(&mut self, p1: &Point, p2: &Point, color: &Color, thickness: u32) {
    let p1 = CvPoint { x: p1.x as i32, y: p1.y as i32 };
    let p2 = CvPoint { x: p2.x as i32, y: p2.y as i32 };
    unsafe {
      cvLine(self.raw as *const CvArr, p1, p2, color.as_scalar(), thickness as i32, 16, 0); // CV_AA
    }
  }

  pub fn add_rectangle(&mut self, p1: &Point, p2: &Point, color: &Color, thickness: u32) {
    let p1 = CvPoint { x: p1.x as i32, y: p1.y as i32 };
    let p2 = CvPoint { x: p2.x as i32, y: p2.y as i32 };
    unsafe {
      cvRectangle(self.raw as *const CvArr, p1, p2, color.as_scalar(), thickness as i32, 16, 0); // CV_AA
    }
  }

  pub fn add_rectangle_r(&mut self, rect: &Rect, color: &Color, thickness: u32) {
    let rect = CvRect { x: rect.x as i32, y: rect.y as i32, width: rect.width as i32, height: rect.height as i32 };
    unsafe {
      cvRectangleR(self.raw as *const CvArr, rect, color.as_scalar(), thickness as i32, 16, 0); // CV_AA
    }
  }

  pub fn add_circle(&mut self, center: &Point, radius: u32, color: &Color, thickness: u32) {
    let center = CvPoint { x: center.x as i32, y: center.y as i32 };
    unsafe {
      cvCircle(self.raw as *const CvArr, center, radius as i32, color.as_scalar(), thickness as i32, 16, 0); // CV_AA
    }
  }

  pub fn add_ellipse(&mut self, center: &Point, axes: &Size, angle: f64, start_angle: f64, end_angle: f64, color: &Color, thickness: u32) {
    let center = CvPoint { x: center.x as i32, y: center.y as i32 };
    let axes = CvSize { width: axes.width as i32, height: axes.height as i32 };
    unsafe {
      cvEllipse(self.raw as *const CvArr, center, axes, angle, start_angle, end_angle, color.as_scalar(), thickness as i32, 16, 0); // CV_AA
    }
  }

  pub fn add_filled_convex_polygon(&mut self, points: &[&Point], color: &Color) {
    let count = points.len();
    let points =
      points.iter()
      .map(|p| {
        CvPoint { x: p.x as i32, y: p.y as i32 }
      })
      .collect::<Vec<CvPoint>>().as_ptr();
    unsafe {
      cvFillConvexPoly(self.raw as *const CvArr, points, count as i32, color.as_scalar(), 16, 0); // CV_AA
    }
  }

  pub fn add_filled_polygons(&mut self, polygons: &[&[&Point]], contours: u32, color: &Color) {
    let counts = polygons.iter().map(|ps| ps.len() as i32).collect::<Vec<i32>>().as_ptr();
    let polygons =
      polygons.iter()
      .map(|ps| {
        ps.iter().map(|p| CvPoint { x: p.x as i32, y: p.y as i32 }).collect::<Vec<CvPoint>>().as_ptr()
      })
      .collect::<Vec<*const CvPoint>>().as_ptr();
    unsafe {
      cvFillPoly(self.raw as *const CvArr, polygons, counts, contours as i32, color.as_scalar(), 16, 0); // CV_AA
    }
  }
}

impl Clone for Image {
  fn clone(&self) -> Image {
    unsafe { Image { raw: cvCloneImage(self.raw), is_owned: true } }
  }
}

impl Drop for Image {
  fn drop(&mut self) {
    if self.is_owned { unsafe { cvReleaseImage(&self.raw); } }
  }
}
