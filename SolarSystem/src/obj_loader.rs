use crate::math::Vec3;

pub struct Mesh {
    pub verts: Vec<Vec3>,
    pub faces: Vec<[usize; 3]>,
}

impl Mesh {
    pub fn from_obj_str(src: &str) -> Self {
        let mut verts = Vec::new();
        let mut faces = Vec::new();

        for line in src.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if line.starts_with("v ") && !line.starts_with("vn") && !line.starts_with("vt") {
                // v x y z
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let x = parts[1].parse::<f32>().unwrap_or(0.0);
                    let y = parts[2].parse::<f32>().unwrap_or(0.0);
                    let z = parts[3].parse::<f32>().unwrap_or(0.0);
                    verts.push(Vec3::new(x, y, z));
                }
            } else if line.starts_with("f ") {
                // f a/b/c d/e/f g/h/i
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let idx = |s: &str| -> Option<usize> {
                        let first = s.split('/').next().unwrap_or("");
                        if first.is_empty() {
                            None
                        } else {
                            // OBJ es 1-based -> nosotros usamos 0-based
                            first.parse::<usize>().ok().map(|i| i.saturating_sub(1))
                        }
                    };

                    if let (Some(i0), Some(i1), Some(i2)) =
                        (idx(parts[1]), idx(parts[2]), idx(parts[3]))
                    {
                        faces.push([i0, i1, i2]);
                    }
                }
            }
        }

        Mesh { verts, faces }
    }
}
