use anyhow::Result;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

struct CardHand {
    hand: [u8; 5],
    bid: u16,
    score1: u32, // No joker
    score2: u32, // With joker
}

impl fmt::Debug for CardHand {
    // Custom Debug formatter so [u8; 5] is printed as a string
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {:4}, 0x{:06x}, 0x{:06x}",
            std::str::from_utf8(&self.hand).expect("invalid sequence"),
            self.bid,
            self.score1,
            self.score2
        )
    }
}

fn poker_score(hand: &[u8], joker: bool) -> u32 {
    const JOKER: usize = 0xb;
    let mut counts = [0u8; 16];
    let mut max = 0;
    let mut max_idx = 0;
    for c in hand {
        let idx = *c as usize;
        counts[idx] += 1;
        if joker && idx != JOKER && counts[idx] > max {
            max = counts[idx];
            max_idx = idx;
        }
    }
    let jokers = counts[JOKER];
    if joker && jokers != 5 {
        counts[JOKER] = 0;
        counts[max_idx] += jokers;
    }

    // strip the zeros so contains() is faster. This doesn't really
    // seem to make any difference in performance.
    let mut counts = counts.to_vec();
    counts.retain(|&v| v != 0);

    // Score hands from 7 to 1
    if counts.contains(&5) {
        7 // Five of a kind
    } else if counts.contains(&4) {
        6 // Four of a kind
    } else if counts.contains(&3) {
        if counts.contains(&2) {
            5 // Full house
        } else {
            4 // Three of a kind
        }
    } else {
        match counts.iter().filter(|&&v| v == 2).count() {
            2 => 3, // Two pair
            1 => 2, // One pair
            _ => 1, // (High card) all different
        }
    }
}

fn score(hand: &[u8], joker: bool) -> u32 {
    // Convert hand to hex for easier scoring
    let hand: [u8; 5] = hand
        .iter()
        .map(|b| match b {
            b'T' => 0xa,
            b'J' => 0xb,
            b'Q' => 0xc,
            b'K' => 0xd,
            b'A' => 0xe,
            _ => *b - b'0',
        })
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap();
    let mut score = poker_score(&hand, joker);
    // shift the bytes into a u32 so we can simply do a numeric comparison.
    score = hand.iter().fold(score, |score, &b| {
        (score << 4) | if joker && b == 0xb { 0 } else { b as u32 }
    });
    score
}

fn parse_and_score_card_hands(filename: &str) -> Result<Vec<CardHand>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let card_hands = reader
        .lines()
        .map(|lines| {
            let lines = lines.unwrap();
            let (hand, bid) = lines.split_once(' ').unwrap();
            let hand: [u8; 5] = hand
                .bytes()
                .collect::<Vec<u8>>()
                .try_into()
                .expect("invalid card hand");
            let bid = bid.parse().unwrap();
            let score1 = score(&hand, false);
            let score2 = score(&hand, true);
            CardHand {
                hand,
                bid,
                score1,
                score2,
            }
        })
        .collect();
    Ok(card_hands)
}

fn get_winnings(sorted_card_hands: &[CardHand]) -> usize {
    sorted_card_hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid as usize)
        .sum()
}

fn part1(card_hands: &mut [CardHand]) -> usize {
    card_hands.sort_by(|a, b| a.score1.cmp(&b.score1));
    // println!("{card_hands:#?}");
    get_winnings(card_hands)
}

fn part2(card_hands: &mut [CardHand]) -> usize {
    card_hands.sort_by(|a, b| a.score2.cmp(&b.score2));
    // println!("{card_hands:#?}");
    get_winnings(card_hands)
}

fn main() -> Result<()> {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "inputs/test1.txt".to_string());

    let start = Instant::now();
    let mut card_hands = parse_and_score_card_hands(&filename)?;
    let duration = start.elapsed();
    println!("parsing time: {duration:?}");

    let start1 = Instant::now();
    let sum1 = part1(&mut card_hands);
    let duration1 = start1.elapsed();
    println!("part1: {sum1}, time: {duration1:?}");

    let start2 = Instant::now();
    let sum2 = part2(&mut card_hands);
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

        let mut card_hands = parse_and_score_card_hands(&filename).unwrap();
        assert_eq!(6440, part1(&mut card_hands));
        assert_eq!(5905, part2(&mut card_hands));
    }

    #[test]
    fn test_solution() {
        let filename = "src/inputs/input.txt";

        let mut card_hands = parse_and_score_card_hands(&filename).unwrap();
        assert_eq!(253313241, part1(&mut card_hands));
        assert_eq!(253362743, part2(&mut card_hands));
    }
}
