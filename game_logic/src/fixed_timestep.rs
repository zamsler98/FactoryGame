/// Simulation ticks per second (UPS target).
pub const TICKS_PER_SECOND: u32 = 60;
/// Max ticks run in one frame; beyond this the backlog is dropped
/// and the game slows down instead of spiralling.
const MAX_TICKS_PER_FRAME: u32 = 5;

/// Converts variable frame time into a whole number of fixed ticks.
pub struct FixedTimestep {
    tick_dt: f64,
    acc: f64,
}

impl FixedTimestep {
    pub fn new(ticks_per_second: u32) -> Self {
        Self {
            tick_dt: 1.0 / ticks_per_second as f64,
            acc: 0.0,
        }
    }

    /// Adds frame_dt seconds and returns how many ticks to run now.
    #[must_use]
    pub fn advance(&mut self, frame_dt: f64) -> u32 {
        self.acc += frame_dt;
        let mut n = 0;
        while self.acc >= self.tick_dt && n < MAX_TICKS_PER_FRAME {
            self.acc -= self.tick_dt;
            n += 1;
        }
        if n == MAX_TICKS_PER_FRAME {
            self.acc = 0.0;
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_dt_runs_no_ticks() {
        let mut timestep = FixedTimestep::new(60);
        assert_eq!(timestep.advance(0.0), 0);
    }

    #[test]
    fn accumulates_across_frames() {
        let mut timestep = FixedTimestep::new(60);
        assert_eq!(timestep.advance(0.01), 0);
        assert_eq!(timestep.advance(0.01), 1);
    }

    #[test]
    fn runs_whole_ticks_only() {
        let mut timestep = FixedTimestep::new(60);
        assert_eq!(timestep.advance(2.5 / 60.0), 2);
    }

    #[test]
    fn caps_ticks_and_drops_backlog() {
        let mut timestep = FixedTimestep::new(60);
        assert_eq!(timestep.advance(10.0), 5);
        assert_eq!(timestep.advance(0.0), 0);
    }
}
