mod app;
mod viewport;
mod camera;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };
    eframe::run_native(
        "Map Editor",
        options,
        Box::new(|cc| Ok(Box::new(app::MapEditorApp::new(cc)))),
    ).unwrap();
}