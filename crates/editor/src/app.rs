use eframe::egui_wgpu;
use uuid::Uuid;
use map_format::brush::{Brush, Plane};
use map_format::document::MapDocument;
use map_format::types::Vec3 as MapVec3;
use crate::viewport::ViewportRenderer;
use crate::camera::Camera;
use crate::tools::ToolKind;

pub struct MapEditorApp {
    camera: Camera,
    show_texture_browser: bool,
    texture_search: String,
    active_tool: ToolKind,
    document: MapDocument,
    selected_brush: Option<Uuid>,
}

impl MapEditorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        let _renderer = cc.wgpu_render_state.as_ref().map(|rs| {
            let renderer = ViewportRenderer::new(&rs.device, rs.target_format);
            let grid = crate::grid::GridRenderer::new(&rs.device, rs.target_format);
            rs.renderer.write().callback_resources.insert(renderer);
            rs.renderer.write().callback_resources.insert(grid);
        });

        let mut document = MapDocument::new("Untitled");
        let layer = map_format::layer::Layer::new_brush("Default", false);
        let layer_id = layer.id;
        document.layers.push(layer);

        let mut brush = Brush::new(layer_id);
        brush.planes = vec![
            Plane { normal: MapVec3 { x:  1.0, y: 0.0, z: 0.0 }, distance:  1.0 },
            Plane { normal: MapVec3 { x: -1.0, y: 0.0, z: 0.0 }, distance:  1.0 },
            Plane { normal: MapVec3 { x: 0.0, y:  1.0, z: 0.0 }, distance:  1.0 },
            Plane { normal: MapVec3 { x: 0.0, y: -1.0, z: 0.0 }, distance:  1.0 },
            Plane { normal: MapVec3 { x: 0.0, y: 0.0, z:  1.0 }, distance:  1.0 },
            Plane { normal: MapVec3 { x: 0.0, y: 0.0, z: -1.0 }, distance:  1.0 },
        ];
        document.brushes.push(brush);

        Self {
            camera: Camera::new(),
            show_texture_browser: false,
            texture_search: String::new(),
            active_tool: ToolKind::Select,
            document,
            selected_brush: None,
        }
    }
}

impl eframe::App for MapEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let dt = ctx.input(|i| i.unstable_dt);

        // Menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    let _ = ui.button("New");
                    let _ = ui.button("Open");
                    let _ = ui.button("Save");
                    let _ = ui.button("Save As");
                    ui.separator();
                    let _ = ui.button("Export");
                    ui.separator();
                    let _ = ui.button("Quit");
                });
                ui.menu_button("Edit", |ui| {
                    let _ = ui.button("Undo");
                    let _ = ui.button("Redo");
                    ui.separator();
                    let _ = ui.button("Cut");
                    let _ = ui.button("Copy");
                    let _ = ui.button("Paste");
                    ui.separator();
                    let _ = ui.button("Select All");
                });
                ui.menu_button("View", |ui| {
                    let _ = ui.button("Toggle Grid");
                    let _ = ui.button("Toggle Stats");
                    ui.separator();
                    if ui.button("Texture Browser").clicked() {
                        self.show_texture_browser = !self.show_texture_browser;
                        ui.close_menu();
                    }
                });
                ui.menu_button("Help", |ui| {
                    let _ = ui.button("Documentation");
                    let _ = ui.button("About");
                });
            });
        });

        // Toolbar
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.selectable_label(
                    matches!(self.active_tool, ToolKind::Select),
                    "⬆ Select"
                ).clicked() {
                    self.active_tool = ToolKind::Select;
                }
                ui.separator();
                if ui.selectable_label(
                    matches!(self.active_tool, ToolKind::Translate),
                    "✋ Translate"
                ).clicked() {
                    self.active_tool = ToolKind::Translate;
                }
                ui.separator();
                if ui.selectable_label(
                    matches!(self.active_tool, ToolKind::Create),
                    "◻ Create"
                ).clicked() {
                    self.active_tool = ToolKind::Create;
                }
            });
        });

        // Status bar
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Grid: 1m");
                ui.separator();
                ui.label(format!("Pos: ({:.1}, {:.1}, {:.1})",
                                 self.camera.position.x,
                                 self.camera.position.y,
                                 self.camera.position.z,
                ));
                ui.separator();
                ui.label(format!("Selected: {}",
                                 match self.selected_brush {
                                     Some(id) => format!("{}", &id.to_string()[..8]),
                                     None => "None".to_string(),
                                 }
                ));
                ui.separator();
                ui.label("Speed:");
                ui.add_sized(
                    [100.0, 20.0],
                    egui::Slider::new(&mut self.camera.scroll_step, 0.0..=crate::camera::SCROLL_STEPS)
                        .show_value(false)
                );
                ui.label(format!("{:.1}", self.camera.move_speed));
                ui.separator();
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("Map Editor v0.1.0");
                });
            });
        });

        // Left panel - layers
        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Layers");
                ui.separator();

                egui::ScrollArea::vertical()
                    .id_salt("layers_scroll")
                    .show(ui, |ui| {
                        ui.collapsing("🔒 Geometry", |ui| {
                            ui.collapsing("  Ground Floor", |ui| {
                                ui.label("  □ Walls");
                                ui.label("  □ Ceiling");
                            });
                            ui.collapsing("  Upper Floor", |ui| {
                                ui.label("  □ Walls");
                                ui.label("  □ Ceiling");
                            });
                        });
                        ui.collapsing("👁 Enemies", |ui| {
                            ui.label("  □ Spawners");
                        });
                        ui.collapsing("👁 Triggers", |ui| {
                            ui.label("  □ Volume_01");
                        });
                    });
            });

        // Right panel - properties + entities
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Properties");
                ui.separator();

                egui::ScrollArea::vertical()
                    .id_salt("properties_scroll")
                    .max_height(300.0)
                    .show(ui, |ui| {
                        ui.label("Nothing selected");
                    });

                ui.separator();
                ui.heading("Entities");
                ui.separator();

                egui::ScrollArea::vertical()
                    .id_salt("entities_scroll")
                    .show(ui, |ui| {
                        ui.collapsing("Enemies", |ui| {
                            let _ = ui.button("zombie_spawner");
                            let _ = ui.button("zombie_walker");
                            let _ = ui.button("zombie_runner");
                        });
                        ui.collapsing("Pickups", |ui| {
                            let _ = ui.button("pickup_ammo");
                            let _ = ui.button("pickup_health");
                            let _ = ui.button("pickup_weapon");
                        });
                        ui.collapsing("Triggers", |ui| {
                            let _ = ui.button("trigger_volume");
                            let _ = ui.button("trigger_spawn_wave");
                        });
                        ui.collapsing("World", |ui| {
                            let _ = ui.button("light_point");
                            let _ = ui.button("light_spot");
                            let _ = ui.button("player_start");
                        });
                    });
            });

        // Central viewport
        egui::CentralPanel::default().show(ctx, |ui| {
            let (rect, response) = ui.allocate_exact_size(
                ui.available_size(),
                egui::Sense::click_and_drag(),
            );

            self.camera.update(ctx, &response, dt);

            let view = self.camera.view_matrix();
            let proj = self.camera.projection_matrix(rect.width() / rect.height());
            let view_proj = proj * view;

            // Handle selection on click
            if matches!(self.active_tool, ToolKind::Select) {
                if response.clicked() {
                    if let Some(mouse_pos) = ctx.input(|i| i.pointer.interact_pos()) {
                        let ray = crate::raycast::Ray::from_screen(
                            mouse_pos,
                            rect,
                            view,
                            proj,
                        );

                        // Test ray against all brushes, find closest hit
                        let mut closest: Option<(Uuid, f32)> = None;

                        for brush in &self.document.brushes {
                            if let Some(hit) = ray.intersect_brush(&brush.planes) {
                                match closest {
                                    None => closest = Some((brush.id, hit.distance)),
                                    Some((_, d)) if hit.distance < d => {
                                        closest = Some((brush.id, hit.distance));
                                    }
                                    _ => {}
                                }
                            }
                        }

                        self.selected_brush = closest.map(|(id, _)| id);
                    }
                }
            }

            let selected_center = self.selected_brush.and_then(|id| {
                self.document.brushes.iter()
                    .find(|b| b.id == id)
                    .map(|b| brush_center(b))
            });

            let callback = egui_wgpu::Callback::new_paint_callback(
                rect,
                crate::viewport::ViewportCallback {
                    view_proj,
                    camera_pos: glam::Vec3::new(
                        self.camera.position.x,
                        self.camera.position.y,
                        self.camera.position.z,
                    ),
                    selected: self.selected_brush.is_some(),
                    selected_center,
                },
            );

            ui.painter().add(callback);

            // Draw center dot for selected brush
            if let Some(center) = selected_center {
                let clip = view_proj * glam::Vec4::new(center.x, center.y, center.z, 1.0);

                // Only draw if in front of camera
                if clip.w > 0.0 {
                    let ndc = glam::Vec2::new(clip.x / clip.w, clip.y / clip.w);
                    let screen_x = rect.min.x + (ndc.x + 1.0) / 2.0 * rect.width();
                    let screen_y = rect.min.y + (1.0 - ndc.y) / 2.0 * rect.height();
                    let screen_pos = egui::pos2(screen_x, screen_y);

                    let painter = ui.painter_at(rect);

                    // Outer circle
                    painter.circle_filled(
                        screen_pos,
                        6.0,
                        egui::Color32::from_rgb(255, 140, 30),
                    );

                    // Inner dot
                    painter.circle_filled(
                        screen_pos,
                        3.0,
                        egui::Color32::WHITE,
                    );
                }
            }
        });

        // Floating texture browser
        egui::Window::new("Texture Browser")
            .open(&mut self.show_texture_browser)
            .resizable(true)
            .default_width(400.0)
            .default_height(300.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Search:");
                    ui.text_edit_singleline(&mut self.texture_search);
                });

                ui.horizontal_wrapped(|ui| {
                    ui.label("Tags:");
                    let _ = ui.button("floor");
                    let _ = ui.button("wall");
                    let _ = ui.button("wood");
                    let _ = ui.button("metal");
                    let _ = ui.button("nature");
                });

                ui.small("No tags selected — showing all textures");
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Recently Used:");
                });

                ui.separator();

                egui::ScrollArea::vertical()
                    .id_salt("texture_scroll")
                    .show(ui, |ui| {
                        egui::Grid::new("texture_grid")
                            .num_columns(4)
                            .spacing([8.0, 8.0])
                            .show(ui, |ui| {
                                for i in 0..16 {
                                    ui.group(|ui| {
                                        ui.set_min_size(egui::vec2(80.0, 80.0));
                                        ui.label(format!("tex_{:02}", i));
                                    });
                                    if (i + 1) % 4 == 0 {
                                        ui.end_row();
                                    }
                                }
                            });
                    });
            });
    }
}

fn brush_center(brush: &map_format::brush::Brush) -> glam::Vec3 {
    let mesh = map_format::geometry::brush_to_mesh(brush);
    if mesh.vertices.is_empty() {
        return glam::Vec3::ZERO;
    }
    let sum = mesh.vertices.iter().fold(
        glam::Vec3::ZERO,
        |acc, v| acc + glam::Vec3::new(v[0], v[1], v[2])
    );
    sum / mesh.vertices.len() as f32
}