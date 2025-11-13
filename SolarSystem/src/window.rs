use minifb::{KeyRepeat, Scale, Window, WindowOptions};

pub struct WindowCtx {
    pub window: Window,
    pub width: usize,
    pub height: usize,
}

impl WindowCtx {
    pub fn new(width: usize, height: usize, title: &str) -> Self {
        let window = Window::new(
            title,
            width, height,
            WindowOptions { scale: Scale::X1, ..WindowOptions::default() }
        ).unwrap();
        Self { window, width, height }
    }

    pub fn is_open(&self) -> bool { self.window.is_open() }

    pub fn key_down(&self, k: Key) -> bool { self.window.is_key_down(k) }

    pub fn key_pressed(&self, k: Key) -> bool { self.window.is_key_pressed(k, KeyRepeat::No) }

    pub fn present(&mut self, buf: &[u32]) {
        self.window.update_with_buffer(buf, self.width, self.height).unwrap();
    }
}


pub use minifb::Key;
