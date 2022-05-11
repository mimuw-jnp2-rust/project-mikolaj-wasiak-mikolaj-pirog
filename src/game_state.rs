use crate::input::input_state::InputState;
use egui_tetra::egui;
use petgraph::{Directed, Graph};
use std::error::Error;
use tetra::graphics::{self, Color, DrawParams, Texture};
use tetra::input::MouseButton;
use tetra::math::Vec2;
use tetra::{input, Context};

type Position = Vec2<f32>;

pub struct GameState {
    pub graph: Graph<Position, (), Directed, u32>,
    pub circle_textrue: Texture,
    pub input_state: InputState,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {
            graph: Graph::new(),
            circle_textrue: Texture::new(ctx, "resources/circle.jpg")?,
            input_state: InputState::Add,
        })
    }
}

impl egui_tetra::State<Box<dyn Error>> for GameState {
    fn draw(&mut self, ctx: &mut Context, _egui_ctx: &egui::CtxRef) -> Result<(), Box<dyn Error>> {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        for node in self.graph.raw_nodes() {
            self.circle_textrue.draw(
                ctx,
                DrawParams::new()
                    .position(node.weight)
                    .scale(Vec2::new(0.1, 0.1)),
            )
        }

        Ok(())
    }

    fn ui(&mut self, _ctx: &mut Context, egui_ctx: &egui::CtxRef) -> Result<(), Box<dyn Error>> {
        egui::Window::new("Graph editor").show(egui_ctx, |ui| {
            ui.heading("Mode");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.input_state, InputState::Add, "Add");
                ui.selectable_value(&mut self.input_state, InputState::Remove, "Remove");
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
            self.input_state
                .on_left_click(&mut self.graph, input::get_mouse_position(ctx));
        }
        Ok(())
    }
}
