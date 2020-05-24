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

    pub fn elapsed(&mut self) -> Duration {
        SystemTime::now().duration_since(self.start_time)
            .expect("Time is broken.")
    }

    pub fn check_and_reset(&mut self) -> Duration {
        let now = SystemTime::now();
        let elapsed = now.duration_since(self.start_time)
            .expect("Time is broken.");
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