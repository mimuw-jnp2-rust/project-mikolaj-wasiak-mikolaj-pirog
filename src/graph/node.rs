use egui_tetra::{egui, State};
use std::error::Error;
use tetra::graphics::mesh::ShapeStyle;
use tetra::graphics::DrawParams;
use tetra::graphics::{mesh::Mesh, Camera, Color};
use tetra::input::get_mouse_position;
use tetra::math::Vec2;
use tetra::Context;

const BASE_RADIUS: f32 = 20.0;
const BASE_BORDER_SIZE: f32 = 4.0;
const HIGHLIGHT_SCALE: Vec2<f32> = Vec2 { x: 1.1, y: 1.1 };

pub struct Node {
    position: Vec2<f32>,
    radius: f32,
    border_color: Color,
    color: Color,

    // To change colors this has to be separate
    circle: Mesh,
    border: Mesh,
}

impl Node {
    pub fn new(ctx: &mut Context, position: Vec2<f32>) -> Result<Node, Box<dyn Error>> {
        Ok(Node {
            position,
            radius: BASE_RADIUS,
            border_color: Color::BLACK,
            color: Color::WHITE,
            border: Mesh::circle(
                ctx,
                ShapeStyle::Stroke(BASE_BORDER_SIZE),
                Vec2 { x: 0.0, y: 0.0 },
                BASE_RADIUS,
            )?,
            circle: Mesh::circle(ctx, ShapeStyle::Fill, Vec2 { x: 0.0, y: 0.0 }, BASE_RADIUS)?,
        })
    }

    // Is point in this shape?
    pub fn contains(&self, point: Vec2<f32>) -> bool {
        Vec2::distance(point, self.position) <= self.radius
    }

    fn get_draw_params(&self, position: Vec2<f32>) -> DrawParams {
        DrawParams::new()
            .scale(if self.contains(position) {
                HIGHLIGHT_SCALE
            } else {
                Vec2::one()
            })
            .position(self.position)
    }
}

impl Node {
    fn update(
        &mut self,
        _ctx: &mut Context,
        _egui_ctx: &egui::CtxRef,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        _egui_ctx: &egui::CtxRef,
        mouse_position: Vec2<f32>,
    ) -> Result<(), Box<dyn Error>> {
        let params = self.get_draw_params(mouse_position);
        self.circle.draw(ctx, params.color(self.color));
        let params = self.get_draw_params(mouse_position);
        self.border.draw(ctx, params.color(self.border_color));
        Ok(())
    }
}
