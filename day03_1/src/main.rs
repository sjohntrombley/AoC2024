use std::fs::read_to_string;

fn main() {
    let mut input = &read_to_string("input.txt").unwrap()[..];
    let mut ans = 0;
    loop {
        input = match input.split_once("mul(") {
            Some((_, rest)) => rest,
            None => break,
        };
        // See if the char in the closure can be removed later
        let (x, rest) = input.split_at(input.find(|c: char| !c.is_ascii_digit()).unwrap_or(input.len()));
        let x: u32 = if x.len() == 0 {
            continue;
        } else if x.len() < 4 {
            input = rest;
            x.parse().unwrap()
        } else {
            input = rest;
            continue;
        };
        input = match input.strip_prefix(',') {
            Some(s) => s,
            None => continue,
        };
        let (y, rest) = input.split_at(input.find(|c: char| !c.is_ascii_digit()).unwrap_or(input.len()));
        let y: u32 = if y.len() == 0 {
            continue;
        } else if y.len() < 4 {
            input = rest;
            y.parse().unwrap()
        } else {
            input = rest;
            continue;
        };
        input = match input.strip_prefix(')') {
            Some(s) => s,
            None => continue,
        };
        ans += x * y;
    }
    println!("{}", ans);
}
