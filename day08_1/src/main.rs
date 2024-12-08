use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let (height, width, antenna_locations) = parse_input(read_to_string("input.txt").unwrap());
    let mut antinodes = HashSet::new();

    for locations in antenna_locations.into_values() {
        for (i, (mut r0, mut c0)) in locations.iter().enumerate() {
            for (mut r1, mut c1) in locations.iter().skip(i + 1) {
                for _ in 0..2 {
                    let ar = 2 * r0 - r1;
                    let ac = 2 * c0 - c1;
                    if 0 <= ar && ar < height && 0 <= ac && ac < width {
                        antinodes.insert((ar, ac));
                    }

                    (r0, r1, c0, c1) = (r1, r0, c1, c0);
                }
            }
        }
    }

    println!("{}", antinodes.len());
}

fn parse_input(input: String) -> (i8, i8, HashMap<char, Vec<(i8, i8)>>) {
    let height = input.lines().count().try_into().unwrap();
    let width = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .count()
        .try_into()
        .unwrap();
    let mut antenna_locations: HashMap<_, Vec<_>> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                antenna_locations
                    .entry(c)
                    .or_default()
                    .push((i.try_into().unwrap(), j.try_into().unwrap()));
            }
        }
    }

    (height, width, antenna_locations)
}
