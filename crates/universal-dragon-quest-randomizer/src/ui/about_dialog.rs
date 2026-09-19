use eframe::egui;
use egui::{Align, Layout, Vec2};

pub struct AboutDialog {
    open: bool,
}

impl AboutDialog {
    pub fn new() -> Self {
        Self { open: false }
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        if !self.open {
            return;
        }

        let response = egui::Modal::new(egui::Id::new("about_dialog")).show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(crate::APP_TITLE);

                ui.add_space(16.0);

                ui.label(format!("Version {}", env!("CARGO_PKG_VERSION")));

                ui.separator();

                ui.add_space(16.0);

                ui.label("A free and open-source randomizer for Dragon Quest games.");

                ui.add_space(16.0);

                ui.columns(2, |columns| {
                    columns[0].allocate_ui_with_layout(Vec2::ZERO, Layout::right_to_left(Align::Center), |ui| {
                        ui.hyperlink_to("License", concat!(env!("CARGO_PKG_REPOSITORY"), "/blob/main/LICENSE"));
                    });

                    columns[1].allocate_ui_with_layout(Vec2::ZERO, Layout::left_to_right(Align::Center), |ui| {
                        ui.hyperlink_to("GitHub", env!("CARGO_PKG_REPOSITORY"));
                    });
                });

                ui.add_space(16.0);

                ui.separator();

                ui.add_space(16.0);

                ui.label("\"Dragon Quest\" is a trademark of Square Enix. This project is not affiliated with Square Enix in any way.")
            });
        });

        if ui.input(|i| i.key_pressed(egui::Key::Escape)) || response.backdrop_response.clicked() {
            self.open = false;
        }
    }

    pub fn open(&mut self) {
        self.open = true;
    }
}
