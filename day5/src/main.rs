use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Debug)]
struct Map {
    dest: usize,
    src: usize,
    len: usize,
}

type Seeds = Vec<usize>;
type Maps = Vec<Map>;

fn parse_almanac(filename: &str) -> Result<(Seeds, Vec<Maps>)> {
    let mut seeds: Seeds = Vec::new();
    let mut maps: Vec<Maps> = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut map = Vec::new();
    let mut in_mapping = false;
    for line in reader.lines() {
        let line = line?;
        // println!("{line}");
        if line.is_empty() {
            if in_mapping {
                maps.push(map);
                map = Vec::new();
                in_mapping = false;
            }
            continue;
        }
        match line.split_once(':') {
            Some(("seeds", seed_str)) => {
                seeds = seed_str
                    .split_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();
            }
            Some((_mapping, _)) => {
                // println!("{_mapping}");
                in_mapping = true;
            }
            None => {
                // we're in a mapping
                let mut map_iter = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
                let dest = map_iter.next().unwrap();
                let src = map_iter.next().unwrap();
                let len = map_iter.next().unwrap();
                map.push(Map { dest, src, len });
            }
        }
    }
    if !map.is_empty() {
        maps.push(map);
    }
    Ok((seeds, maps))
}

fn map_seed_to_location(seed: usize, maps: &Vec<Maps>) -> usize {
    let mut mapping = seed;
    for map in maps {
        // print!("{mapping} -> ");
        for m in map {
            if mapping >= m.src && mapping < m.src + m.len {
                mapping = m.dest + mapping - m.src;
                break;
            }
        }
        // println!("{mapping}");
    }
    println!("seed: {seed}, location: {mapping}");
    mapping
}

fn part1(seeds: &Seeds, maps: &Vec<Maps>) -> usize {
    // println!("{seeds:?} {maps:?}");
    seeds
        .iter()
        .map(|&s| map_seed_to_location(s, maps))
        .min()
        .unwrap()
}

fn seed_range_to_min_location(seed: usize, len: usize, maps: &Vec<Maps>) -> usize {
    let mut min_mapping = usize::MAX;
    let mut s = seed;
    while s < seed + len {
        let mut mapping = s;
        let mut min_span = usize::MAX;
        for map in maps {
            // print!("{mapping} -> ");
            for m in map {
                if mapping >= m.src && mapping < m.src + m.len {
                    let remaining_span = m.len - (mapping - m.src);
                    println!("seed {s} remaining_span {remaining_span}");
                    if m.len < min_span {
                        min_span = remaining_span;
                    }
                    mapping = m.dest + mapping - m.src;
                    break;
                }
            }
            println!("seed {s}, mapping {mapping} min_span {min_span}");
        }
        s += min_span;
        if mapping < min_mapping {
            min_mapping = mapping;
        }
    }
    println!("seed: {seed}, location: {min_mapping}");
    min_mapping
}

fn part2(seeds: &Seeds, maps: &Vec<Maps>) -> usize {
    let mut min = usize::MAX;
    for seed_and_len in seeds.chunks_exact(2) {
        let (seed, len) = (seed_and_len[0], seed_and_len[1]);
        let min_of_range = seed_range_to_min_location(seed, len, maps);
        if min_of_range < min {
            min = min_of_range;
        }
    }
    min
}

fn main() -> Result<()> {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "inputs/test1.txt".to_string());

    let (seeds, maps) = parse_almanac(&filename)?;

    let start1 = Instant::now();
    let sum1 = 0; // part1(&seeds, &maps);
    let duration1 = start1.elapsed();
    println!("part1: {sum1}, time: {duration1:?}");

    let start2 = Instant::now();
    let sum2 = part2(&seeds, &maps);
    let duration2 = start2.elapsed();
    println!("part2: {sum2}, time: {duration2:?}");
    Ok(())
}
