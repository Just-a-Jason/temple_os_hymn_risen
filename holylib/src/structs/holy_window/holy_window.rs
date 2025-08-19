#[cfg(target_os = "linux")]
use super::linux::InnerWindow;

#[derive(Debug)]
pub struct HolyWindow {
    inner: InnerWindow,
}

impl HolyWindow {
    /// Creates a new HolyWindow
    pub fn new(width: u32, height: u32, title: &str, resizable: bool) -> Self {
        HolyWindow {
            inner: InnerWindow::new(width, height, title, resizable),
        }
    }
}

// Self functions
impl HolyWindow {
    pub fn set_title(&self, title: &str) {
        self.inner.set_title(title);
    }
}

// Unsafe functions
impl HolyWindow {
    pub unsafe fn draw_image_at(
        &mut self,
        image: &mut crate::HolyImage,
        position: crate::HolyVector2,
    ) {
        unsafe {
            self.inner.draw_image_at(image, position);
        }
    }

    pub unsafe fn update(&mut self) -> bool {
        unsafe { self.inner.update() }
    }

    pub unsafe fn clear(&self) {
        unsafe { self.inner.clear() };
    }

    pub unsafe fn render_frame(&self) {
        unsafe {
            self.inner.render_frame();
        }
    }
}
