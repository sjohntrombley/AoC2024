use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn parse_input(input: &str) -> HashMap<String, HashSet<String>> {
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input.lines() {
        let (c0, c1) = line.split_once('-').unwrap();
        connections
            .entry(c0.to_string())
            .or_default()
            .insert(c1.to_string());
        connections
            .entry(c1.to_string())
            .or_default()
            .insert(c0.to_string());
    }

    connections
}

fn main() {
    let connections = parse_input(&read_to_string("input.txt").unwrap());
    let mut visited = HashSet::new();
    let mut ans = 0;

    for (computer, neighbors) in connections
        .iter()
        .filter(|(computer, _)| computer.starts_with('t'))
    {
        let mut neighbor0_iter = neighbors
            .iter()
            .filter(|&neighbor| !visited.contains(neighbor));
        while let Some(neighbor0) = neighbor0_iter.next() {
            for neighbor1 in neighbor0_iter.clone() {
                if connections[neighbor0].contains(neighbor1) {
                    ans += 1;
                }
            }
        }

        visited.insert(computer);
    }

    println!("{ans}");
}
