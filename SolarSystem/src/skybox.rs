use crate::math::Vec3;
use crate::camera::Camera;
use crate::projector::Projector;
use crate::draw::{put_px, rgb};

fn rng(seed: &mut u32) -> u32 {
    let mut x = *seed;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    *seed = x;
    x
}

pub fn make_stars(count: usize, seed0: u32) -> Vec<Vec3> {
    let mut s = seed0.max(1);
    let mut stars = Vec::with_capacity(count);
    for _ in 0..count {
        let u = (rng(&mut s) as f32 / u32::MAX as f32).clamp(0.0, 1.0);
        let v = (rng(&mut s) as f32 / u32::MAX as f32).clamp(0.0, 1.0);
        let theta = 2.0 * std::f32::consts::PI * u;
        let z = 2.0 * v - 1.0;
        let r = (1.0 - z * z).sqrt();
        stars.push(Vec3::new(r * theta.cos(), z, r * theta.sin()));
    }
    stars
}

pub fn draw_stars(
    buf: &mut [u32],
    cam: &Camera,
    proj: &Projector,
    dirs: &[Vec3],
    w: usize,
    h: usize,
) {
    let (right, up, fwd) = cam.basis();
    let far = 4000.0;

    for d in dirs {
        let x = d.dot(right);
        let y = d.dot(up);
        let z = d.dot(fwd);
        if z <= 0.0 {
            continue;
        }

        let cam_p = Vec3::new(x * far, y * far, z * far);
        if let Some((sx, sy)) = proj.project(cam_p) {
            // color claro para que se vean
            let c = rgb(230, 230, 255);
            // pequeño "cross" en vez de un solo pixel
            put_px(buf, w, h, sx, sy, c);
            put_px(buf, w, h, sx + 1, sy, c);
            put_px(buf, w, h, sx - 1, sy, c);
            put_px(buf, w, h, sx, sy + 1, c);
            put_px(buf, w, h, sx, sy - 1, c);
        }
    }
}
