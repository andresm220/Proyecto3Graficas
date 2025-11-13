use crate::math::Vec3;

#[derive(Clone)]
pub struct Body {
    pub name: &'static str,
    pub radius: f32,       // radio del planeta
    pub orbit_r: f32,      // radio orbital (distancia al centro)
    pub orbit_speed: f32,  // velocidad angular
    pub rot_speed: f32,    // rotación decorativa
    pub phase: f32,        // fase inicial
    pub color: u32,
    pub draw_orbit: bool,
}

impl Body {
    pub fn pos(&self, t: f32) -> Vec3 {
        if self.orbit_r == 0.0 {
            return Vec3::new(0.0, 0.0, 0.0); // Sol
        }
        let a = self.phase + t * self.orbit_speed;
        Vec3::new(self.orbit_r * a.cos(), 0.0, self.orbit_r * a.sin())
    }
}
