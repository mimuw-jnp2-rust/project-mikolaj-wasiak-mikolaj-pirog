use tetra::graphics::Camera;

use crate::game_state::AppMode;
use crate::ui::ui_state::UiData;
use tetra::Context;

pub struct TetraObjectInfo {
    mode: AppMode,
    ui_data: UiData,
    camera: Camera,
}

impl TetraObjectInfo {
    pub fn new(mode: AppMode, ui_data: UiData, camera: Camera) -> TetraObjectInfo {
        TetraObjectInfo {
            mode,
            ui_data,
            camera,
        }
    }

    pub fn mode(&self) -> &AppMode {
        &self.mode
    }

    pub fn mode_mut(&mut self) -> &mut AppMode {
        &mut self.mode
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn ui_data(&self) -> &UiData {
        &self.ui_data
    }

    pub fn ui_data_mut(&mut self) -> &mut UiData {
        &mut self.ui_data
    }
}

pub trait TetraObject {
    fn draw(&mut self, ctx: &mut Context, info: &mut TetraObjectInfo);

    fn update(&mut self, ctx: &mut Context, info: &mut TetraObjectInfo);
}
