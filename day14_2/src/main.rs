use std::fmt::Debug;
use std::fs::read_to_string;
use std::io::{stdin, stdout, Write};
use std::ops::{AddAssign, RemAssign};
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
    // This is currently still somewhat manual. When manual_search is called, the position of the
    // robots after the first second is printed. Pressing enter will advance one second. If you
    // type anything before pressing enter, the function will exit. Take note of which seconds the
    // robots are clumped together vertically or horizontally. The vertical clumping should repeat
    // every 101 (i.e. WIDTH) seconds and the horizontal clumping should repeate every 103 (i.e.
    // HEIGHT) seconds.
    //
    // Once you know where the cycles start, the second where the cycles intersect can be easily
    // caclulated by hand. After this, comment out the call to manual_search and uncomment the call
    // to check_answer, replacing the 0 argument with the number you calculated. If you got it
    // right, it should print a christmas tree.
    let mut robots: Vec<Robot<i16>> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(parse_robot)
        .collect();

    manual_search(&mut robots);
    //check_answer(&mut robots, 0);
}

fn manual_search<T>(robots: &mut Vec<Robot<T>>)
where
    T: AddAssign + RemAssign + From<i16> + TryInto<usize, Error: Debug> + Copy,
{
    let mut response = String::new();
    let mut time = 1;
    while response.len() <= 2 {
        for robot in robots.iter_mut() {
            robot.position.x += robot.velocity.x;
            robot.position.x %= T::from(WIDTH);
            robot.position.y += robot.velocity.y;
            robot.position.y %= T::from(HEIGHT);
        }

        println!("\x1B[2J\x1B[1;1H{time}");
        print_robots(robots);
        let _ = stdout().flush();
        response.clear();
        let _ = stdin().read_line(&mut response);

        time += 1;
    }
}

fn check_answer<T>(robots: &mut Vec<Robot<T>>, answer: T)
where
    T: AddAssign + RemAssign + From<i16> + TryInto<usize, Error: Debug> + Copy,
{
    for _ in 0..answer.try_into().unwrap() {
        for robot in robots.iter_mut() {
            robot.position.x += robot.velocity.x;
            robot.position.x %= T::from(WIDTH);
            robot.position.y += robot.velocity.y;
            robot.position.y %= T::from(HEIGHT);
        }
    }

    print_robots(robots);
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

fn print_robots<T>(robots: &Vec<Robot<T>>)
where
    T: TryInto<usize, Error: Debug> + Copy,
{
    let mut robot_counts = [[0; WIDTH as usize]; HEIGHT as usize];

    for robot in robots {
        robot_counts[robot.position.y.try_into().unwrap()][robot.position.x.try_into().unwrap()] +=
            1;
    }

    println!(
        "{}",
        robot_counts
            .into_iter()
            .map(|r| r
                .into_iter()
                .map(|n| if n == 0 {
                    ".".to_string()
                } else if n < 10 {
                    n.to_string()
                } else {
                    panic!("more than 9 robots on a square")
                })
                .collect::<String>()
                + "\n")
            .collect::<String>()
    );
}
