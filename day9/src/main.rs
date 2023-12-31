use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn parse_sensor_readings(filename: &str) -> Result<Vec<Vec<i32>>> {
    let mut sensor_readings = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        sensor_readings.push(
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect(),
        );
    }
    Ok(sensor_readings)
}

fn part1(sensor_readings: &[Vec<i32>]) -> i32 {
    let mut sum = 0;
    for reading_history in sensor_readings {
        let mut diffs = reading_history
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect::<Vec<_>>();
        let mut diff_sum = 0;
        while diffs.iter().filter(|&&v| v == 0).count() != diffs.len() {
            // println!("{diffs:?}");
            diff_sum += diffs.last().unwrap();
            diffs = diffs.windows(2).map(|w| w[1] - w[0]).collect();
        }
        let next = reading_history.last().unwrap() + diff_sum;
        // println!("diff_sum: {diff_sum}, next_item: {next}");
        sum += next;
    }
    sum
}

fn part2(sensor_readings: &[Vec<i32>]) -> i32 {
    let mut sum = 0;
    for reading_history in sensor_readings {
        let mut diffs = reading_history
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect::<Vec<_>>();
        
        // If a, b, c, d, e are the first values in the differences, the final diff
        // to subtract from the first element in the sequence works out to
        // (a - (b - (c - (d - e))). This can be simplifed to a - b + c - d + e,
        // so essentially flipping the sign and adding first values instead of
        // storing the first values and computing this later.
        let mut diff_sum = 0;
        let mut sign_flip = 1;
        while diffs.iter().filter(|&&v| v == 0).count() != diffs.len() {
            // println!("{diffs:?}");
            diff_sum += sign_flip * diffs.first().unwrap();
            sign_flip = -sign_flip;
            diffs = diffs.windows(2).map(|w| w[1] - w[0]).collect();
        }
        let prev = reading_history.first().unwrap() - diff_sum;
        // println!("diff_sum: {diff_sum:?}, prev: {prev}");
        sum += prev;
    }
    sum
}

fn main() -> Result<()> {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "inputs/test1.txt".to_string());

    let start = Instant::now();
    let sensor_readings = parse_sensor_readings(&filename)?;
    let parse_duration = start.elapsed();
    println!("sensor readings parse time: {parse_duration:?}");

    let start = Instant::now();
    let sum1 = part1(&sensor_readings);
    let duration1 = start.elapsed();
    println!("part1: {sum1}, time: {duration1:?}");

    let start = Instant::now();
    let sum2 = part2(&sensor_readings);
    let duration2 = start.elapsed();
    println!("part2: {sum2}, time: {duration2:?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let filename = "src/inputs/test1.txt";
        let sensor_readings = parse_sensor_readings(&filename).unwrap();
        assert_eq!(114, part1(&sensor_readings));
        assert_eq!(2, part2(&sensor_readings));
    }

    #[test]
    fn test_solution() {
        let filename = "src/inputs/input.txt";
        let sensor_readings = parse_sensor_readings(&filename).unwrap();
        assert_eq!(1955513104, part1(&sensor_readings));
        assert_eq!(1131, part2(&sensor_readings));
    }
}
