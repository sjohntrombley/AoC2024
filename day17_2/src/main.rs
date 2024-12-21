use std::fs::read_to_string;

type Num = u64;

fn main() {
    let program = parse_input(&read_to_string("input.txt").unwrap());
    println!("{}", get_a(&program, 0).unwrap());
}

fn get_a(program: &[Num], mut a_prev: Num) -> Option<Num> {
    if program.is_empty() {
        return Some(a_prev);
    }

    let mut program = Vec::from(program);
    let b_out = program.pop().unwrap();
    a_prev <<= 3;
    let mut options = Vec::new();
    for mut b in 0..8 {
        let b_prev = b ^ (a_prev >> b);
        let b_prev = b_prev - (b_prev & 7) + b_out;
        let c = b ^ 5 ^ b_prev;
        let a_min = c << b;
        let a_max = (c + 1) << b;
        b ^= 3;
        let a_next = a_prev + b;
        if a_min < a_prev + 8 && a_prev < a_max && a_min <= a_next && a_next < a_max {
            options.push(a_next);
        }
    }

    options.sort();
    for a_next in options {
        let a_option = get_a(&program, a_next);
        if a_option.is_some() {
            return a_option;
        }
    }

    None
}

fn parse_input(input: &str) -> Vec<Num> {
    let mut input = input.lines().skip(4);
    input
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}
