use crate::math::Vec3;
use crate::draw::{put_px, rgb};

#[derive(Clone, Copy)]
pub enum PlanetKind {
    Star,
    Rocky,
    GasGiant,
    Ice,
    Volcanic,
}

fn clamp(x: f32, a: f32, b: f32) -> f32 { x.max(a).min(b) }
fn mix(a: f32, b: f32, t: f32) -> f32 { a * (1.0 - t) + b * t }
fn mix3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    Vec3::new(mix(a.x, b.x, t), mix(a.y, b.y, t), mix(a.z, b.z, t))
}

// ---------------- Shaders muy baratos -----------------

fn shade_star(local: Vec3, n: Vec3, view: Vec3, tsec: f32) -> Vec3 {
    let lat = local.y;
    let lon = local.z.atan2(local.x);

    let base = Vec3::new(1.0, 0.85, 0.45);

    // “granulación” simple
    let gran = (lat * 10.0 + lon * 6.0 + tsec * 4.0).sin() * 0.5 + 0.5;
    let gran_color = mix3(base.mul(0.8), base.mul(1.2), gran);

    let pulse = 0.9 + 0.1 * (tsec * 3.0).sin();

    let mu = clamp(n.dot(view.mul(-1.0)), 0.0, 1.0);
    let limb = mix(0.5, 1.0, mu);

    gran_color.mul(pulse * limb)
}

fn shade_rocky(local: Vec3, n: Vec3, _view: Vec3, _tsec: f32) -> Vec3 {
    let lat = local.y;
    let lon = local.z.atan2(local.x);

    let rock_base = Vec3::new(0.4, 0.3, 0.22);
    let rock_dark = Vec3::new(0.22, 0.15, 0.12);

    let h = (lat * 4.0 + lon * 3.0).sin() * 0.5 + 0.5;
    let cont = mix3(rock_dark, rock_base, h);

    let m = (lat * 9.0 + lon * 11.0).cos() * 0.5 + 0.5;
    let mountains = mix3(cont, cont.mul(1.4), m * 0.6);

    let cap = clamp((lat.abs() - 0.6) / (1.0 - 0.6), 0.0, 1.0);
    let ice = mix3(mountains, Vec3::new(0.9, 0.92, 0.95), cap * 0.8);

    let light = Vec3::new(0.6, 0.8, 0.7).norm();
    let lambert = 0.4 + 0.6 * clamp(n.dot(light), 0.0, 1.0);
    ice.mul(lambert)
}

fn shade_gas_giant(local: Vec3, n: Vec3, _view: Vec3, tsec: f32) -> Vec3 {
    let lat = local.y;

    let band_freq = 10.0;
    let band = (band_freq * lat + tsec * 0.8).sin() * 0.5 + 0.5;

    let col1 = Vec3::new(0.9, 0.85, 0.8);
    let col2 = Vec3::new(0.7, 0.55, 0.4);
    let base = mix3(col1, col2, band);

    let t_small = (lat * 25.0 + tsec * 1.5).sin() * 0.5 + 0.5;
    let with_turb = mix3(base.mul(0.9), base.mul(1.1), t_small);

    let light = Vec3::new(-0.3, 0.8, 0.5).norm();
    let lambert = 0.35 + 0.65 * clamp(n.dot(light), 0.0, 1.0);
    with_turb.mul(lambert)
}

fn shade_ice(local: Vec3, n: Vec3, _view: Vec3, _tsec: f32) -> Vec3 {
    let lat = local.y;

    let base = Vec3::new(0.75, 0.85, 0.95);
    let deep = Vec3::new(0.3, 0.5, 0.8);

    let t = (lat * 4.0).sin() * 0.5 + 0.5;
    let ice = mix3(deep, base, t);

    let light = Vec3::new(0.2, 0.8, 1.0).norm();
    let lambert = 0.4 + 0.6 * clamp(n.dot(light), 0.0, 1.0);
    ice.mul(lambert)
}

fn shade_volcanic(local: Vec3, n: Vec3, _view: Vec3, tsec: f32) -> Vec3 {
    let lat = local.y;
    let lon = local.z.atan2(local.x);

    let rock = Vec3::new(0.12, 0.08, 0.08);
    let lava_hot = Vec3::new(1.2, 0.5, 0.1);

    // zonas de lava cerca del ecuador
    let belt = (lat * 6.0).cos().max(0.0);
    let streaks = ((lon * 8.0) + tsec * 2.0).sin() * 0.5 + 0.5;
    let mask = (belt * streaks).powf(2.0);

    let col = mix3(rock, lava_hot, mask);

    let light = Vec3::new(0.5, 0.6, 0.9).norm();
    let lambert = 0.3 + 0.7 * clamp(n.dot(light), 0.0, 1.0);
    col.mul(lambert)
}

// ---------------- Render esfera desde disco 2D --------------------

pub fn draw_shaded_sphere(
    buf: &mut [u32],
    w: usize,
    h: usize,
    cx: i32,
    cy: i32,
    radius_px: i32,
    kind: PlanetKind,
    tsec: f32,
) {
    if radius_px <= 0 { return; }

    let view_dir = Vec3::new(0.0, 0.0, -1.0);
    let r = radius_px;
    let r2 = (r * r) as f32;

    for dy in -r..=r {
        let yy = cy + dy;
        for dx in -r..=r {
            let xx = cx + dx;

            let fx = dx as f32;
            let fy = dy as f32;
            let d2 = fx * fx + fy * fy;
            if d2 > r2 { continue; }

            let nx = fx / r as f32;
            let ny = fy / r as f32;
            let nz = (1.0 - (nx * nx + ny * ny)).sqrt();
            let normal = Vec3::new(nx, ny, nz).norm();
            let local = normal;

            let col = match kind {
                PlanetKind::Star     => shade_star(local, normal, view_dir, tsec),
                PlanetKind::Rocky    => shade_rocky(local, normal, view_dir, tsec),
                PlanetKind::GasGiant => shade_gas_giant(local, normal, view_dir, tsec),
                PlanetKind::Ice      => shade_ice(local, normal, view_dir, tsec),
                PlanetKind::Volcanic => shade_volcanic(local, normal, view_dir, tsec),
            };

            let r8 = (clamp(col.x, 0.0, 1.0) * 255.0) as u8;
            let g8 = (clamp(col.y, 0.0, 1.0) * 255.0) as u8;
            let b8 = (clamp(col.z, 0.0, 1.0) * 255.0) as u8;
            put_px(buf, w, h, xx, yy, rgb(r8, g8, b8));
        }
    }
}
