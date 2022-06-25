use tetra::math::vec::Vec2;

pub const CAMERA_ZOOM_SPEED: f32 = 0.05;
pub const Y_AXIS_MOVE_SPEED: f32 = 7.;
pub const X_AXIS_MOVE_SPEED: f32 = 7.;
pub const ROTATION_SPEED: f32 = 0.05;

pub const BASE_STROKE_WIDTH: f32 = 5.;
pub const BASE_ARROW_SCALE: f32 = 0.7;
pub const BASE_ARROW_ARMS_SIZE: f32 = 25.;

pub const PUSH_FORCE_FORCE: f32 = 1000.;
pub const PUSH_FORCE_DISTANCE: f32 = 150.;

pub const PULL_FORCE_MIN_DISTANCE: f32 = 100.;
pub const PULL_FORCE_FORCE_AT_TWICE_DISTANCE: f32 = 500.;

pub const BASE_RADIUS: f32 = 20.;
pub const BASE_BORDER_SIZE: f32 = 4.;
pub const HIGHLIGHT_SCALE: Vec2<f32> = Vec2 { x: 1.1, y: 1.1 };

pub const SCREEN_WIDTH: f32 = 1280.;
pub const SCREEN_HEIGHT: f32 = 800.;

// This is necessary to render fonts correctly: when font is rendered "normally", ie at desired
// size and then we zoom in, the font becomes pixelated. To avoid this, font is
// rendered at much bigger size than needed, and then scaled down to desired size. This operations preserve font
// quality and result in font being very clear even at big blow up.
pub const FONT_SIZE: f32 = 10.;
pub const FONT_SIZE_SQUARED: f32 = FONT_SIZE * FONT_SIZE;
