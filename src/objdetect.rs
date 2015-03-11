use ffi::types::{CvArr, CvHaarClassifierCascade, CvSize};
use std::ptr;
use core::{Size, Seq};
use image::{Image};
use ffi::objdetect::*;
use ffi::core::*;

use std::path::Path;
use std::ffi::AsOsStr;
use std::os::unix::prelude::OsStrExt;


pub struct CascadeClassifier {
  raw: *mut CvHaarClassifierCascade
}

impl CascadeClassifier {
  
  pub fn load(path: &Path) -> Result<CascadeClassifier, String> {
    let path_c_str = path.as_os_str().to_cstring().unwrap();
    unsafe {
      match cvLoad(path_c_str.as_ptr(), ptr::null_mut(), ptr::null(), ptr::null()) {
        c if !c.is_null() => Ok(CascadeClassifier { raw: c as *mut CvHaarClassifierCascade }),
        _ => Err(String::from_utf8_lossy(path_c_str.to_bytes()).into_owned()),
      }
    }
  }

  pub fn detect_multi_scale(&self, image: &Image, 
    scale_factor: f64, min_neighbors: int, flags: int, 
    min_size: Size, max_size: Size) -> Result<Seq, String> {

    unsafe {
      match cvHaarDetectObjects(
        image.raw as *const CvArr, 
        self.raw,
        cvCreateMemStorage(0),
        scale_factor,
        min_neighbors as i32,
        flags as i32,
        CvSize { width: min_size.width as i32, height: min_size.height as i32 },
        CvSize { width: max_size.width as i32, height: max_size.height as i32 }
      ) {
        r if !r.is_null() => Ok(Seq { raw: r, curr: 0u }),
        _ => Err("Something went wrong!".to_string())
      }
    }

  }

}
