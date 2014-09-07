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
        let frame_time = new_time - self.current_time_ns;

        // Optional: clip frame_time

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