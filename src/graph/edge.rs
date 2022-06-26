use std::f32::consts::PI;

use tetra::graphics::mesh::GeometryBuilder;
use tetra::graphics::{mesh::Mesh, Color, DrawParams};
use tetra::math::Vec2;
use tetra::Context;

use super::Position;

use super::gravity::PullForceConfig;

use crate::constants::{BASE_ARROW_ARMS_SIZE, BASE_ARROW_SCALE, BASE_STROKE_WIDTH};

use crate::tetra_handling::tetra_object::{TetraObject, TetraObjectInfo};

#[derive(Clone)]
pub struct Edge {
    from: Position,
    to: Position,
    color: Color,
    enabled: bool,
    reversed: bool,

    arrow: Mesh,
    line: Mesh,
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
            reversed: false,
            color: Color::BLACK,
            arrow: Edge::create_arrow(ctx, from, to),
            line: Mesh::polyline(ctx, BASE_STROKE_WIDTH, &[from, to]).unwrap(),
            enabled: true,
        }
    }

    pub fn update_position(&mut self, ctx: &mut Context, from: Position, to: Position) {
        self.from = from;
        self.to = to;
        if !self.reversed {
            self.arrow = Edge::create_arrow(ctx, from, to);
        } else {
            self.arrow = Edge::create_arrow(ctx, to, from);
        }
        self.line = Mesh::polyline(ctx, BASE_STROKE_WIDTH, &[from, to]).unwrap();
    }

    pub fn reverse(&mut self) {
        self.reversed = !self.reversed;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        self.color.a = 0.3;
    }

    pub fn enable(&mut self) {
        self.enabled = true;
        self.color.a = 1.0;
    }

    pub fn reset_state(&mut self) {
        self.reversed = false;
        self.enabled = true;
        self.color.a = 1.0;
    }

    fn draw_params(&self) -> DrawParams {
        DrawParams::new()
            // What is the purpose of this line? After disabling it, the program behaves the same // I still do not know
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

impl TetraObject for Edge {
    fn draw(&mut self, ctx: &mut Context, info: &mut TetraObjectInfo) {
        if info.ui_data().directed() {
            self.arrow.draw(ctx, self.draw_params());
        } else {
            self.line.draw(ctx, self.draw_params());
        }
    }

    fn update(&mut self, _ctx: &mut Context, _info: &mut TetraObjectInfo) {}
}
