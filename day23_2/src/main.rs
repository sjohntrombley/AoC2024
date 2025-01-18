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

// Bron-Kerbosch algorithm
fn gp_helper(
    n: &HashMap<String, HashSet<String>>,
    r: HashSet<String>,
    mut p: HashSet<String>,
    mut x: HashSet<String>,
) -> Option<HashSet<String>> {
    if p.is_empty() && x.is_empty() {
        return Some(r.clone());
    }

    let mut maximum_subgraph: Option<HashSet<String>> = None;
    while !p.is_empty() {
        // If HashSet had a pop method we wouldn't need the clone and could just move it out of p
        // here instead of removing it later.
        let v = p.iter().next().unwrap().clone();
        let mut r = r.clone();
        r.insert(v.clone());
        let ms_candidate = gp_helper(n, r, &p & &n[&v], &x & &n[&v]);
        if maximum_subgraph.is_none()
            || ms_candidate.as_ref().is_some_and(|ms_candidate| {
                ms_candidate.len() > maximum_subgraph.as_ref().unwrap().len()
            })
        {
            maximum_subgraph = ms_candidate;
        }
        p.remove(&v);
        x.insert(v);
    }

    maximum_subgraph
}

fn get_password(neighbor_map: HashMap<String, HashSet<String>>) -> String {
    let r = HashSet::new();
    let p = HashSet::from_iter(neighbor_map.keys().cloned());
    let x = HashSet::new();

    let mut pw_vec = Vec::from_iter(gp_helper(&neighbor_map, r, p, x).unwrap());
    pw_vec.sort();
    pw_vec.join(",")
}

fn main() {
    let neighbor_map = parse_input(&read_to_string("input.txt").unwrap());
    println!("{}", get_password(neighbor_map));
}
