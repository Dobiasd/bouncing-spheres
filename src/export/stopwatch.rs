use std::time::{Duration, SystemTime};

pub struct Stopwatch {
    start_time: SystemTime
}

impl Stopwatch {
    pub fn new() -> Stopwatch {
        Stopwatch {
            start_time: SystemTime::now()
        }
    }

    pub fn check_and_reset(&mut self) -> Duration {
        let now = SystemTime::now();
        let elapsed = now.duration_since(self.start_time)
            .expect("Time is broken.");
        self.start_time = now;
        return elapsed;
    }
}
