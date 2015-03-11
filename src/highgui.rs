use std::{mem, num, ptr};
use libc::{c_int, c_void};
use ffi::highgui::*;
use image::Image;
use std::ffi::CString;

pub fn wait_key(delay: int) -> int {
  unsafe { cvWaitKey(delay as i32) as int }
}

pub struct Trackbar {
  name: String,
  window: String,
  on_change: Option<fn(uint)>,
}

impl Trackbar {
  pub fn position(&self) -> uint {
    let trackbar_name = CString::new(self.name.as_bytes()).unwrap();
    let window_name = CString::new(self.window.as_bytes()).unwrap();
    unsafe {
      cvGetTrackbarPos(trackbar_name.as_ptr(), window_name.as_ptr()) as uint
    }
  }

  pub fn set_position(&mut self, position: uint) {
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

#[derive(FromPrimitive)]
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

pub struct Window {
  name: String,
  trackbars: Vec<Trackbar>,
  on_mouse: Option<fn(MouseEvent, int, int)>,
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

  pub fn move_(&self, x: int, y: int) {
    let name_c_str = CString::new(self.name.as_bytes()).unwrap();
    unsafe {
      cvMoveWindow(name_c_str.as_ptr(), x as i32, y as i32);
    }
  }

  pub fn resize(&self, width: int, height: int) {
    let name_c_str = CString::new(self.name.as_bytes()).unwrap();
    unsafe {
      cvResizeWindow(name_c_str.as_ptr(), width as i32, height as i32);
    }
  }

  pub fn create_trackbar(&mut self, name: &str, position: uint, max: uint, on_change: fn(uint)) -> Trackbar {
    extern "C" fn wrapper(pos: c_int, userdata: *const c_void) {
      let callback = unsafe { mem::transmute::<_, &mut fn(uint)>(userdata) };

      (*callback)(pos as uint);
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

  pub fn on_mouse(&mut self, on_mouse: fn(MouseEvent, int, int)) {
    extern "C" fn wrapper(event: c_int, x: c_int, y: c_int, _: c_int, param: *const c_void) {
      let event = num::from_i32::<MouseEvent>(event).unwrap();
      let callback = unsafe { mem::transmute::<_, &mut fn(MouseEvent, int, int)>(param) };

      (*callback)(event, x as int, y as int);
    }

    self.on_mouse = Some(on_mouse);
    let name_c_str = CString::new(self.name.as_bytes()).unwrap();
    unsafe {
      cvSetMouseCallback(name_c_str.as_ptr(), wrapper, mem::transmute(self.on_mouse.as_ref().unwrap()));
    }
  }
}

#[unsafe_destructor]
impl Drop for Window {
  fn drop(&mut self) {
    let name_c_str = CString::new(self.name.as_bytes()).unwrap();
    unsafe {
      cvDestroyWindow(name_c_str.as_ptr());
    };
  }
}
