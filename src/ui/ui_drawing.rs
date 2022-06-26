use egui_tetra::egui::{self, Button, Ui};
use petgraph::graph::NodeIndex;
use petgraph::{Directed, Undirected};

use crate::graph::random::generate;
use crate::graph::GraphOnCanvas;
use crate::input::input_state::{InputState, StateData};

use crate::ui::ui_state::UiMode;

use crate::step_algorithms::{Bfs, Dfs, Scc, StepAlgorithm};
use crate::step_algorithms::{DirectedStepAlgorithm, UndirectedStepAlgorithm};
use crate::GameState;
use tetra::Context;

fn controls_ui(game_state: &mut GameState, _ctx: &mut Context, egui_ctx: &egui::CtxRef) {
    egui::Window::new("Controls").show(egui_ctx, |ui| {
        ui.checkbox(
            game_state.tetra_info_mut().ui_data_mut().directed_mut(),
            "directed",
        );
        ui.horizontal(|ui| {
            ui.selectable_value(
                game_state.tetra_info_mut().ui_data_mut().state_mut(),
                UiMode::Edit,
                "Edit graph",
            );
            ui.selectable_value(
                game_state.tetra_info_mut().ui_data_mut().state_mut(),
                UiMode::Algorithm,
                "Show algos",
            );
        });
        if ui.button("reset state").clicked() {
            game_state.graph.reset_state();
        }
    });
}

fn graph_editor_ui(game_state: &mut GameState, ctx: &mut Context, egui_ctx: &egui::CtxRef) {
    if matches!(game_state.input_state, InputState::Select(_)) {
        game_state.input_state = InputState::Move(StateData::default());
    }
    egui::Window::new("Edit").show(egui_ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("Nodes");
            ui.add(egui::DragValue::new(
                game_state.tetra_info_mut().ui_data_mut().node_count_mut(),
            ));
        });
        ui.horizontal(|ui| {
            ui.label("Edges");
            ui.add(egui::DragValue::new(
                game_state.tetra_info_mut().ui_data_mut().edge_count_mut(),
            ));
        });
        if ui.button("Generate").clicked() {
            game_state.graph = generate(
                ctx,
                *game_state.tetra_info_mut().ui_data_mut().node_count_mut(),
                *game_state.tetra_info_mut().ui_data_mut().edge_count(),
                game_state.font(),
            );
        }
        ui.heading("Edit Mode");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut game_state.input_state, InputState::Add, "Add");
            ui.selectable_value(&mut game_state.input_state, InputState::Remove, "Remove");
        });
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut game_state.input_state,
                InputState::Connect(StateData::default()),
                "Connect",
            );
            ui.selectable_value(
                &mut game_state.input_state,
                InputState::Move(StateData::default()),
                "Move",
            );
        });

        ui.heading("Forces");
        ui.label("Push:");
        ui.horizontal(|ui| {
            ui.label("Value");
            ui.add(egui::DragValue::new(
                game_state
                    .tetra_info_mut()
                    .ui_data_mut()
                    .push_conf_mut()
                    .force_mut(),
            ));
        });
        ui.horizontal(|ui| {
            ui.label("Distance");
            ui.add(egui::DragValue::new(
                game_state
                    .tetra_info_mut()
                    .ui_data_mut()
                    .push_conf_mut()
                    .distance_mut(),
            ));
        });
        ui.label("Pull:");
        ui.horizontal(|ui| {
            ui.label("Value");
            ui.add(egui::DragValue::new(
                game_state
                    .tetra_info_mut()
                    .ui_data_mut()
                    .pull_conf_mut()
                    .force_at_twice_distance_mut(),
            ));
        });
        ui.horizontal(|ui| {
            ui.label("Min Distance");
            ui.add(egui::DragValue::new(
                game_state
                    .tetra_info_mut()
                    .ui_data_mut()
                    .pull_conf_mut()
                    .min_distance_mut(),
            ));
        });

        // This is done dirty, just to be able to quickly build nontrivial graph.
        if ui.button("clique").clicked() {
            for node in game_state.graph.node_indices() {
                for node_other in game_state.graph.node_indices() {
                    if node != node_other {
                        game_state.graph.connect_nodes(ctx, node, node_other);
                    }
                }
            }
        }
    });
}

fn create_algo_button<T: StepAlgorithm>(
    game_state: &mut GameState,
    selected_idx_opt: Option<NodeIndex>,
    ui: &mut Ui,
    mut algo: T,
    button_name: &str,
) {
    if ui
        .add_enabled(
            matches!(selected_idx_opt, Some(_)),
            Button::new(button_name),
        )
        .clicked()
    {
        if let Some(idx) = selected_idx_opt {
            let is_directed = game_state.tetra_info_mut().ui_data_mut().directed();
            let graph_copy = game_state.graph.clone();
            if is_directed {
                algo.run(&graph_copy.into_edge_type::<Directed>(), idx);
            } else {
                algo.run(&graph_copy.into_edge_type::<Undirected>(), idx);
            }
            game_state.add_algorithm(algo.result());
        }
    }
}

fn create_directed_algo_button<T: DirectedStepAlgorithm>(
    game_state: &mut GameState,
    selected_idx_opt: Option<NodeIndex>,
    ui: &mut Ui,
    mut algo: T,
    button_name: &str,
) {
    if ui
        .add_enabled(
            matches!(selected_idx_opt, Some(_)) && game_state.tetra_info().ui_data().directed(),
            Button::new(button_name),
        )
        .clicked()
    {
        if let Some(idx) = selected_idx_opt {
            algo.run(&game_state.graph, idx);
            game_state.add_algorithm(algo.result());
        }
    }
}

fn _create_undirected_algo_button<T: UndirectedStepAlgorithm>(
    game_state: &mut GameState,
    selected_idx_opt: Option<NodeIndex>,
    ui: &mut Ui,
    mut algo: T,
    button_name: &str,
) {
    if ui
        .add_enabled(
            matches!(selected_idx_opt, Some(_)) && !game_state.tetra_info().ui_data().directed(),
            Button::new(button_name),
        )
        .clicked()
    {
        if let Some(idx) = selected_idx_opt {
            let graph_copy = game_state.graph.clone().into_edge_type::<Undirected>();
            algo.run(&graph_copy.into_edge_type::<Undirected>(), idx);
            game_state.add_algorithm(algo.result());
        }
    }
}

fn algorithm_ui(game_state: &mut GameState, _ctx: &mut Context, egui_ctx: &egui::CtxRef) {
    if !matches!(game_state.input_state, InputState::Select(_)) {
        game_state.input_state = InputState::Select(StateData::default());
    }

    let idx_opt = if let InputState::Select(data) = &mut game_state.input_state {
        data.selected_node
    } else {
        None
    };

    egui::Window::new("Show algorithms").show(egui_ctx, |ui| {
        create_algo_button(
            game_state,
            idx_opt,
            ui,
            Dfs::from_graph(&game_state.graph),
            "dfs",
        );
        create_algo_button(
            game_state,
            idx_opt,
            ui,
            Bfs::from_graph(&game_state.graph),
            "bfs",
        );
        create_directed_algo_button(
            game_state,
            game_state.graph.node_indices().next(),
            ui,
            Scc::new(),
            "strongly connected components",
        )
    });
}

// Disable editing when algorithm is running, disable algorithm when editing
pub fn create_ui(game_state: &mut GameState, ctx: &mut Context, egui_ctx: &egui::CtxRef) {
    controls_ui(game_state, ctx, egui_ctx);
    if matches!(game_state.tetra_info().ui_data().state(), UiMode::Edit) {
        graph_editor_ui(game_state, ctx, egui_ctx);
    } else {
        algorithm_ui(game_state, ctx, egui_ctx);
    }
}
