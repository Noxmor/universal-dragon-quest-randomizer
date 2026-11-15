use crate::{
    APP_TITLE, RomState,
    randomizer::{RandomizerConfig, Seed},
    rom::Rom,
};
use eframe::egui;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
enum ConfigCategory {
    Encounters,
    General,
    Shops,
    Skills,
    Synthesis,
}

impl ConfigCategory {
    pub fn name(self) -> &'static str {
        match self {
            Self::Encounters => "Encounters",
            Self::General => "General",
            Self::Shops => "Shops",
            Self::Skills => "Skills",
            Self::Synthesis => "Synthesis",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Encounters => "Randomizes encounters.",
            Self::General => "General configuration.",
            Self::Shops => "Randomizes shops.",
            Self::Skills => "Randomizes skills.",
            Self::Synthesis => "Randomizes synthesis recipes.",
        }
    }
}

impl Default for ConfigCategory {
    fn default() -> Self {
        Self::General
    }
}

pub struct Editor {
    selected_category: ConfigCategory,
    seed: String,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            selected_category: ConfigCategory::default(),
            seed: String::new(),
        }
    }
    pub fn render(&mut self, ui: &mut egui::Ui, rom: &RomState, cfg: &mut RandomizerConfig) {
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
                    RomState::Loading => {
                        ui.add_space((available_height * 0.28).max(48.0));

                        ui.heading(APP_TITLE);
                        ui.add_space(12.0);
                        ui.spinner();
                        ui.label("Verifying ROM…");
                    }
                    RomState::Loaded(rom) => match rom {
                        Ok(rom) => {
                            ui.add_space(12.0);
                            ui.heading(APP_TITLE);
                            ui.add_space(12.0);

                            ui.columns(2, |columns| {
                                self.render_rom_information(&mut columns[0], rom);
                                self.render_general(&mut columns[1], cfg);
                            });

                            ui.add_space(12.0);

                            self.render_config_editor(ui, cfg);
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
            .stroke(egui::Stroke::new(
                2.0,
                ui.style().visuals.widgets.noninteractive.bg_stroke.color,
            ))
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
                        Self::info_item(ui, "ROM ID", &rom.id.to_string());

                        ui.end_row();

                        Self::info_item(ui, "Region", &rom.region.to_string());

                        ui.end_row();

                        Self::info_item(ui, "Revision", &rom.revision.value().to_string());

                        ui.end_row();
                    });
            });
    }

    fn render_general(&mut self, ui: &mut egui::Ui, cfg: &mut RandomizerConfig) {
        egui::Frame::group(ui.style())
            .inner_margin(egui::Margin::same(12))
            .stroke(egui::Stroke::new(
                2.0,
                ui.style().visuals.widgets.noninteractive.bg_stroke.color,
            ))
            .show(ui, |ui| {
                ui.heading("General");

                ui.separator();
                ui.add_space(8.0);

                egui::Grid::new("general_config")
                    .num_columns(2)
                    .spacing([20.0, 10.0])
                    .show(ui, |ui| {
                        ui.label("Seed");

                        ui.horizontal(|ui| {
                            let seed_width = ui.fonts_mut(|fonts| {
                                fonts.glyph_width(&egui::TextStyle::Body.resolve(ui.style()), '0')
                                    * 20.0
                            });

                            let response = ui.add(
                                egui::TextEdit::singleline(&mut self.seed)
                                    .desired_width(seed_width + 16.0)
                                    .char_limit(20),
                            );

                            if response.changed() {
                                self.seed.retain(|c| c.is_ascii_digit());
                            }

                            if response.lost_focus() && !self.seed.is_empty() {
                                match self.seed.parse::<Seed>() {
                                    Ok(seed) => {
                                        cfg.seed = seed;
                                    }
                                    Err(_) => {
                                        self.seed = u64::MAX.to_string();
                                        cfg.seed = u64::MAX;
                                    }
                                }
                            }

                            if ui.button("🎲").clicked() {
                                let seed: Seed = rand::random();

                                self.seed = seed.to_string();
                                cfg.seed = seed;
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

    fn render_config_editor(&mut self, ui: &mut egui::Ui, cfg: &mut RandomizerConfig) {
        egui::Frame::group(ui.style())
            .inner_margin(egui::Margin::same(12))
            .stroke(egui::Stroke::new(
                2.0,
                ui.style().visuals.widgets.noninteractive.bg_stroke.color,
            ))
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

                    self.render_category_content(ui, cfg);
                });
            });
    }

    fn render_category_navigation_bar(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .id_salt("config_categories")
            .max_width(190.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(8.0);

                    for category in ConfigCategory::iter() {
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
            });
    }

    fn render_category_content(&self, ui: &mut egui::Ui, cfg: &mut RandomizerConfig) {
        ui.set_min_height(ui.available_height());
    }
}
