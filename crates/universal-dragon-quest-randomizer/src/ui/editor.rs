use crate::{APP_TITLE, RomState, rom::Rom};
use eframe::egui;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
enum SettingsCategory {
    Test, // TODO: Replace
}

impl SettingsCategory {
    pub fn name(self) -> &'static str {
        match self {
            Self::Test => "Test",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Test => "Test.",
        }
    }
}

pub struct Editor {
    selected_category: SettingsCategory,
    seed: String,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            selected_category: SettingsCategory::Test,
            seed: String::new(),
        }
    }
    pub fn render(&mut self, ui: &mut egui::Ui, rom: &RomState) {
        egui::CentralPanel::default().show(ui, |ui| {
            let available_height = ui.available_height();

            ui.vertical_centered(|ui| {
                ui.vertical_centered(|ui| match rom {
                    RomState::Empty => {
                        ui.add_space((available_height * 0.28).max(48.0));

                        ui.heading(APP_TITLE);
                        ui.add_space(12.0);
                    }
                    RomState::Pending => {
                        ui.add_space((available_height * 0.28).max(48.0));

                        ui.heading(APP_TITLE);
                        ui.add_space(12.0);
                        ui.spinner();
                        ui.label("Waiting for file selection…");
                    }
                    RomState::Loaded(rom) => match rom {
                        Ok(rom) => {
                            ui.add_space(12.0);
                            ui.heading(APP_TITLE);
                            ui.add_space(12.0);

                            ui.columns(2, |columns| {
                                self.render_rom_information(&mut columns[0], rom);
                                self.render_general(&mut columns[1]);
                            });

                            ui.add_space(12.0);

                            self.render_settings_editor(ui);
                        }
                        Err(e) => {
                            ui.add_space((available_height * 0.28).max(48.0));

                            ui.heading(APP_TITLE);
                            ui.add_space(12.0);
                            ui.label(format!("Error loading ROM: {e}"));
                        }
                    },
                });
            });
        });
    }

    fn render_rom_information(&self, ui: &mut egui::Ui, rom: &Rom) {
        egui::Frame::group(ui.style())
            .inner_margin(egui::Margin::same(12))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("ROM Information");

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new(format!("({})", rom.path().display())).small(),
                        );
                    });
                });

                ui.separator();

                ui.add_space(10.0);

                egui::Grid::new("rom_information")
                    .num_columns(1)
                    .spacing([32.0, 8.0])
                    .striped(false)
                    .show(ui, |ui| {
                        Self::info_item(ui, "ROM ID", &rom.definition().id.to_string());

                        ui.end_row();

                        Self::info_item(ui, "Region", &rom.definition().region.to_string());

                        ui.end_row();

                        Self::info_item(
                            ui,
                            "Revision",
                            &rom.definition().revision.value().to_string(),
                        );

                        ui.end_row();
                    });
            });
    }

    fn render_general(&mut self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style())
            .inner_margin(egui::Margin::same(12))
            .show(ui, |ui| {
                ui.heading("General");

                ui.separator();
                ui.add_space(8.0);

                egui::Grid::new("general_settings")
                    .num_columns(2)
                    .spacing([20.0, 10.0])
                    .show(ui, |ui| {
                        ui.label("Seed");

                        ui.horizontal(|ui| {
                            let response = ui.add(
                                egui::TextEdit::singleline(&mut self.seed).desired_width(140.0),
                            );

                            if response.lost_focus() {
                                // TODO: Apply seed
                            }

                            if ui.button("🎲").clicked() {
                                // TODO: Generate seed
                            }
                        });

                        ui.end_row();

                        if ui.button("Randomize (Save)").clicked() {
                            // TODO: Implement
                        }

                        ui.end_row();
                    });
            });
    }

    fn info_item(ui: &mut egui::Ui, label: &str, value: &str) {
        ui.vertical(|ui| {
            ui.label(
                egui::RichText::new(label)
                    .small()
                    .color(ui.visuals().weak_text_color()),
            );

            ui.label(egui::RichText::new(value).strong());
        });
    }

    fn render_settings_editor(&mut self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style())
            .inner_margin(egui::Margin::same(12))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Randomization");
                });

                ui.separator();

                ui.vertical(|ui| {
                    self.render_category_navigation_bar(ui);

                    ui.add_space(12.0);

                    ui.separator();

                    ui.add_space(12.0);

                    self.render_category_content(ui);
                });
            });
    }

    fn render_category_navigation_bar(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .id_salt("settings_categories")
            .max_width(190.0)
            .show(ui, |ui| {
                ui.add_space(8.0);

                for category in SettingsCategory::iter() {
                    let selected = self.selected_category == category;

                    let response = ui.selectable_label(
                        selected,
                        egui::RichText::new(category.name()).size(14.0),
                    );

                    if response.clicked() {
                        self.selected_category = category;
                    }

                    if response.hovered() {
                        response.on_hover_text(category.description());
                    }
                }
            });
    }

    fn render_category_content(&self, ui: &mut egui::Ui) {
        ui.set_min_height(ui.available_height());
    }
}
