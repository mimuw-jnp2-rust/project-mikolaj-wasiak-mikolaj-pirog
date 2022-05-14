use crate::graph::node::Node;
use crate::graph::Position;
use crate::input::input_state::{ConnectData, InputState};
use egui_tetra::egui;
use egui_tetra::egui::CtxRef;
use petgraph::{Directed, Graph};
use std::error::Error;
use tetra::graphics::scaling::{ScalingMode, ScreenScaler};
use tetra::graphics::{self, Camera, Color, Texture};
use tetra::input::{Key, MouseButton};
use tetra::Context;

pub const SCREEN_WIDTH: f32 = 640.;
pub const SCREEN_HEIGHT: f32 = 480.;
pub const CAMERA_ZOOM_SPEED: f32 = 0.05;
const ROTATION_SPEED: f32 = 0.05;
const Y_AXIS_MOVE_SPEED: f32 = 10.;
const X_AXIS_MOVE_SPEED: f32 = 10.;

pub struct GameState {
    pub graph: Graph<Node, (), Directed, u32>,
    pub circle_textrue: Texture,
    pub input_state: InputState,
    pub camera: Camera,
    scaler: ScreenScaler,
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
        })
    }
}

impl egui_tetra::State<Box<dyn Error>> for GameState {
    fn draw(&mut self, ctx: &mut Context, egui_ctx: &egui::CtxRef) -> Result<(), Box<dyn Error>> {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        graphics::set_transform_matrix(ctx, self.camera.as_matrix());

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
                ui.selectable_value(&mut self.input_state, InputState::Move, "Move");
            });
        });
        Ok(())
    }

    fn event(
        &mut self,
        ctx: &mut tetra::Context,
        _egui_ctx: &egui::CtxRef,
        event: tetra::Event,
    ) -> Result<(), Box<dyn Error>> {
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

        //todo move capturing camera related input to appropriate function.

        // Only y coordinate is accessed because x corresponds to horizontal move of mouse wheel.
        if let tetra::Event::MouseWheelMoved { amount } = &event {
            if amount.y > 0 {
                self.camera.scale += CAMERA_ZOOM_SPEED;
            } else {
                self.camera.scale -= CAMERA_ZOOM_SPEED;
            }
        }

        //todo take into account rotation so moving will be absolute
        if let tetra::Event::KeyPressed { key: Key::W } = &event {
            self.camera.position +=
                Position::unit_y().rotated_z(-self.camera.rotation) * Y_AXIS_MOVE_SPEED;
        }

        if let tetra::Event::KeyPressed { key: Key::S } = &event {
            self.camera.position -=
                Position::unit_y().rotated_z(-self.camera.rotation) * Y_AXIS_MOVE_SPEED;
        }

        if let tetra::Event::KeyPressed { key: Key::A } = &event {
            self.camera.position +=
                Position::unit_x().rotated_z(-self.camera.rotation) * Y_AXIS_MOVE_SPEED;
        }

        if let tetra::Event::KeyPressed { key: Key::D } = &event {
            self.camera.position -=
                Position::unit_x().rotated_z(-self.camera.rotation) * Y_AXIS_MOVE_SPEED;
        }

        self.camera.update();

        if let tetra::Event::Resized { width, height } = event {
            self.scaler.set_outer_size(width, height);
        }

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context, egui_ctx: &CtxRef) -> Result<(), Box<dyn Error>> {
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
