use crate::math::Vec3;
use crate::camera::Camera;

pub struct Projector {
    pub width: usize,
    pub height: usize,
    pub fov_y_deg: f32,
    pub z_near: f32,
    pub z_far: f32,
    fy: f32,
}

impl Projector {
    pub fn new(width: usize, height: usize, fov_y_deg: f32, z_near: f32, z_far: f32) -> Self {
        let fy = (height as f32 * 0.5) / (0.5 * fov_y_deg.to_radians()).tan();
        Self { width, height, fov_y_deg, z_near, z_far, fy }
    }

    pub fn world_to_camera(p: Vec3, cam: &Camera) -> Vec3 {
        let (right, up, fwd) = cam.basis();
        let rel = p.sub(cam.pos);
        Vec3::new(rel.dot(right), rel.dot(up), rel.dot(fwd))
    }

    pub fn project(&self, cam_p: Vec3) -> Option<(i32, i32)> {
        if cam_p.z < self.z_near || cam_p.z > self.z_far { return None; }
        let sx = (cam_p.x * self.fy) / cam_p.z;
        let sy = (cam_p.y * self.fy) / cam_p.z;
        let x = (sx + (self.width as f32)*0.5) as i32;
        let y = ((self.height as f32)*0.5 - sy) as i32;
        Some((x,y))
    }

    pub fn radius_world_to_px(&self, radius_world: f32, z: f32) -> i32 {
        if z <= self.z_near { return 0; }
        ((self.fy * radius_world) / z).max(1.0) as i32
    }
}
