use std::error::Error;

use crate::graph::edge::{PUSH_FORCE_DISTANCE, PUSH_FORCE_FORCE};
use egui_tetra::egui;
use tetra::graphics::mesh::ShapeStyle;
use tetra::graphics::DrawParams;
use tetra::graphics::{mesh::Mesh, Color};
use tetra::math::Vec2;
use tetra::Context;

use super::Position;

const BASE_RADIUS: f32 = 20.;
const BASE_BORDER_SIZE: f32 = 4.;
const HIGHLIGHT_SCALE: Vec2<f32> = Vec2 { x: 1.1, y: 1.1 };

pub enum NodeHighlight {
    Highlighted,
    Normal,
}

pub enum NodeState {
    Visited,
    Queued,
    NotVisited,
}

pub struct Node {
    position: Position,
    radius: f32,

    border_color: Color,
    color: Color,

    highlight: NodeHighlight,
    algorithm_state: NodeState,

    current_force: Position,
    ignore_force: bool,

    // To change colors this has to be separate
    circle: Mesh,
    border: Mesh,
}

#[derive(Default)]
pub struct PushForceConfig {
    force: f32,
    distance: f32,
}

impl PushForceConfig {
    pub fn new() -> PushForceConfig {
        PushForceConfig {
            force: PUSH_FORCE_FORCE,
            distance: PUSH_FORCE_DISTANCE,
        }
    }

    pub fn force(&self) -> f32 {
        self.force
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }
}

impl Node {
    pub fn new(ctx: &mut Context, position: Position) -> Result<Node, Box<dyn Error>> {
        Ok(Node {
            position,
            radius: BASE_RADIUS,
            border_color: Color::BLACK,
            color: Color::WHITE,
            current_force: Position::zero(),
            ignore_force: false,
            border: Mesh::circle(
                ctx,
                ShapeStyle::Stroke(BASE_BORDER_SIZE),
                Vec2 { x: 0.0, y: 0.0 },
                BASE_RADIUS,
            )?,
            algorithm_state: NodeState::NotVisited,
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
        if self.ignore_force {
            return;
        }

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
        self.circle.draw(ctx, params.clone().color(self.color));
        //let params = self.get_draw_params(mouse_position); //todo think if cloning is better than double declaration of the same thing.
        self.border.draw(ctx, params.color(self.border_color));

        Ok(())
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_state(&self) -> &NodeState {
        &self.algorithm_state
    }

    pub fn set_state(&mut self, state: NodeState) {
        self.algorithm_state = state;
        self.color = match self.algorithm_state {
            NodeState::Queued => Color::rgb(0.01, 0.1, 0.5),
            NodeState::Visited => Color::rgb(0.01, 0.9, 0.),
            NodeState::NotVisited => Color::WHITE,
        };

        println!(
            "New color {}, {}, {}",
            self.color.r, self.color.g, self.color.b
        );
    }

    pub fn set_ignore_force(&mut self, value: bool) {
        self.ignore_force = value;
        self.current_force = Position::zero();
    }
}
