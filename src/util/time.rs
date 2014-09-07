extern crate time;

use self::time::precise_time_ns;

/// Deterministic fixed timestep progression
/// Based on http://gafferongames.com/game-physics/fix-your-timestep/
pub struct FixedTimestep {
    t: u64, // Elapsed time
    dt: u64, // Timestep (constant)
    current_time_ns: u64, // Measured time
    accumulator: u64, // Generated time to be consumed
}

impl FixedTimestep {
    pub fn new(dt: u64) -> FixedTimestep {
        FixedTimestep {
            t: 0,
            dt: dt,
            current_time_ns: precise_time_ns(),
            accumulator: 0,
        }
    }

    pub fn tick(&mut self, step: |u64, u64|) {
        let new_time = precise_time_ns();
        let frame_time = new_time - self.current_time_ns; // Optionally clip

        self.current_time_ns = new_time;
        self.accumulator += frame_time;

        while self.accumulator >= self.dt {
            step(self.t, self.dt);

            self.t += self.dt;
            self.accumulator -= self.dt;
        }
    }

    pub fn t(&self) -> u64 {
        self.t
    }

    pub fn dt(&self) -> u64 {
        self.dt
    }

    pub fn current_time_ns(&self) -> u64 {
        self.current_time_ns
    }

    pub fn accumulator(&self) -> u64 {
        self.accumulator
    }
}

/// Counts frames over time measured as Frames per Second
pub struct FpsCounter {
    frames: u64,
    current_frames: u64, // Measured no of frames
    current_time_ns: u64, // Measured time
}

impl FpsCounter {
    pub fn new() -> FpsCounter {
        FpsCounter {
            frames: 0,
            current_frames: 0,
            current_time_ns: precise_time_ns(),
        }
    }

    pub fn frame(&mut self) {
        self.frames += 1;
    }

    pub fn fps(&mut self) -> f32 {
        let new_frames = self.frames;
        let new_time_ns = precise_time_ns();

        let df = new_frames - self.current_frames;
        let dt_ns = new_time_ns - self.current_time_ns;

        let fps = df as f32 / dt_ns.to_seconds();

        self.current_frames = new_frames;
        self.current_time_ns = new_time_ns;

        fps
    }
}

/// Interprets self as nanoseconds
pub trait ToNanoSeconds {
    fn to_nanoseconds(&self) -> u64;
}

/// Interprets self as as seconds
pub trait ToSeconds {
    fn to_seconds(&self) -> f32;
}

/// Interprets f32 seconds as u64 nanoseconds - this needs a semantic type!
impl ToNanoSeconds for f32 {
    fn to_nanoseconds(&self) -> u64 {
        (*self * 1_000_000_000f32) as u64
    }
}

/// Interprets u64 nanoseconds as f32 seconds - this needs a semantic type!
impl ToSeconds for u64 {
    fn to_seconds(&self) -> f32 {
        *self as f32 / 1_000_000_000f32
    }
}