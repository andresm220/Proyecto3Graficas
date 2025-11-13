use crate::draw::{put_px, rgb};

pub fn reticle(buf: &mut [u32], w: usize, h: usize) {
    let cx = (w as i32) / 2;
    let cy = (h as i32) / 2;
    for dx in -6..=6 { put_px(buf, w, h, cx + dx, cy, rgb(0x88,0x99,0xAA)); }
    for dy in -6..=6 { put_px(buf, w, h, cx, cy + dy, rgb(0x88,0x99,0xAA)); }
}
