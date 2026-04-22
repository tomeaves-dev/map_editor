mod app;
mod viewport;
mod camera;
mod terrain;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 960.0]),
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };
    eframe::run_native(
        "Map Editor",
        options,
        Box::new(|cc| Ok(Box::new(app::MapEditorApp::new(cc)))),
    ).unwrap();
}