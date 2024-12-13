use std::fs::read_to_string;
// A_X*A + B_X*B = P_X; A_Y*A + B_Y*B = P_Y
// A = (P_X - B_X*B)/A_X; A = (P_Y - B_Y*B)/A_Y
// A_Y*(P_X - B_X*B) = A_X*(P_Y - B_Y*B)
// A_Y*P_X - A_X*P_Y = (A_Y*B_X - A_X*B_Y)*B
// B = (P_Y*A_X - P_X*A_Y)/(A_X*B_Y - A_Y*B_X)
// A = (P_X - B_X*(A_Y*P_X - A_X*P_Y)/(A_Y*B_X - A_X*B_Y))/A_X
// A = (P_Y*B_X - P_X*B_Y)/(A_Y*B_X - A_X*B_Y)
struct Point {
    x: i32,
    y: i32,
}

struct Machine {
    a: Point,
    b: Point,
    prize: Point,
}

fn main() {
    let input = parse_input(&read_to_string("input.txt").unwrap());
    let mut tokens = 0;
    for Machine { a, b, prize } in input {
        let num_a = prize.y * b.x - prize.x * b.y;
        let denom_a = a.y * b.x - a.x * b.y;
        if num_a % denom_a != 0 {
            continue;
        }

        let num_b = prize.y * a.x - prize.x * a.y;
        let denom_b = -denom_a;
        if num_b % denom_b != 0 {
            continue;
        }

        tokens += num_a / denom_a * 3 + num_b / denom_b;
    }

    println!("{tokens}");
}

fn parse_input(input: &str) -> Vec<Machine> {
    input.trim().split("\n\n").map(parse_machine).collect()
}

fn parse_machine(block: &str) -> Machine {
    let mut lines = block.lines();
    Machine {
        a: parse_button(lines.next().unwrap().strip_prefix("Button A: ").unwrap()),
        b: parse_button(lines.next().unwrap().strip_prefix("Button B: ").unwrap()),
        prize: parse_prize(lines.next().unwrap().strip_prefix("Prize: ").unwrap()),
    }
}

fn parse_button(s: &str) -> Point {
    let (x, y) = s.split_once(", ").unwrap();
    let x = x.strip_prefix("X+").unwrap().parse().unwrap();
    let y = y.strip_prefix("Y+").unwrap().parse().unwrap();
    Point { x, y }
}

fn parse_prize(s: &str) -> Point {
    let (x, y) = s.split_once(", ").unwrap();
    let x = x.strip_prefix("X=").unwrap().parse().unwrap();
    let y = y.strip_prefix("Y=").unwrap().parse().unwrap();
    Point { x, y }
}