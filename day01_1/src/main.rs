use std::fs::read_to_string;

fn main() {
    let (mut left, mut right) = parse_input("input.txt");
    left.sort();
    right.sort();
    println!("{}", left.iter().zip(&right).map(|(&l, &r)| l.abs_diff(r)).sum::<u32>());
}

fn parse_input(path: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in read_to_string(path).unwrap().lines() {
        let (left_string, right_string) = line.split_once("   ").unwrap();
        left.push(left_string.parse().unwrap());
        right.push(right_string.parse().unwrap());
    }

    (left, right)
}
