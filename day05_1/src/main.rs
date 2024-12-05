use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let (rules, updates) = parse_input(&read_to_string("input.txt").unwrap());

    let mut ordered_middle_sum: u16 = 0;
    for update in updates {
        let mut ordered = true;
        'outer: for (i, n) in update.iter().enumerate().skip(1) {
            for m in update.iter().take(i) {
                if rules.get(n).is_some_and(|s| s.contains(m)) {
                    ordered = false;
                    break 'outer;
                }
            }
        }

        if ordered {
            ordered_middle_sum += u16::from(update[update.len() / 2]);
        }
    }

    println!("{ordered_middle_sum}");
}

fn parse_input(input: &str) -> (HashMap<u8, HashSet<u8>>, Vec<Vec<u8>>) {
    let (rules_string, updates_string) = input.split_once("\n\n").unwrap();

    let mut rules: HashMap<_, HashSet<_>> = HashMap::new();
    for line in rules_string.lines() {
        let (before, after) = line.split_once('|').unwrap();
        rules
            .entry(before.parse().unwrap())
            .or_default()
            .insert(after.parse().unwrap());
    }

    let updates = updates_string
        .lines()
        .map(|l| l.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}
