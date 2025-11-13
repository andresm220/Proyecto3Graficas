pub const BG: u32 = 0x000000;

#[inline]
pub fn rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

#[inline]
pub fn put_px(buf: &mut [u32], w: usize, h: usize, x: i32, y: i32, c: u32) {
    if x>=0 && y>=0 && (x as usize) < w && (y as usize) < h {
        buf[y as usize * w + x as usize] = c;
    }
}

pub fn draw_line(buf: &mut [u32], w: usize, h: usize, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
    let (mut x0, mut y0, mut x1, mut y1) = (x0,y0,x1,y1);
    let dx = (x1 - x0).abs(); let sx = if x0 < x1 {1} else {-1};
    let dy = -(y1 - y0).abs(); let sy = if y0 < y1 {1} else {-1};
    let mut err = dx + dy;
    loop {
        put_px(buf, w, h, x0, y0, color);
        if x0==x1 && y0==y1 { break; }
        let e2 = 2*err;
        if e2 >= dy { err += dy; x0 += sx; }
        if e2 <= dx { err += dx; y0 += sy; }
    }
}

pub fn draw_disc(buf: &mut [u32], w: usize, h: usize, cx: i32, cy: i32, r: i32, color: u32) {
    if r <= 0 { return; }
    let r2 = r*r;
    for dy in -r..=r {
        let yy = cy + dy;
        let wspan = (r2 - dy*dy).max(0) as f32;
        let wspan = wspan.sqrt() as i32;
        for dx in -wspan..=wspan {
            put_px(buf, w, h, cx + dx, yy, color);
        }
    }
}
