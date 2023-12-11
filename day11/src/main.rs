use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn parse_and_solve(filename: &str, expansion: usize) -> Result<usize> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut galaxies = Vec::new();
    let mut col_multiplier = Vec::new();
    let mut row_multiplier = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let row = line?;
        let mut empty_row = true;
        if col_multiplier.is_empty() {
            col_multiplier = vec![expansion; row.len()];
        }
        for (j, v) in row.bytes().enumerate() {
            if v == b'#' {
                galaxies.push((i, j));
                empty_row = false;
                col_multiplier[j] = 1;
            }
        }
        let row_expansion = if empty_row { expansion } else { 1 };
        row_multiplier.push(row_expansion);
    }
    let mut sum = 0;
    for (i, p1) in galaxies.iter().enumerate() {
        for p2 in &galaxies[i + 1..] {
            let xrange = if p1.0 < p2.0 { p1.0..p2.0 } else { p2.0..p1.0 };
            for x in xrange {
                sum += row_multiplier[x];
            }
            let yrange = if p1.1 < p2.1 { p1.1..p2.1 } else { p2.1..p1.1 };
            for y in yrange {
                sum += col_multiplier[y];
            }
        }
    }
    Ok(sum)
}

fn part1(filename: &str) -> Result<usize> {
    parse_and_solve(filename, 2)
}

fn part2(filename: &str) -> Result<usize> {
    parse_and_solve(filename, 1_000_000)
}

fn main() -> Result<()> {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "inputs/test1.txt".to_string());

    let start = Instant::now();
    let sum1 = part1(&filename).unwrap();
    let duration1 = start.elapsed();
    println!("part1: {sum1}, time: {duration1:?}");

    let start = Instant::now();
    let sum2 = part2(&filename).unwrap();
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

        assert_eq!(374, part1(&filename).unwrap());
        assert_eq!(82000210, part2(&filename).unwrap());
    }

    #[test]
    fn test_solution() {
        let filename = "src/inputs/input.txt";

        assert_eq!(9543156, part1(filename).unwrap());
        assert_eq!(625243292686, part2(filename).unwrap());
    }
}
