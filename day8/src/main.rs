use anyhow::Result;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

type Node = String;
type Route = String;
type Map = HashMap<Node,(Node,Node)>;

const START: &'static str = "AAA";
const TERMINAL: &'static str = "ZZZ";

fn parse_map(filename: &str) -> Result<(Route,Map)> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut route = Route::new();
    let mut map = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        println!("{line}");
	if line.is_empty() {
	    continue;
	}
	if route.is_empty() {
	    route = line;
	    continue;
	}
	let (node, left_right) = line.split_once('=').unwrap();
	let node = node.trim().to_owned();
	let (left, right) = left_right.split_once(",").unwrap();
	let left = left[2..].to_owned();
 	let right = right[1..right.len()-1].to_owned();
	println!("{node}: ({left}, {right})");
	map.insert(node, (left, right));
    }
    Ok((route, map))
}

fn part1(route: &str, map: &Map) -> usize {
    let mut node = START;
    for (i, r) in route.chars().cycle().enumerate() {
	let (left, right) = &map[node];
	// println!("cur: {cur}, route: {r}, left: {left}, right: {right}");
	node = if r == 'L' { left } else { right };
	if node == TERMINAL {
	    return i + 1;
	}
    }
    0
}

fn part2(route: &str, map: &Map) -> usize {
    let mut nodes = map.keys().filter(|node| node.ends_with('A')).collect::<Vec<_>>();
    println!("{nodes:?}");
    for (i, r) in route.chars().cycle().enumerate() {
	nodes = nodes.iter().map(|&node| {
	    let (left, right) = &map[node];
	    if r == 'L' { left } else { right }	    
	}).collect();
	println!("{nodes:?}");
	if nodes.iter().filter(|&node| node.ends_with('Z')).count() == nodes.len() {
	    return i + 1;
	}
    }
    0
}

fn main() -> Result<()> {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "inputs/test1.txt".to_string());

    let (route, map) = parse_map(&filename)?;

    let start1 = Instant::now();
    let sum1 = 0; // part1(&route, &map);
    let duration1 = start1.elapsed();
    println!("part1: {sum1}, time: {duration1:?}");

    let start2 = Instant::now();
    let sum2 = part2(&route, &map);
    let duration2 = start2.elapsed();
    println!("part2: {sum2}, time: {duration2:?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample1() {
        let filename = "src/inputs/test1.txt";
        let (route, map) = parse_map(&filename).unwrap();
        assert_eq!(288, part1(&route, &map));
        assert_eq!(71503, part2(&route, &map));
    }
}
