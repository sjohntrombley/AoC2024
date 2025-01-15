use std::fs::read_to_string;

type Num = u64;

fn step_secret(n: &mut Num) {
    *n ^= *n * 64;
    *n %= 16777216;
    *n ^= *n / 32;
    *n %= 16777216;
    *n ^= *n * 2048;
    *n %= 16777216;
}

fn main() {
    println!(
        "{}",
        read_to_string("input.txt")
            .unwrap()
            .lines()
            .map(|s| {
                let mut n = s.parse().unwrap();
                for _ in 0..2000 {
                    step_secret(&mut n);
                }
                n
            })
            .sum::<Num>()
    );
}
