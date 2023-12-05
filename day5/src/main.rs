use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Debug)]
struct Map {
    dest: u32,
    src: u32,
    len: u32,
}

type Seeds = Vec<u32>;
type Maps = Vec<Map>;

fn parse_almanac(filename: &str) -> Result<(Seeds, Vec<Maps>)> {
    let mut seeds: Seeds = Vec::new();
    let mut maps: Vec<Maps> = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut map = Vec::new();
    let mut mapping = String::new();
    for line in reader.lines() {
        let line = line?;
	println!("{line}");
	if line.is_empty() {
	    maps.push(map);
	    map = Vec::new();
	    continue;
	}
	match line.split_once(':') {
	    Some(("seeds", seed_str)) => {
		seeds = seed_str.split_whitespace().
		map( |s| s.parse::<u32>().unwrap()).collect();
	    },
	    Some((mapping, _)) => {
		println!("{mapping}");
	    },
	    None => {		// we're in a mapping
		let mut map_iter = line.split_whitespace().
		    map( |s| s.parse::<u32>().unwrap());
		let dest = map_iter.next().unwrap();
		let src = map_iter.next().unwrap();
		let len = map_iter.next().unwrap();
		map.push(Map{ dest, src, len });
	    },
	}
    };
    Ok((seeds, maps))
}

fn part1(seeds: &Seeds, maps: &Vec<Maps>) -> u32 {
    let mut sum = 0;
    println!("{seeds:?} {maps:?}");
    sum
}

fn part2(seeds: &Seeds, maps: &Vec<Maps>) -> u32 {
    let mut sum = 0;
    println!("{seeds:?} {maps:?}");
    sum
}

fn main() -> Result<()> {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "inputs/test1.txt".to_string());

    let (seeds, maps) = parse_almanac(&filename)?;

    let start1 = Instant::now();
    let sum1 = part1(&seeds, &maps);
    let duration1 = start1.elapsed();
    println!("part1: {sum1}, time: {duration1:?}");

    let start2 = Instant::now();
    let sum2 = part2(&seeds, &maps);
    let duration2 = start2.elapsed();
    println!("part2: {sum2}, time: {duration2:?}");
    Ok(())
}
