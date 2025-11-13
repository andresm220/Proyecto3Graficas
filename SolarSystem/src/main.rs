mod math;
mod draw;
mod window;
mod projector;
mod camera;
mod skybox;
mod hud;
mod bodies;
mod orbits;
mod shading;

use std::time::{Instant, Duration};

use crate::window::{WindowCtx, Key};
use crate::projector::Projector;
use crate::camera::Camera;
use crate::skybox::{make_stars, draw_stars};
use crate::hud::reticle;
use crate::bodies::body::Body;
use crate::orbits::draw_orbit_3d;
use crate::draw::{BG, rgb, draw_disc};
use crate::shading::{PlanetKind, draw_shaded_sphere};

fn resolve_collisions(cam: &mut Camera, sun: &Body, planets: &[Body], t: f32) {
    // evita que la cámara entre al sol y planetas
    let mut push_out = |center: crate::math::Vec3, radius: f32| {
        let diff = cam.pos.sub(center);
        let dist = diff.len();
        if dist <= 0.0001 {
            return;
        }
        // factor para no quedar pegado tan cerca
        let min_dist = radius * 1.3;
        if dist < min_dist {
            let dir = diff.mul(1.0 / dist);
            cam.pos = center.add(dir.mul(min_dist));
        }
    };

    // Sol (con radio algo más grande, porque es enorme visualmente)
    push_out(sun.pos(t), sun.radius * 2.0);

    // Planetas
    for p in planets {
        push_out(p.pos(t), p.radius * 1.5);
    }
}

fn main() {
    // Ventana relativamente ligera
    let mut win = WindowCtx::new(800, 480, "Solar 3D — Sistema con Shaders");
    let mut buf = vec![BG; win.width * win.height];

    // Cámara y proyector
    let mut cam = Camera::new();
    let proj = Projector::new(win.width, win.height, 60.0, 0.1, 5000.0);

    // Skybox
    let stars = make_stars(1200, 0xC0FFEE);

    let start = Instant::now();

    // ----------------- Sistema: Sol + 4 planetas -----------------

    let sun = Body {
        name: "Sol",
        radius: 28.0,
        orbit_r: 0.0,
        orbit_speed: 0.0,
        rot_speed: 0.2,
        phase: 0.0,
        color: rgb(255, 210, 60),
        draw_orbit: false,
    };

    let planets = vec![
        Body {
            name: "Aurea",   // rocoso
            radius: 9.0,
            orbit_r: 130.0,
            orbit_speed: 0.7,
            rot_speed: 1.3,
            phase: 0.0,
            color: rgb(255, 180, 120),
            draw_orbit: true,
        },
        Body {
            name: "Cobalt",  // gigante gaseoso
            radius: 14.0,
            orbit_r: 220.0,
            orbit_speed: 0.45,
            rot_speed: 1.0,
            phase: 1.1,
            color: rgb(100, 160, 255),
            draw_orbit: true,
        },
        Body {
            name: "Verdia",  // helado
            radius: 12.0,
            orbit_r: 310.0,
            orbit_speed: 0.28,
            rot_speed: 0.9,
            phase: 2.1,
            color: rgb(120, 255, 255),
            draw_orbit: true,
        },
        Body {
            name: "Crimson", // volcánico
            radius: 13.0,
            orbit_r: 410.0,
            orbit_speed: 0.19,
            rot_speed: 0.5,
            phase: -1.3,
            color: rgb(255, 90, 90),
            draw_orbit: true,
        },
    ];

    // Tipo de shader para cada cuerpo
    let sun_kind = PlanetKind::Star;
    let planet_kinds = vec![
        PlanetKind::Rocky,     // Aurea
        PlanetKind::GasGiant,  // Cobalt
        PlanetKind::Ice,       // Verdia
        PlanetKind::Volcanic,  // Crimson
    ];

    // modo bonito vs modo rápido (P)
    let mut pretty_mode = true;

    #[derive(Clone)]
    struct DrawItem {
        z: f32,
        sx: i32,
        sy: i32,
        r_px: i32,
        kind: PlanetKind,
    }

    // ------------------------- Loop principal ---------------------------
    while win.is_open() {
        let dt = 0.016;
        let t = start.elapsed().as_secs_f32();

        // Targets para warp 1..5 (Sol + planetas)
        let mut warp_targets = Vec::new();
        warp_targets.push(sun.pos(t));
        for p in &planets {
            warp_targets.push(p.pos(t));
        }

        // Movimiento normal (W,S adelante/atrás; A,D strafe; Space/Ctrl subir/bajar)
        cam.handle_input(&win, dt);

        // Toggles
        if win.key_pressed(Key::T) {
            cam.warp_anim_enabled = !cam.warp_anim_enabled;
        }
        if win.key_pressed(Key::P) {
            pretty_mode = !pretty_mode;
        }

        // Warp a cuerpos con 1–5
        cam.handle_warp_keys(&win, &warp_targets);

        // Animación de warp
        cam.update_warp(dt);

        // Colisiones cámara / cuerpos
        resolve_collisions(&mut cam, &sun, &planets, t);

        // Limpiar buffer
        buf.fill(BG);

        // Skybox solo en modo bonito (para ahorrar CPU en modo rápido)
        if pretty_mode {
            draw_stars(&mut buf, &cam, &proj, &stars, win.width, win.height);
        }

        // Órbitas 
        for p in &planets {
            if p.draw_orbit {
                draw_orbit_3d(
                    &mut buf,
                    &cam,
                    &proj,
                    p.orbit_r,
                    win.width,
                    win.height,
                    rgb(40, 40, 70),
                );
            }
        }

        // Proyección de Sol + planetas
        let mut items: Vec<DrawItem> = Vec::new();

        // Sol
        {
            let ws = sun.pos(t);
            let cp = Projector::world_to_camera(ws, &cam);
            if cp.z > proj.z_near && cp.z < proj.z_far {
                if let Some((sx, sy)) = proj.project(cp) {
                    let rpx = proj.radius_world_to_px(sun.radius, cp.z);
                    items.push(DrawItem {
                        z: cp.z,
                        sx,
                        sy,
                        r_px: rpx,
                        kind: sun_kind,
                    });
                }
            }
        }

        // Planetas
        for (i, p) in planets.iter().enumerate() {
            let ws = p.pos(t);
            let cp = Projector::world_to_camera(ws, &cam);
            if cp.z > proj.z_near && cp.z < proj.z_far {
                if let Some((sx, sy)) = proj.project(cp) {
                    let rpx = proj.radius_world_to_px(p.radius, cp.z);
                    items.push(DrawItem {
                        z: cp.z,
                        sx,
                        sy,
                        r_px: rpx,
                        kind: planet_kinds[i],
                    });
                }
            }
        }

        // Painter’s: de lejos a cerca
        items.sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap());

        // Dibujar según modo
        for it in items {
            if pretty_mode {
                // modo bonito: shader por píxel
                draw_shaded_sphere(
                    &mut buf,
                    win.width,
                    win.height,
                    it.sx,
                    it.sy,
                    it.r_px,
                    it.kind,
                    t,
                );
            } else {
                // modo rápido: solo discos de color plano
                let base_color = match it.kind {
                    PlanetKind::Star     => rgb(255, 230, 150),
                    PlanetKind::Rocky    => rgb(170, 140, 110),
                    PlanetKind::GasGiant => rgb(190, 170, 200),
                    PlanetKind::Ice      => rgb(180, 220, 255),
                    PlanetKind::Volcanic => rgb(200, 80, 40),
                };
                draw_disc(
                    &mut buf,
                    win.width,
                    win.height,
                    it.sx,
                    it.sy,
                    it.r_px,
                    base_color,
                );
            }
        }

        // HUD
        reticle(&mut buf, win.width, win.height);

        // Presentar frame
        win.present(&buf);

        std::thread::sleep(Duration::from_millis(16));
    }
}
