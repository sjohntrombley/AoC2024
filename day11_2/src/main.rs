use memoize::memoize;
use std::fs::read_to_string;

fn main() {
    let input: Vec<_> = read_to_string("input.txt")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!(
        "{}",
        input
            .into_iter()
            .map(|stone| count_stones(stone, 75))
            .sum::<u64>()
    );
}

#[memoize(Capacity: 100000)]
fn count_stones(stone: u64, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }
    if stone == 0 {
        return count_stones(1, depth - 1);
    }
    let digits = stone.ilog10() + 1;
    if digits % 2 == 0 {
        return count_stones(stone / 10_u64.pow(digits / 2), depth - 1)
            + count_stones(stone % 10_u64.pow(digits / 2), depth - 1);
    }
    count_stones(stone * 2024, depth - 1)
}
