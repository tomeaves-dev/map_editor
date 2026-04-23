use glam::{Mat4, Vec3, Vec4};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

pub struct RayHit {
    pub distance: f32,
}

impl Ray {
    pub fn from_screen(
        mouse_pos: egui::Pos2,
        rect: egui::Rect,
        view: Mat4,
        proj: Mat4,
    ) -> Self {
        // Step 1 - convert to NDC
        let ndc_x = ((mouse_pos.x - rect.min.x) / rect.width())  * 2.0 - 1.0;
        let ndc_y = -(((mouse_pos.y - rect.min.y) / rect.height()) * 2.0 - 1.0);

        // Step 2 - clip space to view space
        let inv_proj = proj.inverse();
        let clip = Vec4::new(ndc_x, ndc_y, -1.0, 1.0);
        let view_space = inv_proj * clip;
        let view_dir = Vec4::new(view_space.x, view_space.y, -1.0, 0.0);

        // Step 3 - view space to world space
        let inv_view = view.inverse();
        let world = inv_view * view_dir;
        let direction = Vec3::new(world.x, world.y, world.z).normalize();

        // Origin is the camera position - extract from inverse view matrix
        let origin = Vec3::new(
            inv_view.col(3).x,
            inv_view.col(3).y,
            inv_view.col(3).z,
        );

        Self { origin, direction }
    }

    pub fn intersect_brush(&self, planes: &[map_format::brush::Plane]) -> Option<RayHit> {
        let mut t_min = f32::NEG_INFINITY;
        let mut t_max = f32::INFINITY;

        for plane in planes {
            let normal = glam::Vec3::new(
                plane.normal.x,
                plane.normal.y,
                plane.normal.z,
            );
            let denom = normal.dot(self.direction);
            let t = (plane.distance - normal.dot(self.origin)) / denom;

            if denom < 0.0 {
                // Ray entering this half-space
                t_min = t_min.max(t);
            } else if denom > 0.0 {
                // Ray exiting this half-space
                t_max = t_max.min(t);
            }

            if t_min > t_max {
                return None;
            }
        }

        if t_max < 0.0 {
            return None; // brush is behind the ray
        }

        let distance = if t_min >= 0.0 { t_min } else { t_max };
        Some(RayHit { distance })
    }
}