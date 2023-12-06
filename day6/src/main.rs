use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn parse_race_records(filename: &str) -> Result<(Vec<u32>, Vec<u32>)> {
    let mut times = Vec::new();
    let mut records = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        // println!("{line}");
        match line.split_once(':') {
            Some(("Time", time_str)) => {
                times = time_str[1..]
                    .split_whitespace()
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect();
                // println!("{times:?}");
            }
            Some(("Distance", records_str)) => {
                records = records_str[1..]
                    .split_whitespace()
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect();
                // println!("{records:?}");
            }
            _ => {
                panic!("Invalid Race Format");
            }
        }
    }
    Ok((times, records))
}

fn parse_race_records2(filename: &str) -> Result<(usize, usize)> {
    let mut time = 0;
    let mut record = 0;

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        // println!("{line}");
        match line.split_once(':') {
            Some(("Time", time_str)) => {
                time = time_str
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
                // println!("{time:?}");
            }
            Some(("Distance", records_str)) => {
                record = records_str
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
                // println!("{record:?}");
            }
            _ => {
                panic!("Invalid Race Format")
            }
        }
    }
    Ok((time, record))
}

fn part1(times: &[u32], records: &[u32]) -> u32 {
    times
        .iter()
        .zip(records.iter())
        .map(|(&time, &record)| {
            (1..time).rev().find(|t| (time - t) * t > record).unwrap() -
            (1..time).find(|t| (time - t) * t > record).unwrap()
        })
        .product()
}

fn part2(time: usize, record: usize) -> usize {
    (1..time).rev().find(|t| (time - t) * t > record).unwrap() -
        (1..time).find(|t| (time - t) * t > record).unwrap()
}

fn main() -> Result<()> {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "inputs/test1.txt".to_string());

    let start1 = Instant::now();
    let (times, records) = parse_race_records(&filename)?;
    let sum1 = part1(&times, &records);
    let duration1 = start1.elapsed();
    println!("part1: {sum1}, time: {duration1:?}");

    let start2 = Instant::now();
    let (times, records) = parse_race_records2(&filename)?;
    let sum2 = part2(times, records);
    let duration2 = start2.elapsed();
    println!("part2: {sum2}, time: {duration2:?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let filename = "src/inputs/test1.txt";
        let (times, records) = parse_race_records(&filename).unwrap();
        assert_eq!(288, part1(&times, &records));
        let (time, record) = parse_race_records2(&filename).unwrap();
        assert_eq!(71503, part2(time, record));
    }

    #[test]
    fn test_solution() {
        let filename = "src/inputs/input.txt";
        let (times, records) = parse_race_records(&filename).unwrap();
        assert_eq!(588588, part1(&times, &records));
        let (time, record) = parse_race_records2(&filename).unwrap();
        assert_eq!(34655848, part2(time, record));
    }
}
