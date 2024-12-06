use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let (width, height, mut i, mut j, obstacles) =
        parse_input(read_to_string("input.txt").unwrap());
    let mut di = -1;
    let mut dj = 0;

    let mut visited = HashSet::new();
    while 0 <= i && i < height && 0 <= j && j < width {
        visited.insert((i, j));
        if obstacles.contains(&(i + di, j + dj)) {
            (di, dj) = (dj, -di);
        } else {
            i += di;
            j += dj;
        }
    }

    println!("{}", visited.len());
}

fn parse_input(input: String) -> (i16, i16, i16, i16, HashSet<(i16, i16)>) {
    let width = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .count()
        .try_into()
        .unwrap();
    let height = input.lines().count().try_into().unwrap();

    let mut obstacles = HashSet::new();
    let mut start_i = None;
    let mut start_j = None;
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                obstacles.insert((i.try_into().unwrap(), j.try_into().unwrap()));
            } else if c == '^' {
                start_i = Some(i.try_into().unwrap());
                start_j = Some(j.try_into().unwrap());
            }
        }
    }

    (width, height, start_i.unwrap(), start_j.unwrap(), obstacles)
}
