use crate::input::input_state::*;
use crate::GameState;
use egui_tetra::egui;

pub trait Gui {
    fn create(&mut self, egui_ctx: &egui::CtxRef, game_state: GameState);
}

pub struct MainGui {
    pub input_state: Box<dyn PartialEq<Self> + InputState>,
}

impl Gui for MainGui {
    fn create(&mut self, egui_ctx: &egui::CtxRef, game_state: GameState) {
        egui::Window::new("Graph editor").show(egui_ctx, |ui| {
            ui.heading("Mode");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.input_state, Box::new(AddState), "Add");
            });
        });
    }
}
