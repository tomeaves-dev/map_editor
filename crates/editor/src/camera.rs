use glam::{Mat4, Vec2, Vec3};

const MOMENTUM_DECAY: f32 = 8.0;
const SCROLL_BASE: f32 = 1.5;
const MIN_SPEED: f32 = 1.0;
const MAX_SPEED: f32 = 500.0;
pub const SCROLL_STEPS: f32 = 20.0;

pub struct Camera {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub move_speed: f32,
    pub pan_velocity: Vec2,
    pub scroll_step: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vec3::new(7.0, 5.0, 7.0),
            yaw: -std::f32::consts::FRAC_PI_4, // 45 degrees
            pitch: -0.5,
            move_speed: 5.0,
            pan_velocity: Vec2::ZERO,
            scroll_step: 5.0,
        }
    }

    pub fn horizontal_forward(&self) -> Vec3 {
        Vec3::new(
            self.yaw.sin(),
            0.0,
            -self.yaw.cos(),
        ).normalize()
    }

    pub fn forward(&self) -> Vec3 {
        Vec3::new(
            self.yaw.sin() * self.pitch.cos(),
            self.pitch.sin(),
            -self.yaw.cos() * self.pitch.cos(),
        ).normalize()
    }

    pub fn right(&self) -> Vec3 {
        self.horizontal_forward().cross(Vec3::Y).normalize()
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

    pub fn speed_normalised(&self) -> f32 {
        self.scroll_step / SCROLL_STEPS
    }

    fn update_speed_from_scroll(&mut self, scroll_delta: f32) {
        self.scroll_step = (self.scroll_step + scroll_delta)
            .clamp(0.0, SCROLL_STEPS);

        // Maps 0..SCROLL_STEPS to MIN_SPEED..MAX_SPEED exponentially
        let t = self.scroll_step / SCROLL_STEPS;
        self.move_speed = MIN_SPEED * (MAX_SPEED / MIN_SPEED).powf(t);
    }

    pub fn update(&mut self, ctx: &egui::Context, response: &egui::Response, dt: f32) {
        let sensitivity = 0.005;

        // Right click: FPS look
        if response.dragged_by(egui::PointerButton::Secondary)
            || ctx.input(|i| i.pointer.button_down(egui::PointerButton::Secondary))
            && response.hovered()
        {
            let delta = ctx.input(|i| i.pointer.delta());
            self.yaw   += delta.x * sensitivity;
            self.pitch -= delta.y * sensitivity;
            self.pitch = self.pitch.clamp(
                -std::f32::consts::FRAC_PI_2 + 0.01,
                std::f32::consts::FRAC_PI_2 - 0.01,
            );

            // WASD fly mode - only active during right click
            let forward = self.horizontal_forward();
            let right = self.right();
            let speed = self.move_speed * dt;

            ctx.input(|i| {
                if i.key_down(egui::Key::W) { self.position += forward * speed; }
                if i.key_down(egui::Key::S) { self.position -= forward * speed; }
                if i.key_down(egui::Key::A) { self.position -= right   * speed; }
                if i.key_down(egui::Key::D) { self.position += right   * speed; }
                if i.key_down(egui::Key::E) { self.position.y += speed; }
                if i.key_down(egui::Key::Q) { self.position.y -= speed; }
            });
        }

        // Left click: pan with momentum
        if response.dragged_by(egui::PointerButton::Primary) {
            let delta = response.drag_delta();
            let pan_scale = self.move_speed * 0.01;

            let forward = self.horizontal_forward();
            let right = self.right();
            self.position += right   * -delta.x * pan_scale;
            self.position += forward *  delta.y * pan_scale;

            self.pan_velocity = Vec2::new(
                -delta.x * pan_scale,
                delta.y * pan_scale,
            );
        } else {
            // Momentum after release
            let forward = self.horizontal_forward();
            let right = self.right();

            self.position += right   * self.pan_velocity.x;
            self.position += forward * self.pan_velocity.y;

            self.pan_velocity *= (1.0 - MOMENTUM_DECAY * dt).max(0.0);

            if self.pan_velocity.length() < 0.001 {
                self.pan_velocity = Vec2::ZERO;
            }
        }

        // Scroll: adjust speed when hovering viewport
        if response.hovered() {
            let scroll = ctx.input(|i| i.smooth_scroll_delta.y);
            if scroll != 0.0 {
                self.update_speed_from_scroll(scroll * 0.01);
            }
        }

        ctx.request_repaint();
    }
}