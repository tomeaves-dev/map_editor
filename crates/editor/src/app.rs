use eframe::egui_wgpu;
use crate::viewport::ViewportRenderer;
use crate::camera::Camera;

pub struct MapEditorApp {
    camera: Camera,
}

impl MapEditorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        let _renderer = cc.wgpu_render_state.as_ref().map(|rs| {
            let renderer = ViewportRenderer::new(&rs.device, rs.target_format);
            rs.renderer.write().callback_resources.insert(renderer);
        });

        Self {
            camera: Camera::new(),
        }
    }
}

impl eframe::App for MapEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let dt = ctx.input(|i| i.unstable_dt);
        self.camera.update(ctx, dt);
        ctx.request_repaint();

        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(200.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label("Tools");
                });
            });

        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(200.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label("Properties");
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let (rect, _response) = ui.allocate_exact_size(
                ui.available_size(),
                egui::Sense::focusable_noninteractive(),
            );

            let view = self.camera.view_matrix();
            let proj = self.camera.projection_matrix(rect.width() / rect.height());
            let view_proj = proj * view;

            let callback = egui_wgpu::Callback::new_paint_callback(
                rect,
                crate::viewport::ViewportCallback { view_proj },
            );

            ui.painter().add(callback);
        });
    }
}