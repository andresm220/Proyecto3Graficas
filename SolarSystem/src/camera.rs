use crate::math::Vec3;
use crate::window::{WindowCtx, Key};

pub struct Camera {
    pub pos: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    speed: f32,
    lift_speed: f32,
    turn_speed_yaw: f32,
    turn_speed_pitch: f32,
    // Warp
    warp_target: Option<Vec3>,
    warp_t: f32,
    pub warp_anim_enabled: bool,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: Vec3::new(0.0, 40.0, 220.0),
            yaw: -0.2,
            pitch: -0.1,
            speed: 120.0,
            lift_speed: 90.0,
            turn_speed_yaw: 1.2,
            turn_speed_pitch: 0.9,
            warp_target: None,
            warp_t: 0.0,
            warp_anim_enabled: true,
        }
    }

    pub fn basis(&self) -> (Vec3, Vec3, Vec3) {
        let cy = self.yaw.cos(); let sy = self.yaw.sin();
        let cp = self.pitch.cos(); let sp = self.pitch.sin();
        let fwd = Vec3::new(sy*cp, sp, -cy*cp).norm();
        let up = Vec3::new(0.0, 1.0, 0.0);
        let right = fwd.cross(up).mul(-1.0).norm();
        let up2 = right.cross(fwd).norm();
        (right, up2, fwd)
    }

    /// Movimiento normal: WASD + Space/Ctrl + flechas
    pub fn handle_input(&mut self, win: &WindowCtx, dt: f32) {
        let (right, up, fwd) = self.basis();
        let mut move_dir = Vec3::new(0.0,0.0,0.0);

        if win.key_down(Key::W) { move_dir = move_dir.add(fwd.mul(self.speed*dt)); }
        if win.key_down(Key::S) { move_dir = move_dir.add(fwd.mul(-self.speed*dt)); }
        if win.key_down(Key::A) { move_dir = move_dir.add(right.mul(-self.speed*dt)); }
        if win.key_down(Key::D) { move_dir = move_dir.add(right.mul(self.speed*dt)); }
        if win.key_down(Key::Space) { move_dir = move_dir.add(up.mul(self.lift_speed*dt)); }
        if win.key_down(Key::LeftCtrl) { move_dir = move_dir.add(up.mul(-self.lift_speed*dt)); }

        if win.key_down(Key::Left)  { self.yaw   -= self.turn_speed_yaw * dt; }
        if win.key_down(Key::Right) { self.yaw   += self.turn_speed_yaw * dt; }
        if win.key_down(Key::Up)    { self.pitch += self.turn_speed_pitch * dt; }
        if win.key_down(Key::Down)  { self.pitch -= self.turn_speed_pitch * dt; }

        self.pitch = self.pitch.clamp(-1.2, 1.2);
        self.pos = self.pos.add(move_dir);
    }

    /// Maneja teclas 1–5 para hacer warp al Sol/planetas
    pub fn handle_warp_keys(&mut self, win: &WindowCtx, targets: &[Vec3]) {
        let keys = [Key::Key1, Key::Key2, Key::Key3, Key::Key4, Key::Key5];

        for (idx, key) in keys.iter().enumerate() {
            if idx >= targets.len() { break; }
            if win.key_pressed(*key) {
                let target = targets[idx];

                // Posición bonita: un poco arriba y atrás del planeta, mirando hacia él
                let (_, _, fwd) = self.basis();
                let offset_up = Vec3::new(0.0, 30.0, 0.0);
                let offset_back = fwd.mul(-140.0);
                let dest = target.add(offset_up).add(offset_back);

                if self.warp_anim_enabled {
                    self.warp_target = Some(dest);
                    self.warp_t = 0.0;
                } else {
                    self.pos = dest;
                    self.warp_target = None;
                    self.warp_t = 0.0;
                }
            }
        }
    }

    /// Actualiza la animación del warp (lerp hacia el target)
    pub fn update_warp(&mut self, dt: f32) {
        if let Some(target) = self.warp_target {
            let duration = 0.5; // segundos
            self.warp_t += dt / duration;
            let t = self.warp_t.min(1.0);
            self.pos = Vec3::lerp(self.pos, target, t);
            if self.warp_t >= 1.0 {
                self.warp_target = None;
            }
        }
    }
}
