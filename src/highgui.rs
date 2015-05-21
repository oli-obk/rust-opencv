use std::{mem, ptr};
use libc::{c_int, c_void};
use ffi::highgui::*;
use image::Image;
use std::ffi::CString;
use num::FromPrimitive;

pub fn wait_key(delay: i32) -> i32 {
  unsafe { cvWaitKey(delay) as i32 }
}

pub struct Trackbar {
  name: String,
  window: String,
  on_change: Option<fn(u32)>,
}

impl Trackbar {
  pub fn position(&self) -> u32 {
    let trackbar_name = CString::new(self.name.as_bytes()).unwrap();
    let window_name = CString::new(self.window.as_bytes()).unwrap();
    unsafe {
      cvGetTrackbarPos(trackbar_name.as_ptr(), window_name.as_ptr()) as u32
    }
  }

  pub fn set_position(&mut self, position: u32) {
    let trackbar_name = CString::new(self.name.as_bytes()).unwrap();
    let window_name = CString::new(self.window.as_bytes()).unwrap();
    unsafe {
      cvSetTrackbarPos(trackbar_name.as_ptr(), window_name.as_ptr(), position as i32);
    }
  }
}

impl Clone for Trackbar {
  fn clone(&self) -> Trackbar {
    Trackbar { name: self.name.clone(), window: self.window.clone(), on_change: None }
  }
}

enum_from_primitive! {
pub enum MouseEvent {
  MouseMove = 0,
  LeftButtonDown,
  RightButtonDown,
  MiddleButtonDown,
  LeftButtonUp,
  RightButtonUp,
  MiddleButtonUp,
  LeftButtonDoubleClick,
  RightButtonDoubleClick,
  MiddleButtonDoubleClick,
  MouseWheel,
  MouseHorizontalWheel,
}
}

pub struct Window {
  name: String,
  trackbars: Vec<Trackbar>,
  on_mouse: Option<fn(MouseEvent, i32, i32)>,
}

impl Window {
  pub fn named(name: &str) -> Window {
    let name_c_str = CString::new(name.as_bytes()).unwrap();
    unsafe {
      cvNamedWindow(name_c_str.as_ptr(), 1i32);
    };
    Window { name: name.to_string(), trackbars: Vec::new(), on_mouse: None }
  }

  pub fn show(&self, image: &Image) {
    let name_c_str = CString::new(self.name.as_bytes()).unwrap();
    unsafe {
      cvShowImage(name_c_str.as_ptr(), image.raw);
    }
  }

  pub fn move_(&self, x: u32, y: u32) {
    let name_c_str = CString::new(self.name.as_bytes()).unwrap();
    unsafe {
      cvMoveWindow(name_c_str.as_ptr(), x as i32, y as i32);
    }
  }

  pub fn resize(&self, width: u32, height: u32) {
    let name_c_str = CString::new(self.name.as_bytes()).unwrap();
    unsafe {
      cvResizeWindow(name_c_str.as_ptr(), width as i32, height as i32);
    }
  }

  pub fn create_trackbar(&mut self, name: &str, position: u32, max: u32, on_change: fn(u32)) -> Trackbar {
    extern "C" fn wrapper(pos: c_int, userdata: *const c_void) {
      let callback = unsafe { mem::transmute::<_, &mut fn(u32)>(userdata) };

      (*callback)(pos as u32);
    }

    self.trackbars.push(Trackbar { name: name.to_string(), window: self.name.clone(), on_change: Some(on_change) });
    let trackbar = self.trackbars.last().unwrap();

    let trackbar_name = CString::new(trackbar.name.as_bytes()).unwrap();
    let window_name = CString::new(trackbar.window.as_bytes()).unwrap();

    unsafe {
      cvCreateTrackbar2(trackbar_name.as_ptr(), window_name.as_ptr(), ptr::null(), max as i32, wrapper, mem::transmute(trackbar.on_change.as_ref().unwrap()));
      cvSetTrackbarPos(trackbar_name.as_ptr(), window_name.as_ptr(), position as i32);
    }

    trackbar.clone()
  }

  pub fn on_mouse(&mut self, on_mouse: fn(MouseEvent, i32, i32)) {
    extern "C" fn wrapper(event: c_int, x: c_int, y: c_int, _: c_int, param: *const c_void) {
      let event = FromPrimitive::from_i32(event).unwrap();
      let callback = unsafe { mem::transmute::<_, &mut fn(MouseEvent, i32, i32)>(param) };

      (*callback)(event, x as i32, y as i32);
    }

    self.on_mouse = Some(on_mouse);
    let name_c_str = CString::new(self.name.as_bytes()).unwrap();
    unsafe {
      cvSetMouseCallback(name_c_str.as_ptr(), wrapper, mem::transmute(self.on_mouse.as_ref().unwrap()));
    }
  }
}

impl Drop for Window {
  fn drop(&mut self) {
    let name_c_str = CString::new(self.name.as_bytes()).unwrap();
    unsafe {
      cvDestroyWindow(name_c_str.as_ptr());
    };
  }
}
