use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct StationMeasurements {
    count: u32,
    total: f32,
    min: f32,
    max: f32,
}

impl StationMeasurements {
    pub fn add_measurement(&mut self, measurement: f32) {
        self.count += 1;
        self.total += measurement;
        self.min = self.min.min(measurement);
        self.max = self.max.max(measurement);
    }

    pub fn mean(&self) -> f32 {
        self.total / self.count as f32
    }
}

fn round_float_up_1dp(f: f32) -> f32 {
    (f * 10.0).ceil() / 10.0
}

impl Display for StationMeasurements {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:.1}/{:.1}/{:.1}",
            round_float_up_1dp(self.min),
            round_float_up_1dp(self.mean()),
            round_float_up_1dp(self.max)
        )
    }
}

impl Default for StationMeasurements {
    fn default() -> Self {
        Self {
            count: 0,
            total: 0.0,
            min: f32::NAN,
            max: f32::NAN,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round() {
        assert_eq!(round_float_up_1dp(1.234), 1.3);
        assert_eq!(round_float_up_1dp(1.55), 1.6);
        assert_eq!(round_float_up_1dp(2.000001), 2.1);
        assert_eq!(round_float_up_1dp(-1.55), -1.5);
        assert_eq!(round_float_up_1dp(-0.05), 0.0);
    }
}
