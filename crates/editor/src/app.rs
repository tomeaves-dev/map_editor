use eframe::egui_wgpu;
use crate::viewport::ViewportRenderer;
use crate::camera::Camera;

pub struct MapEditorApp {
    camera: Camera,
    show_texture_browser: bool,
    texture_search: String,
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

        Self {
            camera: Camera::new(),
            show_texture_browser: false,
            texture_search: String::new(),
        }
    }
}

impl eframe::App for MapEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let dt = ctx.input(|i| i.unstable_dt);
        ctx.request_repaint();

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    ui.button("New");
                    ui.button("Open");
                    ui.button("Save");
                    ui.button("Save As");
                    ui.separator();
                    ui.button("Export");
                    ui.separator();
                    ui.button("Quit");
                });
                ui.menu_button("Edit", |ui| {
                    ui.button("Undo");
                    ui.button("Redo");
                    ui.separator();
                    ui.button("Cut");
                    ui.button("Copy");
                    ui.button("Paste");
                    ui.separator();
                    ui.button("Select All");
                });
                ui.menu_button("View", |ui| {
                    ui.button("Toggle Grid");
                    ui.button("Toggle Stats");
                    ui.separator();
                    if ui.button("Texture Browser").clicked() {
                        self.show_texture_browser = !self.show_texture_browser;
                        ui.close_menu();
                    }
                });
                ui.menu_button("Help", |ui| {
                    ui.button("Documentation");
                    ui.button("About");
                });
            });
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Grid: 32");
                ui.separator();
                ui.label(format!("Pos: ({:.0}, {:.0}, {:.0})",
                                 self.camera.position.x,
                                 self.camera.position.y,
                                 self.camera.position.z,
                ));
                ui.separator();
                ui.label("Selected: None");
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

        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Layers");
                ui.separator();

                egui::ScrollArea::vertical()
                    .id_salt("layers_scroll")
                    .max_height(300.0)
                    .show(ui, |ui| {
                        // Placeholder layer tree
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

                ui.separator();
                ui.heading("Tools");
                ui.separator();

                egui::ScrollArea::vertical()
                    .id_salt("tools_scroll")
                    .show(ui, |ui| {
                        ui.selectable_label(true,  "⬆ Select");
                        ui.selectable_label(false, "✋ Translate");
                        ui.selectable_label(false, "🔄 Rotate");
                        ui.selectable_label(false, "✂ Clip");
                        ui.selectable_label(false, "◆ Vertex");
                    });
            });

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
                            ui.button("zombie_spawner");
                            ui.button("zombie_walker");
                            ui.button("zombie_runner");
                        });
                        ui.collapsing("Pickups", |ui| {
                            ui.button("pickup_ammo");
                            ui.button("pickup_health");
                            ui.button("pickup_weapon");
                        });
                        ui.collapsing("Triggers", |ui| {
                            ui.button("trigger_volume");
                            ui.button("trigger_spawn_wave");
                        });
                        ui.collapsing("World", |ui| {
                            ui.button("light_point");
                            ui.button("light_spot");
                            ui.button("player_start");
                        });
                    });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let (rect, response) = ui.allocate_exact_size(
                ui.available_size(),
                egui::Sense::click_and_drag(),
            );

            self.camera.update(ctx, &response, dt);

            let view = self.camera.view_matrix();
            let proj = self.camera.projection_matrix(rect.width() / rect.height());
            let view_proj = proj * view;

            let callback = egui_wgpu::Callback::new_paint_callback(
                rect,
                crate::viewport::ViewportCallback {
                    view_proj,
                    camera_pos: glam::Vec3::new(
                        self.camera.position.x,
                        self.camera.position.y,
                        self.camera.position.z,
                    ),
                },
            );

            ui.painter().add(callback);
        });

        egui::Window::new("Texture Browser")
            .open(&mut self.show_texture_browser)
            .resizable(true)
            .default_width(400.0)
            .default_height(300.0)
            .show(ctx, |ui| {
                // Search and tag filter bar
                ui.horizontal(|ui| {
                    ui.label("Search:");
                    ui.text_edit_singleline(&mut self.texture_search);
                });

                ui.horizontal_wrapped(|ui| {
                    ui.label("Tags:");
                    ui.button("floor");
                    ui.button("wall");
                    ui.button("wood");
                    ui.button("metal");
                    ui.button("nature");
                });

                ui.small("No tags selected — showing all textures");

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Recently Used:");
                });

                ui.separator();

                // Texture grid placeholder
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