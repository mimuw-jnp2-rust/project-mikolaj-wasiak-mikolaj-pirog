use egui_tetra::{egui, State};
use std::error::Error;
use tetra::graphics::mesh::ShapeStyle;
use tetra::graphics::{mesh::Mesh, Color};
use tetra::math::Vec2;
use tetra::Context;

const BASE_RADIUS: f32 = 20.0;
const BASE_BORDER_SIZE: f32 = 2.0;

pub struct Node {
    position: Vec2<f32>,
    radius: f32,
    border_color: Color,
    color: Color,
    shape: Mesh,
}

impl Node {
    pub fn new(ctx: &mut Context, position: Vec2<f32>) -> Result<Node, Box<dyn Error>> {
        Ok(Node {
            position,
            radius: BASE_RADIUS,
            border_color: Color::BLACK,
            color: Color::WHITE,
            shape: Mesh::circle(
                ctx,
                ShapeStyle::Stroke(BASE_BORDER_SIZE),
                Vec2 { x: 0.0, y: 0.0 },
                BASE_RADIUS,
            )?,
        })
    }
}

impl State<Box<dyn Error>> for Node {
    fn update(
        &mut self,
        _ctx: &mut Context,
        _egui_ctx: &egui::CtxRef,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _egui_ctx: &egui::CtxRef) -> Result<(), Box<dyn Error>> {
        self.shape.draw(ctx, self.position);
        Ok(())
    }
}
