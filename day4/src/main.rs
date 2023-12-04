use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn get_num_matches(numbers: &str) -> usize {
    // Format "41 48 83 86 17 | 83 86  6 31 17  9 48 53"
    // The first part is the winning numbers and the second part "your" numbers
    let (winning, yours) = numbers.split_once('|').unwrap();
    let winning = winning
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<HashSet<_>>();
    let num_matches = yours
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .filter(|n| winning.contains(n))
        .count();
    num_matches
}

fn part1(filename: &str) -> Result<usize> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let line = line?;
        // Format: "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
        let (_hdr, numbers) = line.split_once(':').unwrap();
        let num_matches = get_num_matches(&numbers);
        let points = if num_matches > 0 {
            usize::pow(2, num_matches as u32 - 1)
        } else {
            0
        };
        sum += points;
    }
    Ok(sum)
}

fn part2(filename: &str) -> Result<usize> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut cards_map = HashMap::<usize, usize>::new(); // card id -> num cards
    for line in reader.lines() {
        let line = line?;
        // Format: "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
        let (hdr, numbers) = line.split_once(':').unwrap();
        let card = hdr
            .split_whitespace()
            .nth(1) // skip 'Card' prefix
            .unwrap()
            .parse::<usize>()?;
        *cards_map.entry(card).or_default() += 1; // original card
        let num_matches = get_num_matches(&numbers);
        let num_copies: usize = cards_map[&card];
        for c in (card + 1)..=(card + num_matches) {
            *cards_map.entry(c).or_default() += num_copies;
        }
        // println!("card {card}, num_matches {num_matches} {cards_map:?}");
    }
    Ok(cards_map.values().sum())
}

fn main() -> Result<()> {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "inputs/test1.txt".to_string());
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

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test() {
        let example = "src/inputs/test1.txt";
        assert_eq!(13, part1(example).unwrap());
        assert_eq!(30, part2(example).unwrap());
    }
}
