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

impl Display for StationMeasurements {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.min, self.mean(), self.max)
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
