use egui_tetra::egui;

use std::f32;

use crate::game_state::{AppMode, FONT_SIZE};
use tetra::graphics::mesh::ShapeStyle;
use tetra::graphics::text::{Font, Text};
use tetra::graphics::{mesh::Mesh, Color};
use tetra::graphics::{Camera, DrawParams};
use tetra::input::Key;
use tetra::math::Vec2;
use tetra::{input, Context};

use super::gravity::PushForceConfig;
use super::Position;

use crate::tetra_handling::tetra_object::{TetraObject, TetraObjectInfo};

const BASE_RADIUS: f32 = 20.;
const BASE_BORDER_SIZE: f32 = 4.;
const HIGHLIGHT_SCALE: Vec2<f32> = Vec2 { x: 1.1, y: 1.1 };

#[derive(Clone)]
pub enum NodeHighlight {
    Highlighted,
    Normal,
}

#[derive(Clone)]
pub struct Node {
    position: Position,
    radius: f32,

    border_color: Color,
    color: Color,

    highlight: NodeHighlight,

    current_force: Position,
    ignore_force: bool,

    // To change colors this has to be separate
    circle: Mesh,
    border: Mesh,

    node_text: String,
    font: Font,
}

impl Node {
    // Adding font here is not perfect, but I don't see better solution. Creating font with every frame is expensive, since it has to cache the file again and again.
    // creating the font as a static is out of the question, since its ctro takes ctx as argument. Hence I believe it should be stored in gamestate and cloned every time
    // new nodes is created (cloning node is cheap, its just Rc under the hood).
    pub fn new(ctx: &mut Context, position: Position, font: Font) -> Node {
        Node {
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
            )
            .unwrap(),
            circle: Mesh::circle(ctx, ShapeStyle::Fill, Vec2 { x: 0.0, y: 0.0 }, BASE_RADIUS)
                .unwrap(),
            highlight: NodeHighlight::Normal,
            node_text: position.to_string(),
            font,
        }
    }

    // Is point in this shape?
    pub fn contains(&self, point: Position) -> bool {
        Vec2::distance(point, self.position) <= self.radius
    }

    fn draw_params(&self, position: Position) -> DrawParams {
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

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
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
        let mut direction_to = self.position() - point;
        if direction_to.is_approx_zero() {
            direction_to = Position::up().rotated_z(rand::random::<f32>());
        }
        let push_direction = direction_to.normalized();
        let force_div = 1. - self.position().distance(point) / push_conf.distance();

        if force_div <= 0. {
            return;
        }

        self.current_force += push_direction * push_conf.force() * force_div;
    }

    pub fn consume_force(&mut self, ctx: &mut Context) {
        if self.ignore_force {
            return;
        }

        self.position += self.current_force * tetra::time::get_delta_time(ctx).as_secs_f32();
        self.current_force = Position::zero();
    }

    pub fn set_ignore_force(&mut self, value: bool) {
        self.ignore_force = value;
        self.current_force = Position::zero();
    }

    pub fn draw_text(&mut self, ctx: &mut Context, rotation: f32, mouse_position: Vec2<f32>) {
        if self.node_text.is_empty() {
            return;
        }

        let mut text = Text::new(&self.node_text, self.font.clone());

        // This turns on text wrapping after BASE_RADIUS
        text.set_max_width(Some(BASE_RADIUS));

        let mut text_params = self.draw_params(mouse_position).color(Color::BLACK);

        // We set the origin to the center of the text, so rotation will behave nicely.
        text_params.origin = text.get_bounds(ctx).unwrap().bottom_right() / 2.;
        text_params.position = self.position;
        // We do not want the text to rotate.
        text_params.rotation = -rotation;
        text_params.scale /= FONT_SIZE;
        text.draw(ctx, text_params);
    }

    pub fn input(&mut self, ctx: &mut Context, mode: &mut AppMode) {
        if let Some(new_input) = input::get_text_input(ctx) {
            if self.node_text.len() <= 10 {
                self.node_text.push_str(new_input);
            }
        }

        if input::is_key_pressed(ctx, Key::Backspace) {
            self.node_text.pop();
        }

        if input::is_key_pressed(ctx, Key::Enter) {
            *mode = AppMode::Normal;
        }
    }
}

impl TetraObject for Node {
    fn draw(&mut self, ctx: &mut Context, info: &mut TetraObjectInfo) {
        let params = self.draw_params(info.camera().mouse_position(ctx));
        self.circle.draw(ctx, params.clone().color(self.color()));

        self.border.draw(ctx, params.color(self.border_color));

        self.draw_text(
            ctx,
            info.camera().rotation,
            info.camera().mouse_position(ctx),
        );
    }

    fn update(&mut self, ctx: &mut Context, info: &mut TetraObjectInfo) {
        if let AppMode::Write = info.mode() {
            if self.contains(info.camera().mouse_position(ctx)) {
                self.input(ctx, info.mode_mut());
            }
        }
    }
}
