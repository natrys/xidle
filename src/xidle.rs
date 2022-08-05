use x11::xlib;
use x11::xss;
use x11::xss::XScreenSaverInfo;

use std::mem::zeroed;
use std::ptr::null;
use std::time::Duration;


pub struct XIdleService {
    display: *mut xlib::Display,
    root: xlib::Drawable,
}

impl XIdleService {
    pub fn new() -> XIdleService {
        let (display, root) = unsafe {
            let display = xlib::XOpenDisplay(null());
            if display.is_null() {
                panic!("can't open display");
            };
            let root = xlib::XRootWindow(display, xlib::XDefaultScreen(display));
            (display, root)
        };

        XIdleService {
            display,
            root,
        }
    }

    pub fn query(&self) -> XScreenSaverInfo {
        unsafe {
            let mut info = zeroed();
            xss::XScreenSaverQueryInfo(self.display, self.root, &mut info);
            info
        }
    }

    pub fn idle(&self) -> Duration {
        Duration::from_millis(self.query().idle)
    }
}

impl Drop for XIdleService {
    fn drop(&mut self) {
        unsafe { xlib::XCloseDisplay(self.display) };
    }
}
