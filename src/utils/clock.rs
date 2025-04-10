use std::{
    thread::sleep,
    time::{Duration, Instant},
};

pub struct FrameClock {
    target_frametime: Duration,
    last_instant: Option<Instant>,
}

impl FrameClock {
    pub fn new(target_framerate: f64) -> Self {
        Self {
            target_frametime: Duration::from_secs_f64(1.0 / target_framerate),
            last_instant: None,
        }
    }

    pub fn wait_next_frame(&mut self) -> Duration {
        let mut delta = Duration::ZERO;
        if let Some(last_instant) = self.last_instant {
            delta = Instant::now() - last_instant;
            sleep(self.target_frametime.saturating_sub(delta));
        }

        self.last_instant = Some(Instant::now());

        delta
    }
}
