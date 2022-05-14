use super::Position;
use egui_tetra::egui;
use std::error::Error;
use tetra::graphics::mesh::ShapeStyle;
use tetra::graphics::DrawParams;
use tetra::graphics::{mesh::Mesh, Color};
use tetra::math::Vec2;
use tetra::Context;

const BASE_RADIUS: f32 = 20.;
const BASE_BORDER_SIZE: f32 = 4.;
const HIGHLIGHT_SCALE: Vec2<f32> = Vec2 { x: 1.1, y: 1.1 };

pub enum NodeHighlight {
    Highlighted,
    Normal,
}

pub struct Node {
    position: Position,
    radius: f32,
    border_color: Color,
    color: Color,
    highlight: NodeHighlight,
    current_force: Position,

    // To change colors this has to be separate
    circle: Mesh,
    border: Mesh,
}

pub struct PushForceConfig {
    pub force: f32,
    pub distance: f32,
}

impl Node {
    pub fn new(ctx: &mut Context, position: Position) -> Result<Node, Box<dyn Error>> {
        Ok(Node {
            position,
            radius: BASE_RADIUS,
            border_color: Color::BLACK,
            color: Color::WHITE,
            current_force: Position::zero(),
            border: Mesh::circle(
                ctx,
                ShapeStyle::Stroke(BASE_BORDER_SIZE),
                Vec2 { x: 0.0, y: 0.0 },
                BASE_RADIUS,
            )?,
            circle: Mesh::circle(ctx, ShapeStyle::Fill, Vec2 { x: 0.0, y: 0.0 }, BASE_RADIUS)?,
            highlight: NodeHighlight::Normal,
        })
    }

    // Is point in this shape?
    pub fn contains(&self, point: Position) -> bool {
        Vec2::distance(point, self.position) <= self.radius
    }

    fn get_draw_params(&self, position: Position) -> DrawParams {
        DrawParams::new()
            .scale(
                if matches!(self.highlight, NodeHighlight::Highlighted) || self.contains(position) {
                    HIGHLIGHT_SCALE
                } else {
                    Vec2::one()
                },
            )
            .position(self.position)
    }

    pub fn set_highlight(&mut self, highlight: NodeHighlight) {
        self.highlight = highlight;
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn add_force(&mut self, force: Position) {
        self.current_force += force;
    }

    pub fn push_away_from_point(&mut self, point: Position, push_conf: &PushForceConfig) {
        let push_direction = (self.position() - point).normalized();
        let force_div = 1. - self.position().distance(point) / push_conf.distance;
        if force_div <= 0. {
            return;
        }
        self.current_force += push_direction * push_conf.force * force_div;
    }

    pub fn consume_force(&mut self, ctx: &mut Context) {
        self.position += self.current_force * tetra::time::get_delta_time(ctx).as_secs_f32();
        self.current_force = Position::zero();
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
