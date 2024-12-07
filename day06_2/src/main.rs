use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let (width, height, start_i, start_j, obstacles) =
        parse_input(read_to_string("input.txt").unwrap());
    let mut i = start_i;
    let mut j = start_j;
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

    let mut position_count = 0;
    visited.remove(&(start_i, start_j));
    for (i, j) in visited {
        if creates_loop(start_i, start_j, i, j, width, height, &obstacles) {
            position_count += 1;
        }
    }

    println!("{position_count}");
}

fn creates_loop(
    mut i: i16,
    mut j: i16,
    obstacle_i: i16,
    obstacle_j: i16,
    width: i16,
    height: i16,
    obstacles: &HashSet<(i16, i16)>,
) -> bool {
    let mut di = -1;
    let mut dj = 0;

    let mut visited = HashSet::new();
    while 0 <= i && i < height && 0 <= j && j < width {
        if visited.contains(&(i, j, di, dj)) {
            return true;
        } else {
            visited.insert((i, j, di, dj));
        }

        if obstacles.contains(&(i + di, j + dj)) || i + di == obstacle_i && j + dj == obstacle_j {
            (di, dj) = (dj, -di);
        } else {
            i += di;
            j += dj;
        }
    }

    false
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
