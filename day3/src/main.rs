use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use lazy_static::lazy_static;
use regex::Regex;

type Schematic = Vec<String>;

#[derive(Debug)]
struct PartNumber {
    row: usize,
    start: usize,
    end: usize,
    num: usize,
}

fn parse_part_numbers(schematic: &Schematic) -> Vec<PartNumber> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)").unwrap();
    }
    let mut part_numbers = Vec::with_capacity(schematic.len());
    for (row, line) in schematic.iter().enumerate() {
        for m in RE.find_iter(line) {
            let part_number = PartNumber {
                row: row, start: m.start(), end: m.end(),
                num: m.as_str().parse::<usize>().unwrap()
            };
            // println!("{part_number:?}");
            part_numbers.push(part_number);
        }
    }
    part_numbers
}

fn part1(schematic: &Schematic, part_numbers: &Vec<PartNumber>) -> usize {
    let mut sum = 0;
    'part_number: for pn in part_numbers {
        // check to the left and right in current row
        let cur_row = schematic[pn.row].as_bytes();
        if pn.start > 0 && cur_row[pn.start - 1] != b'.' { // left
            sum += pn.num;
            // println!("part number: {}", pn.num);
            continue;
        }
        if pn.end < schematic[pn.row].len() && cur_row[pn.end] != b'.' { // right
            sum += pn.num;
            // println!("part number: {}", pn.num);
            continue;
        }
        
        // For prev and next row, we need to check -/+ one to account for diagonal
        let start = if pn.start > 0 { pn.start - 1 } else { pn.start };
        let end = if pn.end < cur_row.len() - 1 { pn.end + 1 } else { pn.end };
        
        // check previous row
        if pn.row > 0 {
            let prev_row = schematic[pn.row - 1].as_bytes();
            for c in &prev_row[start..end] {
                if *c != b'.' && !c.is_ascii_digit() {
                    // found a symbol
                    sum += pn.num;
                    // println!("part number: {}", pn.num);
                    continue 'part_number;
                }
            }
        }
        // check next row
        if pn.row < schematic.len() - 1 {
            let next_row = schematic[pn.row + 1].as_bytes();
            for c in &next_row[start..end] {
                if *c != b'.' && !c.is_ascii_digit() {
                    // found a symbol
                    sum += pn.num;
                    // println!("part number: {}", pn.num);
                    continue 'part_number;
                }
            }
        }
    }
    sum
}

fn part2(schematic: &Schematic, part_numbers: &Vec<PartNumber>) -> usize {
    let mut sum = 0;
    for (r, line) in schematic.iter().enumerate() {
        for (c, ch) in line.bytes().enumerate() {
            if ch != b'*' {
                continue;
            }
            // println!("{r},{c}: *");
            let mut adjacents = Vec::with_capacity(2);
            for pn in part_numbers {
                if r > 1 && pn.row < r - 1 {
                    continue;   // this row cannot be adjacent
                }
                let start = if pn.start > 0 { pn.start - 1 } else { pn.start };
                let end = if pn.end > line.len() - 1 { pn.end + 1 } else { pn.end };
                if r > 0 && pn.row == r - 1 {
                    // previous row
                    if start <= c && end >= c {
                        adjacents.push(pn.num);
                    }
                } else if pn.row == r {
                    // current row
                    if pn.start == c + 1 || pn.end == c {
                        adjacents.push(pn.num);
                    }
                } else if pn.row == r + 1 {
                    // next row
                    if start <= c && end >= c {
                        adjacents.push(pn.num);
                    }
                }
                if pn.row > r + 1 {
                    break;
                }
            }
            // println!("{adjacents:?}");
            if adjacents.len() == 2 {
                sum += adjacents[0] * adjacents[1]
            }
        }
    }
    sum
}

fn parse_file(filename: &str) -> Result<Schematic> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut schematic = Vec::new();
    for line in reader.lines() {
        schematic.push(line?);
    }
    Ok(schematic)
}

fn main() -> Result<()> {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "inputs/test1.txt".to_string());

    let schematic = parse_file(&filename).expect("Couldn't parse file");
    let part_numbers = parse_part_numbers(&schematic);
    let start1 = Instant::now();
    let sum1 = part1(&schematic, &part_numbers);
    let duration1 = start1.elapsed();
    println!("part1: {sum1}, time: {duration1:?}");

    let start2 = Instant::now();
    let sum2 = part2(&schematic, &part_numbers);
    let duration2 = start2.elapsed();
    println!("part2: {sum2}, time: {duration2:?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test() {
        let example = "src/inputs/test1.txt";
        assert_eq!(8, part1(example).unwrap());
        assert_eq!(2286, part2(example).unwrap());
    }
}
