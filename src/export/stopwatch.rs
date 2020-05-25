use std::time::{Duration, Instant};

pub struct Stopwatch {
    start_time: Instant
}

impl Stopwatch {
    pub fn new() -> Stopwatch {
        Stopwatch {
            start_time: Instant::now()
        }
    }

    pub fn elapsed(&mut self) -> Duration {
        Instant::now().duration_since(self.start_time)
    }

    pub fn check_and_reset(&mut self) -> Duration {
        let now = Instant::now();
        let elapsed = now.duration_since(self.start_time);
        self.start_time = now;
        return elapsed;
    }
}

pub fn measure<F, Res>(mut f: F) -> (Res, Duration)
    where
        F: FnMut() -> Res,
{
    let mut stopwatch = Stopwatch::new();
    let result = f();
    (result, stopwatch.elapsed())
}