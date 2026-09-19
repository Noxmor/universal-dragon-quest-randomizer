use std::path::PathBuf;

use eframe::egui;
use egui_async::Bind;
use rfd::AsyncFileDialog;

const OPEN_ROM_SHORTCUT: crate::egui::KeyboardShortcut =
    egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::O);

const QUIT_SHORTCUT: egui::KeyboardShortcut =
    egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::Q);

pub struct MenuBar {
    open_rom: Bind<Option<PathBuf>, ()>,
}

impl MenuBar {
    pub fn new() -> Self {
        Self {
            open_rom: Bind::new(true),
        }
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        let open_rom_requested = ctx.input_mut(|input| input.consume_shortcut(&OPEN_ROM_SHORTCUT));

        if open_rom_requested && !self.open_rom.is_pending() {
            self.request_open_rom();
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        egui::Panel::top("menu_bar").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    let open_enabled = !self.open_rom.is_pending();

                    let open_rom_shortcut = ui.ctx().format_shortcut(&OPEN_ROM_SHORTCUT);
                    let quit_shortcut = ui.ctx().format_shortcut(&QUIT_SHORTCUT);

                    if ui
                        .add_enabled(
                            open_enabled,
                            egui::Button::new("Open ROM…").shortcut_text(open_rom_shortcut),
                        )
                        .clicked()
                    {
                        ui.close();
                        self.request_open_rom();
                    }

                    ui.separator();

                    if ui
                        .add(egui::Button::new("Quit").shortcut_text(quit_shortcut))
                        .clicked()
                    {
                        ui.close();
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });
    }

    pub fn retrieve_open_rom(&mut self) -> &mut Bind<Option<PathBuf>, ()> {
        &mut self.open_rom
    }

    fn request_open_rom(&mut self) {
        self.open_rom.request(async {
            let file = AsyncFileDialog::new().pick_file().await;

            Ok(file.map(|file| file.path().to_path_buf()))
        });
    }
}
