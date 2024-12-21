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
            .filter(|design| is_possible(&towels, towel_max_len, design))
            .count()
    );
}

#[cached(
    ty = "SizedCache<String, bool>",
    create = "{ SizedCache::with_size(10000) }",
    convert = "{ design.to_string() }"
)]
fn is_possible(towels: &HashSet<String>, towel_max_len: usize, design: &str) -> bool {
    if design.is_empty() {
        return true;
    }

    for l in (1..=towel_max_len.min(design.len())).rev() {
        if towels.contains(&design[..l]) && is_possible(towels, towel_max_len, &design[l..]) {
            return true;
        }
    }

    false
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
