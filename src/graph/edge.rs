use egui_tetra::State;
use std::error::Error;
use tetra::graphics::{mesh::Mesh, Color, DrawParams};
use tetra::Context;

use super::Position;

const BASE_STROKE_WIDTH: f32 = 4.0;

pub struct Edge {
    from: Position,
    to: Position,
    color: Color,

    shape: Mesh,
}

impl Edge {
    pub fn new(ctx: &mut Context, from: Position, to: Position) -> Result<Edge, Box<dyn Error>> {
        Ok(Edge {
            from,
            to,
            color: Color::BLACK,
            shape: Mesh::polyline(ctx, BASE_STROKE_WIDTH, &[from, to])?,
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
        self.shape = Mesh::polyline(ctx, BASE_STROKE_WIDTH, &[from, to])?;
        Ok(())
    }

    fn get_draw_params(&self) -> DrawParams {
        DrawParams::new()
            .position(Position::zero())
            .color(self.color)
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
