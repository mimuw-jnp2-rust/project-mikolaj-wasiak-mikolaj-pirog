use std::error::Error;

use egui_tetra::egui;
use egui_tetra::egui::CtxRef;
use tetra::graphics::scaling::{ScalingMode, ScreenScaler};
use tetra::graphics::{self, Camera, Color};
use tetra::input::MouseButton;
use tetra::Context;

use crate::camera_handling::camera_state::CameraState;
use crate::graph::edge::PullForceConfig;
use crate::graph::node::PushForceConfig;
use crate::graph::{Graph, GraphOnCanvas};
use crate::input::input_state::InputState;
use crate::step_algorithms::algorithm::Algorithm;
use crate::ui::ui_drawing::graph_params_editor_ui;

pub const SCREEN_WIDTH: f32 = 640.;
pub const SCREEN_HEIGHT: f32 = 480.;

pub struct GameState {
    pub graph: Graph,
    pub input_state: InputState,
    pub camera: Camera,
    pub scaler: ScreenScaler,

    // This maybe should be under ui struct
    // But we don't have ui struct
    pub(crate) push_conf: PushForceConfig,
    pub(crate) pull_conf: PullForceConfig,

    pub(crate) algorithm: Option<Algorithm>,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {
            graph: Graph::new(),
            input_state: InputState::Add,
            camera: Camera::new(SCREEN_WIDTH, SCREEN_HEIGHT),
            scaler: ScreenScaler::with_window_size(
                ctx,
                SCREEN_WIDTH as i32,
                SCREEN_HEIGHT as i32,
                ScalingMode::ShowAllPixelPerfect,
            )?,
            push_conf: PushForceConfig::new(),
            pull_conf: PullForceConfig::new(),
            algorithm: None,
        })
    }
}

impl egui_tetra::State<Box<dyn Error>> for GameState {
    fn ui(&mut self, _ctx: &mut Context, egui_ctx: &CtxRef) -> Result<(), Box<dyn Error>> {
        graph_params_editor_ui(self, egui_ctx);

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context, egui_ctx: &CtxRef) -> Result<(), Box<dyn Error>> {
        self.graph
            .update(ctx, egui_ctx, &self.push_conf, &self.pull_conf)?;

        if let Some(alg) = &mut self.algorithm {
            alg.update(ctx, &mut self.graph);
        }

        self.camera.update_camera_transformation(ctx)
    }

    fn draw(&mut self, ctx: &mut Context, egui_ctx: &egui::CtxRef) -> Result<(), Box<dyn Error>> {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        graphics::set_transform_matrix(ctx, self.camera.as_matrix());

        self.graph
            .draw(self.camera.mouse_position(ctx), ctx, egui_ctx)?;

        graphics::reset_transform_matrix(ctx);

        self.scaler.draw(ctx);

        Ok(())
    }

    fn event(
        &mut self,
        ctx: &mut Context,
        _egui_ctx: &CtxRef,
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

        self.camera.handle_camera_events(event)
    }
}
