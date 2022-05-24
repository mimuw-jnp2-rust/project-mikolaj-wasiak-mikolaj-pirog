use egui_tetra::egui;

use crate::input::input_state::{ConnectData, InputState, MoveData};
use crate::step_algorithms::algorithm::Algorithm;
use crate::GameState;

pub fn graph_params_editor_ui(game_state: &mut GameState, egui_ctx: &egui::CtxRef) {
    egui::Window::new("Graph editor").show(egui_ctx, |ui| {
        ui.heading("Mode");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut game_state.input_state, InputState::Add, "Add");
            ui.selectable_value(&mut game_state.input_state, InputState::Remove, "Remove");
        });
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut game_state.input_state,
                InputState::Connect(ConnectData::default()),
                "Connect",
            );
            ui.selectable_value(
                &mut game_state.input_state,
                InputState::Move(MoveData::default()),
                "Move",
            );
        });
        ui.heading("Forces");
        ui.label("Push:");
        ui.horizontal(|ui| {
            ui.label("Value");
            ui.add(egui::DragValue::new(&mut game_state.push_conf.force()));
        });
        ui.horizontal(|ui| {
            ui.label("Distance");
            ui.add(egui::DragValue::new(&mut game_state.push_conf.distance()));
        });
        ui.label("Pull:");
        ui.horizontal(|ui| {
            ui.label("Value");
            ui.add(egui::DragValue::new(
                &mut game_state.pull_conf.force_at_twice_distance(),
            ));
        });
        ui.horizontal(|ui| {
            ui.label("Min Distance");
            ui.add(egui::DragValue::new(
                &mut game_state.pull_conf.min_distance(),
            ));
        });
        if ui.button("dfs").clicked() {
            if let Some(idx) = game_state.graph.node_indices().next() {
                game_state.algorithm = Some(Algorithm::new(idx));
                if let Some(algo) = &mut game_state.algorithm {
                    algo.show_dfs(&mut game_state.graph);
                }
            }
        }
    });
}
