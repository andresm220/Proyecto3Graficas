use crate::math::Vec3;

#[derive(Clone)]
pub struct Moon {
    pub name: &'static str,
    pub radius: f32,
    pub orbit_r: f32,
    pub orbit_speed: f32,
    pub phase: f32,
    pub color: u32,
    pub parent_idx: usize, // índice del planeta padre en el vector de planetas
}

impl Moon {
    pub fn pos(&self, t: f32, parent_world: Vec3) -> Vec3 {
        let a = self.phase + t * self.orbit_speed;
        parent_world.add(Vec3::new(self.orbit_r * a.cos(), 0.0, self.orbit_r * a.sin()))
    }
}
