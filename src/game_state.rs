use std::error::Error;
use std::ops::{Add, AddAssign};
use std::ops::{Deref, DerefMut};

use egui_tetra::egui;
use egui_tetra::egui::{CtxRef, Vec2};
use tetra::graphics::scaling::{ScalingMode, ScreenScaler};
use tetra::graphics::{self, Camera, Color, Texture};
use tetra::input::{Key, MouseButton};
use tetra::Context;

use crate::algo::{Dfs, ShowAlgorithm};
use crate::camera_event;
use crate::graph::edge::PullForceConfig;
use crate::graph::node::Node;
use crate::graph::node::PushForceConfig;
use crate::graph::{Graph, GraphOnCanvas};
use crate::input::input_state::{ConnectData, InputState, MoveData};

pub const SCREEN_WIDTH: f32 = 640.;
pub const SCREEN_HEIGHT: f32 = 480.;
const ROTATION_SPEED: f32 = 0.05;

pub struct GameState {
    pub graph: Graph,
    pub circle_textrue: Texture,
    pub input_state: InputState,
    pub camera: Camera,
    pub scaler: ScreenScaler,

    // This maybe should be under ui struct
    // But we don't have ui struct
    push_conf: PushForceConfig,
    pull_conf: PullForceConfig,

    algorithm: Option<Box<dyn ShowAlgorithm>>,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {
            graph: Graph::new(),
            circle_textrue: Texture::new(ctx, "resources/circle.jpg")?,
            input_state: InputState::Add,
            camera: Camera::new(SCREEN_WIDTH, SCREEN_HEIGHT),
            scaler: ScreenScaler::with_window_size(
                ctx,
                SCREEN_WIDTH as i32,
                SCREEN_HEIGHT as i32,
                ScalingMode::ShowAllPixelPerfect,
            )?,
            push_conf: PushForceConfig {
                force: 1000.,
                distance: 150.,
            },
            pull_conf: PullForceConfig {
                min_distance: 100.,
                force_at_twice_distance: 500.,
            },
            algorithm: None,
        })
    }
}

impl egui_tetra::State<Box<dyn Error>> for GameState {
    fn draw(&mut self, ctx: &mut Context, egui_ctx: &egui::CtxRef) -> Result<(), Box<dyn Error>> {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        graphics::set_transform_matrix(ctx, self.camera.as_matrix());

        for edge in self.graph.edge_weights_mut() {
            edge.draw(ctx, egui_ctx)?;
        }
        for node in self.graph.node_weights_mut() {
            node.draw(ctx, egui_ctx, self.camera.mouse_position(ctx))?;
        }

        graphics::reset_transform_matrix(ctx);

        self.scaler.draw(ctx);

        Ok(())
    }

    fn ui(&mut self, _ctx: &mut Context, egui_ctx: &egui::CtxRef) -> Result<(), Box<dyn Error>> {
        egui::Window::new("Graph editor").show(egui_ctx, |ui| {
            ui.heading("Mode");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.input_state, InputState::Add, "Add");
                ui.selectable_value(&mut self.input_state, InputState::Remove, "Remove");
            });
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.input_state,
                    InputState::Connect(ConnectData::default()),
                    "Connect",
                );
                ui.selectable_value(
                    &mut self.input_state,
                    InputState::Move(MoveData::default()),
                    "Move",
                );
            });
            ui.heading("Forces");
            ui.label("Push:");
            ui.horizontal(|ui| {
                ui.label("Value");
                ui.add(egui::DragValue::new(&mut self.push_conf.force));
            });
            ui.horizontal(|ui| {
                ui.label("Distance");
                ui.add(egui::DragValue::new(&mut self.push_conf.distance));
            });
            ui.label("Pull:");
            ui.horizontal(|ui| {
                ui.label("Value");
                ui.add(egui::DragValue::new(
                    &mut self.pull_conf.force_at_twice_distance,
                ));
            });
            ui.horizontal(|ui| {
                ui.label("Min Distance");
                ui.add(egui::DragValue::new(&mut self.pull_conf.min_distance));
            });
            if ui.button("dfs").clicked() {
                self.algorithm = Some(Box::new(Dfs::new()));
                if let Some(idx) = self.graph.node_indices().next() {
                    if let Some(algo) = &mut self.algorithm {
                        self.graph.move_node(_ctx, idx, self.camera.position);
                        algo.deref_mut().run_algorithm(&mut self.graph, idx);
                    }
                }
            }
        });
        Ok(())
    }

    fn event(
        &mut self,
        ctx: &mut tetra::Context,
        _egui_ctx: &egui::CtxRef,
        event: tetra::Event,
    ) -> Result<(), Box<dyn Error>> {
        if let tetra::Event::MouseMoved { .. } = &event {
            self.input_state.on_mouse_drag(
                ctx,
                &mut self.graph,
                self.camera.mouse_position(ctx),
            )?;
        }

        if let tetra::Event::MouseButtonPressed {
            button: MouseButton::Left,
        } = &event
        {
            self.input_state.on_left_click(
                ctx,
                &mut self.graph,
                self.camera.mouse_position(ctx),
            )?;
        }
        camera_event::handle_camera_events(self, event);

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context, egui_ctx: &CtxRef) -> Result<(), Box<dyn Error>> {
        self.graph
            .update(ctx, egui_ctx, &self.push_conf, &self.pull_conf)?;

        if let Some(alg) = &mut self.algorithm {
            alg.update(ctx, &mut self.graph);
        }

        if tetra::input::is_key_down(ctx, Key::Q) {
            self.camera.rotation += ROTATION_SPEED;
        }
        if tetra::input::is_key_down(ctx, Key::E) {
            self.camera.rotation -= ROTATION_SPEED;
        }

        self.camera.update();

        Ok(())
    }
}
