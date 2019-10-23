// derived from https://github.com/AngryLawyer/rust-sdl2-playground/blob/master/src/turning/ratelimiter.rs
use std::time::Instant;
use std::time::Duration;
use std::thread::sleep;

pub struct RateLimiter {
    fps: u32,
    last_instant: Instant,
}

impl RateLimiter {

    pub fn new(fps: u32) -> RateLimiter {
        RateLimiter {
            fps: fps,
            last_instant: Instant::now(),
        }
    }

    pub fn limit(&mut self) -> u32 {
        let elapsed_millis = self.last_instant.elapsed().as_millis() as u32;
        if elapsed_millis < (1000 / self.fps) {
            sleep(Duration::from_millis(((1000 / self.fps) - elapsed_millis) as u64))
        }
        self.last_instant = Instant::now();
        return elapsed_millis;
    }
}
