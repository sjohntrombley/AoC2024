use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

type Num = u16;
type Coord = (i16, i16);

fn main() {
    let (start, end, walls) = parse_input(&read_to_string("input.txt").unwrap());

    let start_dists = get_dists(start, &walls);
    let end_dists = get_dists(end, &walls);
    let best_dist = start_dists[&end];

    let mut cheat_count = 0;
    for (x, y) in &walls {
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let coord0 = (x + dx, y + dy);
            if start_dists.contains_key(&coord0) {
                for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    let coord1 = (x + dx, y + dy);
                    if end_dists.contains_key(&coord1)
                        && start_dists[&coord0] + end_dists[&coord1] + 102 <= best_dist
                    {
                        cheat_count += 1;
                    }
                }
            }
        }
    }

    println!("{cheat_count}");
}

fn get_dists(start: Coord, walls: &HashSet<Coord>) -> HashMap<Coord, Num> {
    let mut dists = HashMap::new();
    dists.insert(start, 0);

    let mut to_visit = VecDeque::new();
    to_visit.push_back(start);

    while !to_visit.is_empty() {
        let coord = to_visit.pop_front().unwrap();
        let (x, y) = coord;
        let dist = dists[&coord];
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let coord = (x + dx, y + dy);
            if !walls.contains(&coord) && !dists.contains_key(&coord) {
                dists.insert(coord, dist + 1);
                to_visit.push_back(coord);
            }
        }
    }

    dists
}

fn parse_input(input: &str) -> (Coord, Coord, HashSet<Coord>) {
    let mut walls = HashSet::new();
    let mut start = None;
    let mut end = None;

    for (y, l) in input.lines().enumerate() {
        let y = y.try_into().unwrap();
        for (x, c) in l.chars().enumerate() {
            let x = x.try_into().unwrap();
            if c == '#' {
                walls.insert((x, y));
            } else if c == 'S' {
                start = Some((x, y));
            } else if c == 'E' {
                end = Some((x, y));
            }
        }
    }

    (start.unwrap(), end.unwrap(), walls)
}
