use std::error::Error;
use std::f32::consts::PI;

use egui_tetra::State;
use tetra::graphics::mesh::GeometryBuilder;
use tetra::graphics::{mesh::Mesh, Color, DrawParams};
use tetra::math::Vec2;
use tetra::Context;

use super::Position;

use super::gravity::PullForceConfig;

const BASE_STROKE_WIDTH: f32 = 5.;
const BASE_ARROW_SCALE: f32 = 0.7;
const BASE_ARROW_ARMS_SIZE: f32 = 25.;

pub const PUSH_FORCE_FORCE: f32 = 1000.;
pub const PUSH_FORCE_DISTANCE: f32 = 150.;

pub const PULL_FORCE_MIN_DISTANCE: f32 = 100.;
pub const PULL_FORCE_FORCE_AT_TWICE_DISTANCE: f32 = 500.;

#[derive(Clone)]
pub struct Edge {
    from: Position,
    to: Position,
    color: Color,
    enabled: bool,

    shape: Mesh,
}

impl Edge {
    fn create_arrow(ctx: &mut Context, from: Position, to: Position) -> Mesh {
        let (from, to) = (
            Position::lerp(from, to, (1. - BASE_ARROW_SCALE) / 2.),
            Position::lerp(from, to, (1. + BASE_ARROW_SCALE) / 2.),
        );
        let left_arrow_point =
            (to - from).rotated_z(PI * 3. / 4.).normalized() * BASE_ARROW_ARMS_SIZE + to;
        let right_arrow_point =
            (to - from).rotated_z(-PI * 3. / 4.).normalized() * BASE_ARROW_ARMS_SIZE + to;
        let mut builder = GeometryBuilder::new();

        builder.polyline(BASE_STROKE_WIDTH, &[from, to]).unwrap();
        builder
            .polyline(
                BASE_STROKE_WIDTH,
                &[left_arrow_point, to, right_arrow_point],
            )
            .unwrap();
        builder.build_mesh(ctx).unwrap()
    }

    pub fn new(ctx: &mut Context, from: Position, to: Position) -> Edge {
        Edge {
            from,
            to,
            color: Color::BLACK,
            shape: Edge::create_arrow(ctx, from, to),
            enabled: true,
        }
    }

    pub fn update_position(&mut self, ctx: &mut Context, from: Position, to: Position) {
        self.from = from;
        self.to = to;
        self.shape = Edge::create_arrow(ctx, from, to);
    }

    pub fn disable_edge(&mut self) {
        self.enabled = false;
        self.color.a = 0.5;
    }

    pub fn enable_edge(&mut self) {
        self.reset_state();
    }

    pub fn reset_state(&mut self) {
        self.enabled = true;
        self.color.a = 1.0;
    }

    fn get_draw_params(&self) -> DrawParams {
        DrawParams::new()
            // What is the purpose of this line? After disabling it, the program behaves the same
            .position(Position::zero())
            .color(self.color)
    }

    pub fn calculate_pull_force(&self, config: &PullForceConfig) -> Position {
        if !self.enabled {
            return Position::zero();
        }

        let distance = self.from.distance(self.to);

        if distance < config.min_distance() {
            Position::zero()
        } else {
            let direction = (self.to - self.from).normalized();
            let force_value =
                (distance / config.min_distance() - 1.) * config.force_at_twice_distance();
            direction * force_value
        }
    }

    pub fn is_point_in_shape(&self, point: Vec2<f32>) -> bool {
        // We have to make sure that the point is between the lines,
        // otherwise it would be possible to remove edge by clicking anywhere along the line (from, to)
        // since triangle area check would yield zero.
        if !((point.ge(&self.from) && point.le(&self.to))
            || (point.ge(&self.to) && point.le(&self.from)))
        {
            return false;
        }

        let max_triangle_area =
            Vec2::triangle_area(self.from, self.to, self.from + 1.5 * BASE_STROKE_WIDTH);

        let triangle_area = Vec2::triangle_area(self.from, self.to, point);

        if triangle_area <= max_triangle_area {
            return true;
        }

        false
    }
}

// todo think if this should be a tetra state. I believe we should implement a
// trait Drawable that will require a drawing function from a struct.
// Using tetra's State seems like a overkill, because we have to implement
// dummy functions we never used, just to satisfy the constraint.
// It seems like there is no benefit in using tetra's state, other than consistency and using libary feature
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
