use egui_tetra::egui::{self, Button, Ui};
use petgraph::graph::NodeIndex;
use petgraph::{Directed, Undirected};

use crate::graph::edge::{
    Edge, PULL_FORCE_FORCE_AT_TWICE_DISTANCE, PULL_FORCE_MIN_DISTANCE, PUSH_FORCE_DISTANCE,
    PUSH_FORCE_FORCE,
};
use crate::graph::gravity::{PullForceConfig, PushForceConfig};
use crate::graph::node::Node;
use crate::graph::random::generate;
use crate::graph::GraphOnCanvas;
use crate::input::input_state::{InputState, StateData};

use crate::step_algorithms::{Bfs, Dfs, StepAlgorithm};
use crate::step_algorithms::{DirectedStepAlgorithm, UndirectedStepAlgorithm};
use crate::GameState;
use tetra::Context;

#[derive(PartialEq)]
enum UiState {
    Edit,
    Algorithm,
}

pub struct UiData {
    state: UiState,

    is_directed: bool,

    //   force:
    push_conf: PushForceConfig,
    pull_conf: PullForceConfig,

    //   random-gen:
    node_count: u32,
    edge_count: u32,
}

impl UiData {
    pub fn new() -> UiData {
        UiData {
            is_directed: true,
            push_conf: PushForceConfig::new(PUSH_FORCE_FORCE, PUSH_FORCE_DISTANCE),
            pull_conf: PullForceConfig::new(
                PULL_FORCE_MIN_DISTANCE,
                PULL_FORCE_FORCE_AT_TWICE_DISTANCE,
            ),
            node_count: 10,
            edge_count: 15,
            state: UiState::Edit,
        }
    }

    pub fn push_conf(&self) -> PushForceConfig {
        self.push_conf
    }

    pub fn pull_conf(&self) -> PullForceConfig {
        self.pull_conf
    }

    pub fn is_directed(&self) -> bool {
        self.is_directed
    }
}

impl Default for UiData {
    fn default() -> Self {
        Self::new()
    }
}

fn controls_ui(game_state: &mut GameState, _ctx: &mut Context, egui_ctx: &egui::CtxRef) {
    egui::Window::new("Controls").show(egui_ctx, |ui| {
        ui.checkbox(&mut game_state.ui_data.is_directed, "directed");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut game_state.ui_data.state, UiState::Edit, "Edit graph");
            ui.selectable_value(
                &mut game_state.ui_data.state,
                UiState::Algorithm,
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
            ui.add(egui::DragValue::new(&mut game_state.ui_data.node_count));
        });
        ui.horizontal(|ui| {
            ui.label("Edges");
            ui.add(egui::DragValue::new(&mut game_state.ui_data.edge_count));
        });
        if ui.button("Generate").clicked() {
            game_state.graph = generate(
                ctx,
                game_state.ui_data.node_count,
                game_state.ui_data.edge_count,
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

        // FIXME: reference to ephemeral variables - can't edit force/distance
        ui.heading("Forces");
        ui.label("Push:");
        ui.horizontal(|ui| {
            ui.label("Value");
            ui.add(egui::DragValue::new(
                &mut game_state.ui_data.push_conf().force(),
            ));
        });
        ui.horizontal(|ui| {
            ui.label("Distance");
            ui.add(egui::DragValue::new(
                &mut game_state.ui_data.push_conf().distance(),
            ));
        });
        ui.label("Pull:");
        ui.horizontal(|ui| {
            ui.label("Value");
            ui.add(egui::DragValue::new(
                &mut game_state.ui_data.pull_conf().force_at_twice_distance(),
            ));
        });
        ui.horizontal(|ui| {
            ui.label("Min Distance");
            ui.add(egui::DragValue::new(
                &mut game_state.ui_data.pull_conf().min_distance(),
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
    algo: T,
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
            let is_directed = game_state.ui_data.is_directed;
            let graph_copy = game_state.graph.clone();
            let result = if is_directed {
                algo.get_result(&graph_copy.into_edge_type::<Directed>(), idx)
            } else {
                algo.get_result(&graph_copy.into_edge_type::<Undirected>(), idx)
            };
            game_state.add_algorithm(result);
        }
    }
}

fn _create_directed_algo_button<T: DirectedStepAlgorithm<Node, Edge>>(
    game_state: &mut GameState,
    selected_idx_opt: Option<NodeIndex>,
    ui: &mut Ui,
    algo: T,
    button_name: &str,
) {
    if ui
        .add_enabled(
            matches!(selected_idx_opt, Some(_)) && game_state.ui_data.is_directed,
            Button::new(button_name),
        )
        .clicked()
    {
        if let Some(idx) = selected_idx_opt {
            let result = algo.get_result(&game_state.graph, idx);
            game_state.add_algorithm(result);
        }
    }
}

fn _create_undirected_algo_button<T: UndirectedStepAlgorithm<Node, Edge>>(
    game_state: &mut GameState,
    selected_idx_opt: Option<NodeIndex>,
    ui: &mut Ui,
    algo: T,
    button_name: &str,
) {
    if ui
        .add_enabled(
            matches!(selected_idx_opt, Some(_)) && !game_state.ui_data.is_directed,
            Button::new(button_name),
        )
        .clicked()
    {
        if let Some(idx) = selected_idx_opt {
            let graph_copy = game_state.graph.clone().into_edge_type::<Undirected>();
            let result = algo.get_result(&graph_copy, idx);
            game_state.add_algorithm(result);
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
    });
}

// Disable editing when algorithm is running, disable algorithm when editing
pub fn create_ui(game_state: &mut GameState, ctx: &mut Context, egui_ctx: &egui::CtxRef) {
    controls_ui(game_state, ctx, egui_ctx);
    if matches!(game_state.ui_data.state, UiState::Edit) {
        graph_editor_ui(game_state, ctx, egui_ctx);
    } else {
        algorithm_ui(game_state, ctx, egui_ctx);
    }
}
