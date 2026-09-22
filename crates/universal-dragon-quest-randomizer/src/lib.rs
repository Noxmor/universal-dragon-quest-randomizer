use eframe::egui;

mod rom;
mod ui;

pub const APP_TITLE: &str = "Universal Dragon Quest Randomizer";

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
    rom: Option<Result<rom::Rom, rom::RomError>>,
    menu_bar: ui::MenuBar,
}

impl Default for App {
    fn default() -> Self {
        Self {
            rom: None,
            menu_bar: ui::MenuBar::new(),
        }
    }
}

impl eframe::App for App {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.plugin_or_default::<egui_async::EguiAsyncPlugin>();

        self.menu_bar.update(ctx);
        if let Some(rom) = self.menu_bar.retrieve_rom() {
            self.rom = Some(rom);
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.menu_bar.render(ui);

        egui::CentralPanel::default().show(ui, |ui| {
            let available_height = ui.available_height();

            ui.vertical_centered(|ui| {
                ui.add_space((available_height * 0.28).max(48.0));

                ui.heading(APP_TITLE);
                ui.add_space(12.0);

                if self.menu_bar.is_pending() {
                    ui.add_space(12.0);
                    ui.spinner();
                    ui.label("Waiting for file selection…");

                    ui.add_space(16.0);
                } else if let Some(Ok(rom)) = &self.rom {
                    let name = &rom.definition().id;
                    let region = &rom.definition().region;
                    ui.label(format!("Selected ROM: {name} ({region})"));
                } else if let Some(Err(e)) = &self.rom {
                    ui.label(format!("Error loading ROM: {e}"));
                }
            });
        });
    }
}
