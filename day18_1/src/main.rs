use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

type BytePosition = (i8, i8);

fn main() {
    let bytes: HashSet<_> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(parse_byte_position)
        .take(1024)
        .collect();

    let mut dists = HashMap::new();
    dists.insert((70, 70), 0);

    let mut to_visit = VecDeque::new();
    to_visit.push_back((70, 70));

    loop {
        let (x, y) = to_visit.pop_front().unwrap();
        let dist = dists[&(x, y)];
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let x = x + dx;
            let y = y + dy;
            let coord = (x, y);

            if x == 0 && y == 0 {
                println!("{}", dist + 1);
                return;
            }

            if (0..=70).contains(&x)
                && (0..=70).contains(&y)
                && !bytes.contains(&coord)
                && !dists.contains_key(&coord)
            {
                dists.insert(coord, dist + 1);
                to_visit.push_back(coord);
            }
        }
    }
}

fn parse_byte_position(s: &str) -> BytePosition {
    let (x, y) = s.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}
