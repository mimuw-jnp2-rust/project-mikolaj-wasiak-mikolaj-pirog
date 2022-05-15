use tetra::Context;

pub struct Timer {
    time_remaining: f32,
    active: bool,
    loops: bool,
    time: f32,
}

impl Timer {
    pub fn new(time: f32, loops: bool) -> Timer {
        Timer {
            time,
            active: false,
            loops,
            time_remaining: 0.,
        }
    }

    pub fn start(&mut self) {
        self.active = true;
        self.time_remaining = self.time;
    }

    pub fn stop(&mut self) {
        self.active = false;
        self.time_remaining = 0.;
    }

    fn finished(&mut self) -> bool {
        if self.active && self.time_remaining <= 0. {
            if self.loops {
                self.start();
            } else {
                self.stop();
            }
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, ctx: &mut Context) -> bool {
        if self.active {
            self.time_remaining -= tetra::time::get_delta_time(ctx).as_secs_f32();
            self.finished()
        } else {
            false
        }
    }
}
