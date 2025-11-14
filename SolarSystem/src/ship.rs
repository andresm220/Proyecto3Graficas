use crate::math::Vec3;
use crate::camera::Camera;
use crate::projector::Projector;
use crate::draw::draw_line;
use crate::obj_loader::Mesh;
use crate::draw::rgb;


pub struct Ship {
    mesh: Mesh,
    scale: f32,
    forward_offset: f32,
    down_offset: f32,
    wire_color: u32,
    face_stride: usize,
}

impl Ship {
    pub fn new() -> Self {
        const OBJ_DATA: &str = include_str!("../nave_andres.obj");
        let mesh = Mesh::from_obj_str(OBJ_DATA);

        Self {
            mesh,
            scale: 3.0,
            forward_offset: 60.0,
            down_offset: 9.0,
            wire_color: rgb(220, 230, 255),
            face_stride: 5, // solo dibuja 1 de cada 5 caras
        }
    }

    pub fn draw(
        &self,
        buf: &mut [u32],
        w: usize,
        h: usize,
        cam: &Camera,
        proj: &Projector,
    ) {
        let n_verts = self.mesh.verts.len();
        if n_verts == 0 {
            return;
        }

        let (right, up, fwd) = cam.basis();
        let center = cam
            .pos
            .add(fwd.mul(self.forward_offset))
            .add(up.mul(-self.down_offset));

        let mut cam_verts = vec![Vec3::new(0.0, 0.0, 0.0); n_verts];
        let mut screen_verts = vec![None; n_verts];

        for (i, v) in self.mesh.verts.iter().enumerate() {
            let ws = center
                .add(right.mul(v.x * self.scale))
                .add(up.mul(v.y * self.scale))
                .add(fwd.mul(v.z * self.scale));

            let cp = Projector::world_to_camera(ws, cam);
            cam_verts[i] = cp;

            if cp.z <= proj.z_near || cp.z >= proj.z_far {
                continue;
            }

            if let Some((sx, sy)) = proj.project(cp) {
                screen_verts[i] = Some((sx, sy));
            }
        }

        for (face_idx, face) in self.mesh.faces.iter().enumerate() {
            if face_idx % self.face_stride != 0 {
                continue;
            }

            let [i0, i1, i2] = *face;

            let v0 = cam_verts[i0];
            let v1 = cam_verts[i1];
            let v2 = cam_verts[i2];

            let n = v1.sub(v0).cross(v2.sub(v0)).norm();
            if n.z >= 0.0 {
                continue;
            }

            let (Some((x0, y0)), Some((x1, y1)), Some((x2, y2))) =
                (screen_verts[i0], screen_verts[i1], screen_verts[i2])
            else {
                continue;
            };

            draw_line(buf, w, h, x0, y0, x1, y1, self.wire_color);
            draw_line(buf, w, h, x1, y1, x2, y2, self.wire_color);
            draw_line(buf, w, h, x2, y2, x0, y0, self.wire_color);
        }
    }
}
