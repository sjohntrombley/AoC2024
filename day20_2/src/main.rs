use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

type Num = i16;
type Coord = (Num, Num);

fn main() {
    let (start, end, walls) = parse_input(&read_to_string("input.txt").unwrap());

    let start_dists = get_dists(start, &walls);
    let end_dists = get_dists(end, &walls);
    let best_dist = start_dists[&end];

    let mut cheat_count = 0;
    for ((x0, y0), start_dist) in &start_dists {
        for dy in Num::try_from(-20).unwrap()..=20 {
            let x_range = 20 - dy.abs();
            for dx in -x_range..=x_range {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let x1 = x0 + dx;
                let y1 = y0 + dy;
                if end_dists.get(&(x1, y1)).is_some_and(|end_dist| {
                    start_dist + end_dist + dx.abs() + dy.abs() <= best_dist - 100
                }) {
                    cheat_count += 1;
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
