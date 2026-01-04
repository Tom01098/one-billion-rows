use clap::Parser;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use memmap2::Mmap;

#[derive(Parser)]
struct Args {
    file: String,
}

#[derive(Debug)]
struct StationMeasurement {
    count: u32,
    total: f32,
    min: f32,
    max: f32,
}

impl StationMeasurement {
    fn add_measurement(&mut self, measurement: f32) {
        self.count += 1;
        self.total += measurement;
        self.min = self.min.min(measurement);
        self.max = self.max.max(measurement);
    }

    fn mean(&self) -> f32 {
        self.total / self.count as f32
    }
}

impl Display for StationMeasurement {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.min, self.mean(), self.max)
    }
}

impl Default for StationMeasurement {
    fn default() -> Self {
        Self {
            count: 0,
            total: 0.0,
            min: f32::NAN,
            max: f32::NAN,
        }
    }
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.file).unwrap();
    let mmap = unsafe { Mmap::map(&file).unwrap() };

    let mut measurements: HashMap<String, StationMeasurement> = HashMap::new();

    for line in mmap.split(|&b| b == b'\n') {
        if line.is_empty() {
            continue;
        }

        let line = unsafe { std::str::from_utf8_unchecked(line) };
        let (station, measurement) = line.split_once(";").unwrap();

        measurements
            .entry(station.to_string())
            .or_default()
            .add_measurement(measurement.parse().unwrap());
    }

    let mut stations = measurements.keys().collect::<Vec<_>>();
    stations.sort();
    let results = stations
        .into_iter()
        .map(|station| {
            format!(
                "{}={}",
                station,
                measurements.get(station).unwrap().to_string()
            )
        })
        .collect::<Vec<_>>();
    println!("{{{}}}", results.join(", "));
}
