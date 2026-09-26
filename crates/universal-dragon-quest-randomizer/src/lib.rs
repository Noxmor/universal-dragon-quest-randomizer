use eframe::egui;
use std::sync::mpsc::{self, Receiver, Sender};

use crate::randomizer::RandomizerSettings;

mod randomizer;
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
    Loading,
    Loaded(Result<rom::Rom, rom::Error>),
}

struct App {
    rom: RomState,
    rom_tx: Sender<Result<rom::Rom, rom::Error>>,
    rom_rx: Receiver<Result<rom::Rom, rom::Error>>,
    settings: RandomizerSettings,
    menu_bar: ui::MenuBar,
    editor: ui::Editor,
}

impl Default for App {
    fn default() -> Self {
        let (rom_tx, rom_rx) = mpsc::channel();
        Self {
            rom: RomState::Empty,
            rom_tx,
            rom_rx,
            settings: RandomizerSettings::default(),
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
        } else if let Some(path) = self.menu_bar.retrieve_rom() {
            self.rom = RomState::Loading;

            let tx = self.rom_tx.clone();

            std::thread::spawn(move || {
                let result = rom::RomLoader::new(path).load();

                let _ = tx.send(result);
            });
        } else if let Ok(rom) = self.rom_rx.try_recv() {
            self.rom = RomState::Loaded(rom);
        } else if !matches!(self.rom, RomState::Loading | RomState::Loaded(_)) {
            self.rom = RomState::Empty;
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.menu_bar.render(ui);
        self.editor.render(ui, &self.rom, &mut self.settings);
    }
}
