use egui_tetra::egui;

use crate::graph::GraphOnCanvas;
use crate::graph::random::generate;
use crate::input::input_state::{ConnectData, InputState, MoveData};
use crate::step_algorithms::dfs::Dfs;
use crate::step_algorithms::algorithm::{Algorithm, VisibleAlgorithm};
use crate::GameState;
use tetra::Context;

pub fn graph_params_editor_ui(
    game_state: &mut GameState,
    ctx: &mut Context,
    egui_ctx: &egui::CtxRef,
) {
    egui::Window::new("Create").show(egui_ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("Nodes");
            ui.add(egui::DragValue::new(&mut game_state.node_count));
        });
        ui.horizontal(|ui| {
            ui.label("Edges");
            ui.add(egui::DragValue::new(&mut game_state.edge_count));
        });
        if ui.button("Generate").clicked() {
            // TODO: Un Unwrap
            generate(ctx, &mut game_state.graph, game_state.node_count, game_state.edge_count);
        }
    });

    egui::Window::new("Edit").show(egui_ctx, |ui| {
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
            ui.add(egui::DragValue::new(&mut game_state.push_conf().force()));
        });
        ui.horizontal(|ui| {
            ui.label("Distance");
            ui.add(egui::DragValue::new(&mut game_state.push_conf().distance()));
        });
        ui.label("Pull:");
        ui.horizontal(|ui| {
            ui.label("Value");
            ui.add(egui::DragValue::new(
                &mut game_state.pull_conf().force_at_twice_distance(),
            ));
        });
        ui.horizontal(|ui| {
            ui.label("Min Distance");
            ui.add(egui::DragValue::new(
                &mut game_state.pull_conf().min_distance(),
            ));
        });
        if ui.button("dfs").clicked() {
            if let Some(idx) = game_state.graph.node_indices().next() {
                let mut algorithm = Dfs::new(idx);
                algorithm.run_algorithm(&mut game_state.graph);
                algorithm.show_algorithm(&mut game_state.graph);

                game_state.add_algorithm(Box::new(algorithm));
            }
        }
        // This is done dirty, just to be able to quickly build nontrivial graph.
        if ui.button("clique").clicked() {
            for node in game_state.graph.node_indices() {
                for node_other in game_state.graph.node_indices() {
                    if node != node_other {
                        game_state
                            .graph
                            .connect_nodes(ctx, node, node_other);
                    }
                }
            }
        }
    });
}
