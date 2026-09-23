use crate::{APP_TITLE, RomState};
use eframe::egui;

pub struct Editor;

impl Editor {
    pub fn new() -> Self {
        Self {}
    }
    pub fn render(&self, ui: &mut egui::Ui, rom: &RomState) {
        egui::CentralPanel::default().show(ui, |ui| {
            let available_height = ui.available_height();

            ui.vertical_centered(|ui| {
                ui.add_space((available_height * 0.28).max(48.0));

                ui.heading(APP_TITLE);
                ui.add_space(12.0);

                match rom {
                    RomState::Empty => {}
                    RomState::Pending => {
                        ui.add_space(12.0);
                        ui.spinner();
                        ui.label("Waiting for file selection…");
                        ui.add_space(16.0);
                    }
                    RomState::Loaded(rom) => match rom {
                        Ok(rom) => {
                            let name = &rom.definition().id;
                            let region = &rom.definition().region;
                            ui.label(format!("Selected ROM: {name} ({region})"));
                        }
                        Err(e) => {
                            ui.label(format!("Error loading ROM: {e}"));
                        }
                    },
                }
            });
        });
    }
}
