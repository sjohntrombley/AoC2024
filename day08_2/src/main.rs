use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let (height, width, antenna_locations) = parse_input(read_to_string("input.txt").unwrap());
    let mut antinodes = HashSet::new();

    for locations in antenna_locations.into_values() {
        for (i, (r0, c0)) in locations.iter().enumerate() {
            for (r1, c1) in locations.iter().skip(i + 1) {
                let mut dr = r1 - r0;
                let mut dc = c1 - c0;
                let d = gcd(dr.abs(), dc.abs());
                dr /= d;
                dc /= d;

                for _ in 0..2 {
                    let mut r = *r0;
                    let mut c = *c0;

                    while 0 <= r && r < height && 0 <= c && c < width {
                        antinodes.insert((r, c));
                        r += dr;
                        c += dc;
                    }

                    dr *= -1;
                    dc *= -1;
                }
            }
        }
    }

    println!("{}", antinodes.len());
}

fn gcd(mut x: i8, mut y: i8) -> i8 {
    while y > 0 {
        (x, y) = (y, x % y);
    }
    x
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
