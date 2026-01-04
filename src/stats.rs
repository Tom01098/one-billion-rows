use crate::measurements::StationMeasurements;
use rapidhash::{HashMapExt, RapidHashMap};

pub fn compute_statistics(input: &[u8]) -> Vec<String> {
    let mut measurements: RapidHashMap<&str, StationMeasurements> =
        RapidHashMap::with_capacity(10_000);

    for line in input.split(|&b| b == b'\n') {
        if line.is_empty() {
            continue;
        }

        let line = unsafe { std::str::from_utf8_unchecked(line) };
        let (station, measurement) = line.split_once(";").unwrap();

        measurements
            .entry(station)
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

    results
}
