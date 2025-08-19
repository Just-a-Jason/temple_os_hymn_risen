use std::ptr;
use x11::xlib;

/// Linux implementation of HolyWindow
#[derive(Debug)]
pub struct InnerWindow {
    display: *mut xlib::Display,
    window: xlib::Window,
    width: u32,
    height: u32,
    pixmap: xlib::Pixmap,
}

impl InnerWindow {
    pub fn new(width: u32, height: u32, title: &str, resizable: bool) -> Self {
        unsafe {
            // Tries to connect to x11 server and returs multable raw pointer to Display enum.
            // *mut Display
            let display: *mut xlib::Display = xlib::XOpenDisplay(ptr::null());

            // If *mut Display is null it means that it failed to connect with x11 server in this case I would like to panic.
            if display.is_null() {
                panic!("HolyWindow::InnerWindow: Linux window cannot be initialized");
            }

            // XDefaultScreen(_1: *mut _XDisplay) takes display rawpointer and returns
            let screen = xlib::XDefaultScreen(display);

            let window = xlib::XCreateSimpleWindow(
                display,
                xlib::XRootWindow(display, screen),
                0,
                0,
                width,
                height,
                0,
                0,
                0,
            );

            let c_title = std::ffi::CString::new(title).unwrap();
            xlib::XStoreName(display, window, c_title.as_ptr());

            let w_mask = xlib::KeyPressMask | xlib::ExposureMask | xlib::StructureNotifyMask;

            // c_long = i64
            // XSelectInput(display: *mut _XDisplay, window: window_id, mask: i64)
            xlib::XSelectInput(display, window, w_mask);

            xlib::XMapWindow(display, window);

            if !resizable {
                let mut hints: xlib::XSizeHints = std::mem::zeroed();
                hints.flags = xlib::PMinSize | xlib::PMaxSize;

                let width_i32 = width as i32;
                let height_i32 = height as i32;

                // Set max width & height as width and hight provided.
                hints.min_width = width_i32;
                hints.min_height = height_i32;

                hints.max_width = width_i32;
                hints.max_height = height_i32;

                xlib::XSetNormalHints(display, window, &mut hints);
            }

            xlib::XFlush(display);

            let pixmap = xlib::XCreatePixmap(
                display,
                window,
                width,
                height,
                xlib::XDefaultDepth(display, screen) as u32,
            );

            InnerWindow {
                display,
                window,
                width,
                height,
                pixmap,
            }
        }
    }
}

impl InnerWindow {
    pub fn set_title(&self, title: &str) {
        unsafe {
            let c_title = std::ffi::CString::new(title).unwrap();
            xlib::XStoreName(self.display, self.window, c_title.as_ptr());
            xlib::XFlush(self.display);
        }
    }
}

impl InnerWindow {
    pub unsafe fn update(&mut self) -> bool {
        unsafe {
            let mut event: xlib::XEvent = std::mem::zeroed();

            while xlib::XPending(self.display) > 0 {
                xlib::XNextEvent(self.display, &mut event);

                // Close the linux based window when Escape key is pressed
                if event.get_type() == xlib::KeyPress {
                    if xlib::XLookupKeysym(&mut event.key, 0) == x11::keysym::XK_Escape.into() {
                        xlib::XDestroyWindow(self.display, self.window);
                        xlib::XCloseDisplay(self.display);
                        return false;
                    }
                }
            }

            xlib::XFlush(self.display);
            true
        }
    }

    pub unsafe fn clear(&self) {
        unsafe {
            let gc = xlib::XCreateGC(self.display, self.pixmap, 0, std::ptr::null_mut());

            xlib::XSetForeground(self.display, gc, 0);
            xlib::XFillRectangle(self.display, self.pixmap, gc, 0, 0, self.width, self.height);

            xlib::XFreeGC(self.display, gc);
        }
    }

    pub unsafe fn draw_image_at(&self, image: &mut crate::HolyImage, position: crate::HolyVector2) {
        unsafe {
            let gc = xlib::XCreateGC(self.display, self.pixmap, 0, std::ptr::null_mut());

            let sprite = image.sprite();

            let ximage = xlib::XCreateImage(
                self.display,
                xlib::XDefaultVisual(self.display, xlib::XDefaultScreen(self.display)),
                xlib::XDefaultDepth(self.display, xlib::XDefaultScreen(self.display)) as u32,
                xlib::ZPixmap,
                0,
                sprite.raw_ptr(),
                sprite.width(),
                sprite.height(),
                32,
                0,
            );

            /*
               docs: https://tronche.com/gui/x/xlib/graphics/XPutImage.html
               XPutImage(display, d, gc, image, src_x, src_y, dest_x, dest_y, width, height)
               Display *display;
               Drawable d;
               GC gc;
               XImage *image;
               int src_x, src_y;
               int dest_x, dest_y;
               unsigned int width, height;
            */

            xlib::XPutImage(
                self.display,
                self.pixmap,
                gc,
                ximage,
                0,
                0,
                // dest_x: c_int (i32)
                position.x - sprite.width() as i32 / 2,
                // dest_y: c_int (i32)
                position.y - sprite.height() as i32 / 2,
                sprite.width(),
                sprite.height(),
            );

            xlib::XFreeGC(self.display, gc);
        }
    }

    pub unsafe fn render_frame(&self) {
        unsafe {
            let gc = xlib::XCreateGC(self.display, self.window, 0, std::ptr::null_mut());

            xlib::XCopyArea(
                self.display,
                self.pixmap,
                self.window,
                gc,
                0,
                0,
                self.width,
                self.height,
                0,
                0,
            );

            xlib::XFreeGC(self.display, gc);
            xlib::XFlush(self.display);
        }
    }
}
