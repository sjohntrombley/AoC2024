use std::fs::read_to_string;

fn main() {
    let mut input = &read_to_string("input.txt").unwrap()[..];
    let mut enabled = true;
    let mut ans = 0;
    loop {
        let i_do = input.find("do()");
        let i_dont = input.find("don't()");
        let i_mul = match input.find("mul(") {
            Some(i_mul) => i_mul,
            None => {
                break;
            }
        };
        match (i_do, i_dont) {
            (Some(i_do), Some(i_dont)) => {
                if i_do < i_dont && i_do < i_mul {
                    enabled = true;
                    (_, input) = input.split_at(i_do + 4);
                    continue;
                } else if i_dont < i_do && i_dont < i_mul {
                    enabled = false;
                    (_, input) = input.split_at(i_dont + 7);
                    continue;
                } else {
                    (_, input) = input.split_at(i_mul + 4);
                }
            }
            (None, Some(i_dont)) => {
                if i_dont < i_mul {
                    enabled = false;
                    (_, input) = input.split_at(i_dont + 7);
                    continue;
                } else {
                    (_, input) = input.split_at(i_mul + 4);
                }
            }
            (Some(i_do), None) => {
                if i_do < i_mul {
                    enabled = true;
                    (_, input) = input.split_at(i_do + 4);
                    continue;
                } else {
                    (_, input) = input.split_at(i_mul + 4);
                }
            }
            (None, None) => {
                (_, input) = input.split_at(i_mul + 4);
            }
        };
        if !enabled {
            continue;
        }
        // See if the char in the closure can be removed later
        let (x, rest) = input.split_at(
            input
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(input.len()),
        );
        let x: u32 = if x.is_empty() {
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
        let (y, rest) = input.split_at(
            input
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(input.len()),
        );
        let y: u32 = if y.is_empty() {
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
