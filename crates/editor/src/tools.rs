use egui::Response;

pub enum ToolKind {
    Select,
    Translate,
    Create,
}

pub struct ToolContext<'a> {
    pub response: &'a Response,
    pub camera: &'a crate::camera::Camera,
    pub dt: f32,
}

pub trait Tool {
    fn on_pointer_down(&mut self, ctx: &ToolContext);
    fn on_pointer_up(&mut self, ctx: &ToolContext);
    fn on_pointer_move(&mut self, ctx: &ToolContext);
    fn draw_overlay(&self, painter: &egui::Painter);
}

pub struct SelectTool;
pub struct TranslateTool;
pub struct CreateTool;

impl Tool for SelectTool {
    fn on_pointer_down(&mut self, _ctx: &ToolContext) {}
    fn on_pointer_up(&mut self, _ctx: &ToolContext) {}
    fn on_pointer_move(&mut self, _ctx: &ToolContext) {}
    fn draw_overlay(&self, _painter: &egui::Painter) {}
}

impl Tool for TranslateTool {
    fn on_pointer_down(&mut self, _ctx: &ToolContext) {}
    fn on_pointer_up(&mut self, _ctx: &ToolContext) {}
    fn on_pointer_move(&mut self, _ctx: &ToolContext) {}
    fn draw_overlay(&self, _painter: &egui::Painter) {}
}

impl Tool for CreateTool {
    fn on_pointer_down(&mut self, _ctx: &ToolContext) {}
    fn on_pointer_up(&mut self, _ctx: &ToolContext) {}
    fn on_pointer_move(&mut self, _ctx: &ToolContext) {}
    fn draw_overlay(&self, _painter: &egui::Painter) {}
}