use cached::proc_macro::cached;
use cached::SizedCache;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let (towels, designs) = parse_input(&read_to_string("input.txt").unwrap());
    let towel_max_len = towels.iter().map(String::len).max().unwrap();
    println!(
        "{}",
        designs
            .iter()
            .map(|design| count_ways(&towels, towel_max_len, design))
            .sum::<u64>()
    );
}

#[cached(
    ty = "SizedCache<String, u64>",
    create = "{ SizedCache::with_size(10000) }",
    convert = "{ design.to_string() }"
)]
fn count_ways(towels: &HashSet<String>, towel_max_len: usize, design: &str) -> u64 {
    if design.is_empty() {
        return 1;
    }

    let mut ways = 0;
    for l in 1..=towel_max_len.min(design.len()) {
        if towels.contains(&design[..l]) {
            ways += count_ways(towels, towel_max_len, &design[l..]);
        }
    }

    ways
}

fn parse_input(input: &str) -> (HashSet<String>, Vec<String>) {
    let mut input = input.lines();
    let towels = input
        .next()
        .unwrap()
        .split(", ")
        .map(str::to_string)
        .collect();
    input.next();
    let designs = input.map(str::to_string).collect();

    (towels, designs)
}
