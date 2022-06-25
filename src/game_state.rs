use std::error::Error;

use egui_tetra::egui;
use egui_tetra::egui::CtxRef;
use tetra::graphics::scaling::{ScalingMode, ScreenScaler};
use tetra::graphics::text::Font;
use tetra::graphics::{self, Camera, Color, FilterMode};

use tetra::input::MouseButton;
use tetra::Context;

use crate::camera_handling::camera_state::CameraState;
use crate::graph::{Graph, GraphOnCanvas};
use crate::input::input_state::{InputState, StateData};
use crate::step_algorithms::StepAlgorithmResult;
use crate::tetra_handling::tetra_object::{TetraObject, TetraObjectInfo};
use crate::ui::ui_drawing::create_ui;
use crate::ui::ui_state::UiData;

pub const SCREEN_WIDTH: f32 = 1280.;
pub const SCREEN_HEIGHT: f32 = 800.;

// This is necessary to render fonts correctly: when font is rendered "normally", ie at desired
// size and then we zoom in, the font becomes pixelated. To avoid this, font is
// rendered at much bigger size than needed, and then scaled down to desired size. This operations preserve font
// quality and result in font being very clear even at big blow up.
pub const FONT_SIZE: f32 = 10.;
pub const FONT_SIZE_SQUARED: f32 = FONT_SIZE * FONT_SIZE;

pub enum AppMode {
    Write,
    Normal,
}

pub struct GameState {
    pub graph: Graph,
    pub input_state: InputState,

    scaler: ScreenScaler,

    font: Font,

    tetra_info: TetraObjectInfo,

    algorithm: Option<StepAlgorithmResult>,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameState {
        GameState {
            graph: Graph::new(),
            input_state: InputState::Move(StateData::default()),
            tetra_info: TetraObjectInfo::new(
                AppMode::Normal,
                UiData::new(),
                Camera::new(SCREEN_WIDTH, SCREEN_HEIGHT),
            ),
            scaler: ScreenScaler::with_window_size(
                ctx,
                SCREEN_WIDTH as i32,
                SCREEN_HEIGHT as i32,
                ScalingMode::ShowAllPixelPerfect,
            )
            .unwrap(),
            algorithm: None,
            font: {
                let mut font = Font::vector(
                    ctx,
                    "resources/fonts/JetBrainsMono-Regular.ttf",
                    FONT_SIZE_SQUARED,
                )
                .unwrap();
                font.set_filter_mode(ctx, FilterMode::Linear);
                font
            },
        }
    }

    pub fn add_algorithm(&mut self, mut algorithm_res: StepAlgorithmResult) {
        algorithm_res.show_algorithm(&mut self.graph);
        self.algorithm = Some(algorithm_res);
    }

    pub fn font(&self) -> Font {
        self.font.clone()
    }

    pub fn tetra_info(&self) -> &TetraObjectInfo {
        &self.tetra_info
    }

    pub fn tetra_info_mut(&mut self) -> &mut TetraObjectInfo {
        &mut self.tetra_info
    }
}

impl egui_tetra::State<Box<dyn Error>> for GameState {
    fn ui(&mut self, ctx: &mut Context, egui_ctx: &CtxRef) -> Result<(), Box<dyn Error>> {
        create_ui(self, ctx, egui_ctx);

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context, _egui_ctx: &CtxRef) -> Result<(), Box<dyn Error>> {
        self.graph.update(ctx, &mut self.tetra_info);

        if let Some(alg) = &mut self.algorithm {
            alg.update(ctx, &mut self.graph);
        }

        if let AppMode::Normal = self.tetra_info.mode() {
            self.tetra_info
                .camera_mut()
                .update_camera_transformation(ctx)
        } else {
            Ok(())
        }
    }

    fn draw(&mut self, ctx: &mut Context, _egui_ctx: &egui::CtxRef) -> Result<(), Box<dyn Error>> {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        graphics::set_transform_matrix(ctx, self.tetra_info.camera().as_matrix());

        self.graph.draw(ctx, &mut self.tetra_info);

        graphics::reset_transform_matrix(ctx);

        self.scaler.draw(ctx);

        Ok(())
    }

    fn event(
        &mut self,
        ctx: &mut Context,
        _egui_ctx: &CtxRef,
        event: tetra::Event,
    ) -> Result<(), Box<dyn Error>> {
        if let tetra::Event::MouseMoved { .. } = &event {
            self.input_state.on_mouse_drag(
                ctx,
                &mut self.graph,
                self.tetra_info.camera().mouse_position(ctx),
            );
        }

        if let tetra::Event::MouseButtonPressed {
            button: MouseButton::Left,
        } = &event
        {
            self.input_state.on_left_click(
                ctx,
                &mut self.graph,
                self.tetra_info.camera().mouse_position(ctx),
                self.font.clone(),
            );
        }

        if let tetra::Event::MouseButtonPressed {
            button: MouseButton::Right,
        } = &event
        {
            if self
                .graph
                .node_from_point(self.tetra_info.camera().mouse_position(ctx))
                .is_some()
            {
                *self.tetra_info.mode_mut() = AppMode::Write;
            }
        }

        if self
            .graph
            .node_from_point(self.tetra_info.camera().mouse_position(ctx))
            .is_none()
        {
            *self.tetra_info.mode_mut() = AppMode::Normal;
        }

        if let AppMode::Normal = self.tetra_info.mode() {
            self.tetra_info.camera_mut().handle_camera_events(event)
        } else {
            Ok(())
        }
    }
}
