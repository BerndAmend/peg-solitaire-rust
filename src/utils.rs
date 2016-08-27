use std::default::Default;
use std::fmt;
use std::time::Instant;

pub struct Stopwatch {
    start: Instant,
}

impl Default for Stopwatch {
    fn default() -> Stopwatch {
        Stopwatch {
            start: Instant::now(),
        }
    }
}

impl Stopwatch {
    fn elapsed_ms(&self) -> i64 {
        let elapsed = self.start.elapsed();
        (elapsed.as_secs() * 1000 + (elapsed.subsec_nanos() / 1000000) as u64) as i64
    }
}

impl fmt::Display for Stopwatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}ms", self.elapsed_ms());
    }
}
