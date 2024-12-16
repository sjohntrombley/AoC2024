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

impl Add for &Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<&Point> for Point {
    type Output = Self;

    fn add(self, other: &Point) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign<&Point> for Point {
    fn add_assign(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

const UP_MOVE: Point = Point { x: 0, y: -1 };
const RIGHT_MOVE: Point = Point { x: 1, y: 0 };
const DOWN_MOVE: Point = Point { x: 0, y: 1 };
const LEFT_MOVE: Point = Point { x: -1, y: 0 };

struct WarehouseState {
    robot_position: Point,
    walls: HashSet<Point>,
    boxes: HashSet<Point>,
}

fn main() {
    let (mut warehouse, moves) = parse_input(&read_to_string("input.txt").unwrap());

    for move_ in moves {
        if move_.x == 0 {
            move_robot_vertical(&mut warehouse, &move_);
        // We still need to accout for the fact that boxes are twice as wide in the horizontal case
        } else if move_.x == 1 {
            move_robot_right(&mut warehouse, &move_);
        } else {
            move_robot_left(&mut warehouse, &move_);
        }
    }

    println!(
        "{}",
        warehouse
            .boxes
            .into_iter()
            .map(|p| i32::from(p.y) * 100 + i32::from(p.x))
            .sum::<i32>()
    );
}

fn move_robot_vertical(warehouse: &mut WarehouseState, move_: &Point) {
    let next_position = warehouse.robot_position + move_;

    if warehouse.walls.contains(&next_position) {
        return;
    }

    let box_position = warehouse
        .boxes
        .get(&next_position)
        .or(warehouse.boxes.get(&(next_position + LEFT_MOVE)));

    if let Some(&box_position) = box_position {
        if can_move_boxes_vertical(warehouse, &box_position, move_) {
            move_boxes_vertical(warehouse, &box_position, move_);
            warehouse.robot_position = next_position;
        }
    } else {
        warehouse.robot_position = next_position;
    }
}

fn can_move_boxes_vertical(
    warehouse: &WarehouseState,
    box_position: &Point,
    move_: &Point,
) -> bool {
    let next_position = box_position + move_;
    let right_position = next_position + RIGHT_MOVE;
    if warehouse.walls.contains(&next_position) || warehouse.walls.contains(&right_position) {
        return false;
    }

    let left_position = next_position + LEFT_MOVE;
    (!warehouse.boxes.contains(&left_position)
        || can_move_boxes_vertical(warehouse, &left_position, move_))
        && (!warehouse.boxes.contains(&next_position)
            || can_move_boxes_vertical(warehouse, &next_position, move_))
        && (!warehouse.boxes.contains(&right_position)
            || can_move_boxes_vertical(warehouse, &right_position, move_))
}

fn move_boxes_vertical(warehouse: &mut WarehouseState, box_position: &Point, move_: &Point) {
    let next_position = *box_position + *move_;

    if warehouse.boxes.contains(&next_position) {
        move_boxes_vertical(warehouse, &next_position, move_);
    } else {
        let left_position = next_position + LEFT_MOVE;
        if warehouse.boxes.contains(&left_position) {
            move_boxes_vertical(warehouse, &left_position, move_);
        }

        let right_position = next_position + RIGHT_MOVE;
        if warehouse.boxes.contains(&right_position) {
            move_boxes_vertical(warehouse, &right_position, move_);
        }
    }

    warehouse.boxes.remove(box_position);
    warehouse.boxes.insert(next_position);
}

fn move_robot_right(warehouse: &mut WarehouseState, move_: &Point) {
    let mut next_position = warehouse.robot_position + *move_;
    let double_move = *move_ + *move_;
    let mut boxes = Vec::new();

    while warehouse.boxes.contains(&next_position) {
        boxes.push(next_position);
        next_position += double_move;
    }

    if !warehouse.walls.contains(&next_position) {
        for box_position in boxes {
            warehouse.boxes.remove(&box_position);
            warehouse.boxes.insert(box_position + *move_);
        }
        warehouse.robot_position += *move_;
    }
}

fn move_robot_left(warehouse: &mut WarehouseState, move_: &Point) {
    let double_move = move_ + move_;
    let mut next_position = warehouse.robot_position;
    let mut boxes = Vec::new();

    while warehouse.boxes.contains(&(next_position + double_move)) {
        next_position += double_move;
        boxes.push(next_position);
    }

    if !warehouse.walls.contains(&(next_position + move_)) {
        for box_position in boxes {
            warehouse.boxes.remove(&box_position);
            warehouse.boxes.insert(box_position + move_);
        }
        warehouse.robot_position += move_;
    }
}

fn parse_input(input: &str) -> (WarehouseState, Vec<Point>) {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let (robot_position, walls, boxes) = parse_map(map);
    (
        WarehouseState {
            robot_position,
            walls,
            boxes,
        },
        parse_moves(moves),
    )
}

fn parse_map(map: &str) -> (Point, HashSet<Point>, HashSet<Point>) {
    let mut robot_start = None;
    let mut walls = HashSet::new();
    let mut boxes = HashSet::new();

    for (y, row) in map.lines().enumerate() {
        let y = y.try_into().unwrap();
        for (x, c) in row.chars().enumerate() {
            let x: i8 = x.try_into().unwrap();
            match c {
                '#' => {
                    walls.insert(Point { x: 2 * x, y });
                    walls.insert(Point { x: 2 * x + 1, y });
                }
                'O' => {
                    boxes.insert(Point { x: 2 * x, y });
                }
                '@' => robot_start = Some(Point { x: 2 * x, y }),
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
            '^' => UP_MOVE,
            '>' => RIGHT_MOVE,
            'v' => DOWN_MOVE,
            '<' => LEFT_MOVE,
            _ => panic!("Character other that ^, >, v, or < found in moves"),
        })
    }

    v
}

// Visualizations for debugging
//const WIDTH: Num = 100;
//const HEIGHT: Num = 50;
//
//fn print_move(move_: &Point) {
//    println!(
//        "Move {}:",
//        match *move_ {
//            UP_MOVE => '^',
//            RIGHT_MOVE => '>',
//            DOWN_MOVE => 'v',
//            LEFT_MOVE => '<',
//            _ => panic!("Invalid move"),
//        }
//    );
//}
//
//fn print_map(warehouse: &WarehouseState) {
//    for y in 0..HEIGHT {
//        for x in 0..WIDTH {
//            let p = Point { x, y };
//            if warehouse.walls.contains(&p) {
//                print!("#");
//            } else if warehouse.boxes.contains(&p) {
//                print!("[");
//            } else if warehouse.boxes.contains(&(p + LEFT_MOVE)) {
//                print!("]");
//            } else if p == warehouse.robot_position {
//                print!("@");
//            } else {
//                print!(".");
//            }
//        }
//        println!("");
//    }
//    println!("");
//}
