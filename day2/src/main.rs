use anyhow::Result;
use std::cmp::max;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

#[derive(Debug)]
struct RGB {
    r: u32,
    g: u32,
    b: u32,
}

fn parse_rgb(play: &str) -> RGB {
    let mut rgb = RGB { r: 0, g: 0, b: 0 };
    for cube in play.split(',') {
        let mut cube_iter = cube[1..].split(' ');
        let num = cube_iter.next().unwrap().parse::<u32>().unwrap();
        let color = cube_iter.next().unwrap();
        match color {
            "red" => rgb.r = num,
            "green" => rgb.g = num,
            "blue" => rgb.b = num,
            _ => panic!("Invalid color {color}"),
        };
    }
    rgb
}

fn part1(filename: &str) -> Result<u32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let line = line?;
        // println!("{line}");
        // Format: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        let mut line_iter = line.split(':');
        let game_id = line_iter.next().unwrap()[5..].parse::<u32>()?;
        let mut game_possible = true;
        for game in line_iter.next().unwrap().split(';') {
            let rgb = parse_rgb(game);
            // println!("{rgb:?}");
            if rgb.r > RED || rgb.g > GREEN || rgb.b > BLUE {
                game_possible = false;
                break;
            }
        }
        if game_possible {
            // println!("{line} possible");
            sum += game_id;
        }
    }
    Ok(sum)
}

fn part2(filename: &str) -> Result<u32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let line = line?;
        // println!("{line}");
        // Format: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        let mut line_iter = line.split(':');
        let _game_id = line_iter.next().unwrap()[5..].parse::<u32>()?;
        let mut min_rgb = RGB { r: 0, g: 0, b: 0 };
        for game in line_iter.next().unwrap().split(';') {
            let rgb = parse_rgb(game);
            // println!("{rgb:?}");
            min_rgb.r = max(rgb.r, min_rgb.r);
            min_rgb.g = max(rgb.g, min_rgb.g);
            min_rgb.b = max(rgb.b, min_rgb.b);
        }
        let power = min_rgb.r * min_rgb.g * min_rgb.b;
        // println!("min_rgb: {min_rgb:?}");
        sum += power;
    }
    Ok(sum)
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
