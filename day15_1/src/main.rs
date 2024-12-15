use std::collections::HashSet;
use std::fs::read_to_string;
use std::ops::{Add, AddAssign};

type Num = i8;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: Num,
    y: Num,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

fn main() {
    let (mut robot_position, walls, mut boxes, moves) =
        parse_input(&read_to_string("input.txt").unwrap());

    for move_ in moves {
        let next_position = robot_position + move_;
        if boxes.contains(&next_position) {
            let mut test_position = next_position + move_;
            while boxes.contains(&test_position) {
                test_position += move_;
            }

            if !walls.contains(&test_position) {
                boxes.remove(&next_position);
                boxes.insert(test_position);
                robot_position = next_position;
            }
        } else if !walls.contains(&next_position) {
            robot_position = next_position;
        }
    }

    println!(
        "{}",
        boxes
            .into_iter()
            .map(|p| i32::from(p.y) * 100 + i32::from(p.x))
            .sum::<i32>()
    );
}

fn parse_input(input: &str) -> (Point, HashSet<Point>, HashSet<Point>, Vec<Point>) {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let (robot_start, walls, boxes) = parse_map(map);
    (robot_start, walls, boxes, parse_moves(moves))
}

fn parse_map(map: &str) -> (Point, HashSet<Point>, HashSet<Point>) {
    let mut robot_start = None;
    let mut walls = HashSet::new();
    let mut boxes = HashSet::new();

    for (y, row) in map.lines().enumerate() {
        let y = y.try_into().unwrap();
        for (x, c) in row.chars().enumerate() {
            let x = x.try_into().unwrap();
            match c {
                '#' => {
                    walls.insert(Point { x, y });
                }
                'O' => {
                    boxes.insert(Point { x, y });
                }
                '@' => robot_start = Some(Point { x, y }),
                _ => (),
            }
        }
    }

    (robot_start.unwrap(), walls, boxes)
}

fn parse_moves(moves: &str) -> Vec<Point> {
    let mut v = Vec::new();
    for c in moves.lines().flat_map(str::chars) {
        v.push(match c {
            '^' => Point { x: 0, y: -1 },
            '>' => Point { x: 1, y: 0 },
            'v' => Point { x: 0, y: 1 },
            '<' => Point { x: -1, y: 0 },
            _ => panic!("Character other that ^, >, v, or < found in moves"),
        })
    }

    v
}
