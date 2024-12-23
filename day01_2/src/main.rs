use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let (left, right) = parse_input("input.txt");
    println!(
        "{}",
        left.iter()
            .map(|l| l * right.get(l).unwrap_or(&0))
            .sum::<u32>()
    );
}

fn parse_input(path: &str) -> (Vec<u32>, HashMap<u32, u32>) {
    let mut left = Vec::new();
    let mut right = HashMap::new();

    for line in read_to_string(path).unwrap().lines() {
        let (left_string, right_string) = line.split_once("   ").unwrap();
        left.push(left_string.parse().unwrap());
        let r = right_string.parse().unwrap();
        right.entry(r).and_modify(|count| *count += 1).or_insert(1);
    }

    (left, right)
}
