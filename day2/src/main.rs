use anyhow::{Error, Result};
use std::cmp::max;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

type CubeSet = [u32; 3];

fn parse_cubeset(cubeset_str: &str) -> Result<CubeSet> {
    let mut cubeset = [0, 0, 0];
    const COLORS: [&str; 3] = ["red", "green", "blue"];
    for num_and_cube in cubeset_str.split(',') {
        let (num, color) = num_and_cube[1..]
            .split_once(' ')
            .ok_or_else(|| Error::msg("invalid cube configuration"))?;
        let pos = COLORS
            .iter()
            .position(|c| c == &color)
            .ok_or_else(|| Error::msg("invalid color"))?;
        cubeset[pos] = num.parse::<u32>()?;
    }
    Ok(cubeset)
}

fn part1(filename: &str) -> Result<u32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut sum = 0;
    const CUBE_BAG: [u32; 3] = [12, 13, 14]; // Number of red, green, blue in bag
    'game: for line in reader.lines() {
        let line = line?;
        // Format: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        let (hdr, game_plays) = line
            .split_once(':')
            .ok_or_else(|| Error::msg("invalid game format"))?;
        for game_play in game_plays.split(';') {
            let cubeset = parse_cubeset(game_play)?;
            for (bag, cube) in CUBE_BAG.iter().zip(cubeset.iter()) {
                if bag < cube {
                    // this game is impossible
                    continue 'game;
                }
            }
        }
        let game_id = hdr[5..].parse::<u32>()?;
        sum += game_id;
    }
    Ok(sum)
}

fn part2(filename: &str) -> Result<u32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let line = line?;
        // Format: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        let (_hdr, game_plays) = line
            .split_once(':')
            .ok_or_else(|| Error::msg("invalid game format"))?;
        let mut min_cubeset = [0, 0, 0];
        for game_play in game_plays.split(';') {
            let cubeset = parse_cubeset(game_play)?;
            for (min_color, color) in min_cubeset.iter_mut().zip(cubeset.iter()) {
                *min_color = max(*min_color, *color);
            }
        }
        let power: u32 = min_cubeset.iter().product();
        sum += power;
    }
    Ok(sum)
}

fn main() -> Result<()> {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "inputs/input.txt".to_string());
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
        assert_eq!(8, part1(example).unwrap());
        assert_eq!(2286, part2(example).unwrap());
    }
}
