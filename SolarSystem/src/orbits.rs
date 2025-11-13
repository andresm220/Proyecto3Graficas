use crate::math::Vec3;
use crate::draw::draw_line;
use crate::projector::Projector;
use crate::camera::Camera;

pub fn draw_orbit_3d(
    buf: &mut [u32],
    cam: &Camera,
    proj: &Projector,
    radius: f32,
    w: usize,
    h: usize,
    color: u32,
) {
    let segs = 200;
    let mut prev: Option<(i32, i32)> = None;

    for i in 0..=segs {
        let a = (i as f32 / segs as f32) * std::f32::consts::PI * 2.0;
        let p = Vec3::new(radius * a.cos(), 0.0, radius * a.sin());
        let cp = Projector::world_to_camera(p, cam);

        if cp.z <= proj.z_near || cp.z >= proj.z_far {
            prev = None;
            continue;
        }

        if let Some((sx, sy)) = proj.project(cp) {
            if let Some(pr) = prev {
                draw_line(buf, w, h, pr.0, pr.1, sx, sy, color);
            }
            prev = Some((sx,sy));
        } else {
            prev = None;
        }
    }
}
