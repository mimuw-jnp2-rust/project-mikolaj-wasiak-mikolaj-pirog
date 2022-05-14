use std::error::Error;
use std::f32::consts::PI;

use egui_tetra::State;
use tetra::graphics::mesh::GeometryBuilder;
use tetra::graphics::{mesh::Mesh, Color, DrawParams};
use tetra::Context;

use super::Position;

const BASE_STROKE_WIDTH: f32 = 5.;
const BASE_ARROW_SCALE: f32 = 0.7;
const BASE_ARROW_ARMS_SIZE: f32 = 25.;

pub struct Edge {
    from: Position,
    to: Position,
    color: Color,

    shape: Mesh,
}

pub struct PullForceConfig {
    pub min_distance: f32,
    pub force_at_twice_distance: f32,
}

impl Edge {
    fn create_arrow(
        ctx: &mut Context,
        from: Position,
        to: Position,
    ) -> Result<Mesh, tetra::TetraError> {
        let (from, to) = (
            Position::lerp(from, to, (1. - BASE_ARROW_SCALE) / 2.),
            Position::lerp(from, to, (1. + BASE_ARROW_SCALE) / 2.),
        );
        let left_arrow_point =
            (to - from).rotated_z(PI * 3. / 4.).normalized() * BASE_ARROW_ARMS_SIZE + to;
        let right_arrow_point =
            (to - from).rotated_z(-PI * 3. / 4.).normalized() * BASE_ARROW_ARMS_SIZE + to;
        let mut builder = GeometryBuilder::new();
        builder.polyline(BASE_STROKE_WIDTH, &[from, to])?;
        builder.polyline(
            BASE_STROKE_WIDTH,
            &[left_arrow_point, to, right_arrow_point],
        )?;
        builder.build_mesh(ctx)
    }

    pub fn new(ctx: &mut Context, from: Position, to: Position) -> Result<Edge, Box<dyn Error>> {
        Ok(Edge {
            from,
            to,
            color: Color::BLACK,
            shape: Edge::create_arrow(ctx, from, to)?,
        })
    }

    pub fn update_position(
        &mut self,
        ctx: &mut Context,
        from: Position,
        to: Position,
    ) -> Result<(), Box<dyn Error>> {
        self.from = from;
        self.to = to;
        self.shape = Edge::create_arrow(ctx, from, to)?;
        Ok(())
    }

    fn get_draw_params(&self) -> DrawParams {
        DrawParams::new()
            .position(Position::zero())
            .color(self.color)
    }

    pub fn calculate_pull_force(&self, config: &PullForceConfig) -> Position {
        let distance = self.from.distance(self.to);
        if distance < config.min_distance {
            Position::zero()
        } else {
            let direction = (self.to - self.from).normalized();
            let force_value =
                (distance / config.min_distance - 1.) * config.force_at_twice_distance;
            direction * force_value
        }
    }
}

impl State<Box<dyn Error>> for Edge {
    fn update(
        &mut self,
        _ctx: &mut Context,
        _egui_ctx: &egui_tetra::egui::CtxRef,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn draw(
        &mut self,
        ctx: &mut Context,
        _egui_ctx: &egui_tetra::egui::CtxRef,
    ) -> Result<(), Box<dyn Error>> {
        self.shape.draw(ctx, self.get_draw_params());
        Ok(())
    }
}
