use glam::{Mat4, Vec3};
use egui;

pub struct Camera {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 5.0),
            yaw: 0.0,
            pitch: 0.0,
        }
    }

    pub fn forward(&self) -> Vec3 {
        Vec3::new(
            self.yaw.sin() * self.pitch.cos(),
            self.pitch.sin(),
            -self.yaw.cos() * self.pitch.cos(),
        ).normalize()
    }

    pub fn right(&self) -> Vec3 {
        self.forward().cross(Vec3::Y).normalize()
    }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(
            self.position,
            self.position + self.forward(),
            Vec3::Y,
        )
    }

    pub fn projection_matrix(&self, aspect: f32) -> Mat4 {
        Mat4::perspective_rh(
            std::f32::consts::FRAC_PI_4,
            aspect,
            0.1,
            1000.0,
        )
    }

    pub fn update(&mut self, ctx: &egui::Context, dt: f32) {
        let speed = 5.0 * dt;
        let sensitivity = 0.005;

        // FPS look - only when right mouse button held
        if ctx.input(|i| i.pointer.secondary_down()) {
            let delta = ctx.input(|i| i.pointer.delta());
            self.yaw   += delta.x * sensitivity;
            self.pitch -= delta.y * sensitivity;

            // Clamp pitch so you can't flip upside down
            self.pitch = self.pitch.clamp(
                -std::f32::consts::FRAC_PI_2 + 0.01,
                std::f32::consts::FRAC_PI_2 - 0.01,
            );
        }

        // WASD movement
        let forward = self.forward();
        let right = self.right();

        ctx.input(|i| {
            if i.key_down(egui::Key::W) { self.position += forward * speed; }
            if i.key_down(egui::Key::S) { self.position -= forward * speed; }
            if i.key_down(egui::Key::A) { self.position -= right * speed; }
            if i.key_down(egui::Key::D) { self.position += right * speed; }
            if i.key_down(egui::Key::E) { self.position.y += speed; }
            if i.key_down(egui::Key::Q) { self.position.y -= speed; }
        });
    }
}