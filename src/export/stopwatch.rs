use std::time::{Duration, Instant};

pub trait Stopwatch {
    fn check_and_reset(&mut self) -> Duration;
}

impl Stopwatch for Instant {
    fn check_and_reset(&mut self) -> Duration {
        let now = Instant::now();
        let elapsed = now.duration_since(*self);
        *self = now;
        return elapsed;
    }
}

pub fn measure<F, Res>(mut f: F) -> (Res, Duration)
    where
        F: FnMut() -> Res,
{
    let start = Instant::now();
    let result = f();
    (result, start.elapsed())
}