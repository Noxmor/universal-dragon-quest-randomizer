use std::path::{Path, PathBuf};

use eframe::egui;

mod ui;

const APP_TITLE: &str = "Universal Dragon Quest Randomizer";

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
    menu_bar: ui::MenuBar,
}

impl Default for App {
    fn default() -> Self {
        Self {
            rom_path: None,
            menu_bar: ui::MenuBar::new(),
        }
    }
}

impl App {
    fn handle_open_rom_result(&mut self) {
        if let Some(Ok(Some(path))) = self.menu_bar.retrieve_open_rom().take() {
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

        self.menu_bar.update(ctx);
        self.handle_open_rom_result();
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.menu_bar.render(ui);

        egui::CentralPanel::default().show(ui, |ui| {
            let available_height = ui.available_height();

            ui.vertical_centered(|ui| {
                ui.add_space((available_height * 0.28).max(48.0));

                ui.heading(APP_TITLE);
                ui.add_space(12.0);

                if self.menu_bar.retrieve_open_rom().is_pending() {
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
