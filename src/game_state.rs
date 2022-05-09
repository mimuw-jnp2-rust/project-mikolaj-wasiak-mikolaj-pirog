use petgraph::{Directed, Graph};
use tetra::graphics::{self, Color, DrawParams, Texture};
use tetra::math::Vec2;
use tetra::{input, Context, State};

type Position = Vec2<f32>;

pub struct GameState {
    pub graph: Graph<Position, (), Directed, u32>,
    pub circle_textrue: Texture,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {
            graph: Graph::new(),
            circle_textrue: Texture::new(ctx, "resources/circle.jpg")?,
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
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
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_mouse_button_down(ctx, input::MouseButton::Left) {
            let position = input::get_mouse_position(ctx).round();
            self.graph.add_node(position);
        }
        Ok(())
    }
}
