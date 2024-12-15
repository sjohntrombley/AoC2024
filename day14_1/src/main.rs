use std::cmp::Ordering::{Greater, Less};
use std::fmt::Debug;
use std::fs::read_to_string;
use std::ops::AddAssign;
use std::str::FromStr;

const WIDTH: i16 = 101;
const HEIGHT: i16 = 103;

struct Point<T> {
    x: T,
    y: T,
}

struct Robot<T> {
    position: Point<T>,
    velocity: Point<T>,
}

fn main() {
    let mut robots: Vec<Robot<i16>> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(parse_robot)
        .collect();

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.position.x += robot.velocity.x;
            robot.position.x %= WIDTH;
            robot.position.y += robot.velocity.y;
            robot.position.y %= HEIGHT;
        }
    }

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for robot in robots {
        match (
            robot.position.x.cmp(&(WIDTH / 2)),
            robot.position.y.cmp(&(HEIGHT / 2)),
        ) {
            (Less, Less) => q1 += 1,
            (Less, Greater) => q2 += 1,
            (Greater, Less) => q3 += 1,
            (Greater, Greater) => q4 += 1,
            _ => (),
        }
    }

    println!("{}", q1 * q2 * q3 * q4);
}

fn parse_robot<T>(s: &str) -> Robot<T>
where
    T: FromStr<Err: Debug> + From<i16> + PartialOrd + AddAssign,
{
    let (position, velocity) = s.split_once(' ').unwrap();
    let position = parse_point(position.strip_prefix("p=").unwrap());
    let mut velocity = parse_point(velocity.strip_prefix("v=").unwrap());

    // Because of the teleporting, every negative component of the velocity has an equivalent
    // non-negative component, so we keep adding WIDTH (or HEIGHT for the y component) until both
    // components are non-negative. This is useful because it keeps the position non-negative when
    // we do a %= WIDTH (or HEIGHT).
    while velocity.x < T::from(0) {
        velocity.x += T::from(WIDTH);
    }
    while velocity.y < T::from(0) {
        velocity.y += T::from(HEIGHT);
    }

    Robot { position, velocity }
}

fn parse_point<T>(s: &str) -> Point<T>
where
    T: FromStr<Err: Debug>,
{
    let (x, y) = s.split_once(',').unwrap();
    Point {
        x: x.parse().unwrap(),
        y: y.parse().unwrap(),
    }
}
