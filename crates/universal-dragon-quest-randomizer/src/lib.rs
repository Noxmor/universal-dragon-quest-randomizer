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

enum RomState {
    Empty,
    Pending,
    Loaded(Result<rom::Rom, rom::RomError>),
}

struct App {
    rom: RomState,
    menu_bar: ui::MenuBar,
    editor: ui::Editor,
}

impl Default for App {
    fn default() -> Self {
        Self {
            rom: RomState::Empty,
            menu_bar: ui::MenuBar::new(),
            editor: ui::Editor::new(),
        }
    }
}

impl eframe::App for App {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.plugin_or_default::<egui_async::EguiAsyncPlugin>();

        self.menu_bar.update(ctx);

        if self.menu_bar.is_pending() {
            self.rom = RomState::Pending;
        } else if let Some(rom) = self.menu_bar.retrieve_rom() {
            self.rom = RomState::Loaded(rom);
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.menu_bar.render(ui);
        self.editor.render(ui, &self.rom);
    }
}
