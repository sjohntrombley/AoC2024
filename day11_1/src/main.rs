use std::collections::LinkedList;
use std::fs::read_to_string;

fn main() {
    let mut unprocessed: LinkedList<u64> = read_to_string("input.txt")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut processed = LinkedList::new();

    for _ in 0..25 {
        while !unprocessed.is_empty() {
            let stone = unprocessed.pop_front().unwrap();
            if stone == 0 {
                processed.push_back(1);
                continue
            }

            let digits = stone.ilog10() + 1;
            if digits % 2 == 0 {
                processed.push_back(stone / 10_u64.pow(digits / 2));
                processed.push_back(stone % 10_u64.pow(digits / 2));
            } else {
                processed.push_back(stone * 2024);
            }
        }
        //{
        //    use std::io::{Write, stdin, stdout};
        //    let mut s = String::new();
        //    println!("{:?}", processed);
        //    print!("Press enter to continue:");
        //    stdout().flush().unwrap();
        //    stdin().read_line(&mut s).unwrap();
        //}
        (processed, unprocessed) = (unprocessed, processed);
    }

    println!("{}", unprocessed.len());
}
