use std::borrow::BorrowMut;

#[derive(Clone, Copy)]
pub struct PullForceConfig {
    min_distance: f32,
    force_at_twice_distance: f32,
}

impl PullForceConfig {
    pub fn new(min_distance: f32, force_at_twice_distance: f32) -> PullForceConfig {
        PullForceConfig {
            min_distance,
            force_at_twice_distance,
        }
    }

    pub fn min_distance(&self) -> f32 {
        self.min_distance
    }

    pub fn min_distance_mut(&mut self) -> &mut f32 {
        self.min_distance.borrow_mut()
    }

    pub fn force_at_twice_distance(&self) -> f32 {
        self.force_at_twice_distance
    }

    pub fn force_at_twice_distance_mut(&mut self) -> &mut f32 {
        self.force_at_twice_distance.borrow_mut()
    }
}

#[derive(Clone, Copy)]
pub struct PushForceConfig {
    force: f32,
    distance: f32,
}

impl PushForceConfig {
    pub fn new(force: f32, distance: f32) -> PushForceConfig {
        PushForceConfig { force, distance }
    }

    pub fn force(&self) -> f32 {
        self.force
    }

    pub fn force_mut(&mut self) -> &mut f32 {
        self.force.borrow_mut()
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }

    pub fn distance_mut(&mut self) -> &mut f32 {
        self.distance.borrow_mut()
    }
}
