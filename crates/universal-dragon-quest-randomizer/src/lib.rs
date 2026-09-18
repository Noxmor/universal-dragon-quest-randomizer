use std::path::{Path, PathBuf};

use eframe::egui;
use egui_async::Bind;
use rfd::AsyncFileDialog;

const APP_TITLE: &str = "Universal Dragon Quest Randomizer";

const OPEN_ROM_SHORTCUT: egui::KeyboardShortcut =
    egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::O);

const QUIT_SHORTCUT: egui::KeyboardShortcut =
    egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::Q);

pub fn run() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        centered: true,
        viewport: egui::ViewportBuilder::default()
            .with_title(APP_TITLE)
            .with_inner_size([1920.0, 1080.0])
            .with_min_inner_size([1280.0, 720.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };

    eframe::run_native(
        APP_TITLE,
        native_options,
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )
}

struct App {
    rom_path: Option<PathBuf>,
    open_rom: Bind<Option<PathBuf>, ()>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            rom_path: None,
            open_rom: Bind::new(true),
        }
    }
}

impl App {
    fn request_open_rom(&mut self) {
        self.open_rom.request(async {
            let file = AsyncFileDialog::new().pick_file().await;

            Ok(file.map(|file| file.path().to_path_buf()))
        });
    }

    fn handle_open_rom_result(&mut self) {
        if let Some(Ok(Some(path))) = self.open_rom.take() {
            self.set_rom(path);
        }
    }

    fn set_rom(&mut self, path: PathBuf) {
        self.rom_path = Some(path);
    }
}

impl eframe::App for App {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.plugin_or_default::<egui_async::EguiAsyncPlugin>();
        self.handle_open_rom_result();

        let open_rom_requested = ctx.input_mut(|input| input.consume_shortcut(&OPEN_ROM_SHORTCUT));

        if open_rom_requested && !self.open_rom.is_pending() {
            self.request_open_rom();
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
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

        egui::CentralPanel::default().show(ui, |ui| {
            let available_height = ui.available_height();

            ui.vertical_centered(|ui| {
                ui.add_space((available_height * 0.28).max(48.0));

                ui.heading(APP_TITLE);
                ui.add_space(12.0);

                if self.open_rom.is_pending() {
                    ui.add_space(12.0);
                    ui.spinner();
                    ui.label("Waiting for file selection…");
                } else if let Some(path) = &self.rom_path {
                    ui.add_space(16.0);
                    ui.label(format!("Selected ROM: {}", display_name(path)));
                }
            });
        });
    }
}

fn display_name(path: &Path) -> &str {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("Unknown file")
}
