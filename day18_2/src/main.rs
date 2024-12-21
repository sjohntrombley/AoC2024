use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;

type BytePosition = (i8, i8);

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut bytes: HashSet<_> = input
        .lines()
        .map(parse_byte_position)
        .take(1024)
        .collect();

    for coord in input.lines().skip(1024).map(parse_byte_position) {
        bytes.insert(coord);
        if !can_escape(&bytes) {
            println!("{},{}", coord.0, coord.1);
            break
        }
    }
}

fn can_escape(bytes: &HashSet<BytePosition>) -> bool {
    let mut visited = HashSet::new();
    visited.insert((70, 70));

    let mut to_visit = VecDeque::new();
    to_visit.push_back((70, 70));

    while !to_visit.is_empty() {
        let (x, y) = to_visit.pop_front().unwrap();
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let x = x + dx;
            let y = y + dy;
            let coord = (x, y);

            if x == 0 && y == 0 {
                return true
            }

            if (0..=70).contains(&x)
                && (0..=70).contains(&y)
                && !bytes.contains(&coord)
                && !visited.contains(&coord)
            {
                visited.insert(coord);
                to_visit.push_back(coord);
            }
        }
    }

    false
}

fn parse_byte_position(s: &str) -> BytePosition {
    let (x, y) = s.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}
