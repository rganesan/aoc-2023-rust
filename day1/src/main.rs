use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn part1(filename: &str) -> Result<u32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let line = line?;
        let first = line
            .chars()
            .find(|c| c.is_ascii_digit())
            .unwrap_or( '0')
            .to_digit(10).unwrap();
        let last = line
            .chars()
            .rfind(|c| c.is_ascii_digit())
            .unwrap_or( '0')
            .to_digit(10).unwrap();
        sum += first * 10 + last;
    }
    Ok(sum)
}

fn match_number(s: &str) -> Option<u32> {
    const NUM_STRINGS: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let c = s.chars().next()?;
    if c.is_ascii_digit() {
        return c.to_digit(10);
    };
    for (i, num) in NUM_STRINGS.iter().enumerate() {
        if s.starts_with(num) {
            return Some(i as u32);
        }
    }
    None
}

fn part2(filename: &str) -> Result<u32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let line = line?;
        for i in 0..line.len() {
            if let Some(first) = match_number(&line[i..]) {
                sum += first * 10;
                break;
            }
        }
        for i in (0..line.len()).rev() {
            if let Some(last) = match_number(&line[i..]) {
                sum += last;
                break;
            }
        }
    }
    Ok(sum)
}

fn main() -> Result<()> {
    let filename = env::args().nth(1).unwrap_or_else(|| "inputs/input.txt".to_string());
    let start1 = Instant::now();
    let sum1 = part1(&filename).expect("failed");
    let duration1 = start1.elapsed();
    println!("part1: {sum1}, time: {duration1:?}");
    let start2 = Instant::now();
    let sum2 = part2(&filename).expect("failed");
    let duration2 = start2.elapsed();
    println!("part2: {sum2}, time: {duration2:?}");
    Ok(())
}
