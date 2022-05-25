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

    pub fn force_at_twice_distance(&self) -> f32 {
        self.force_at_twice_distance
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

    pub fn distance(&self) -> f32 {
        self.distance
    }
}
