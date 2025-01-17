use std::collections::HashSet;
use std::fs::read_to_string;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Num = u64;

struct SequenceIter {
    price_changes: Arc<Vec<Box<[i8]>>>,
    monkey_index: usize,
    window_start: usize,
    checked: HashSet<[i8; 4]>,
}

impl SequenceIter {
    fn new(price_changes: Arc<Vec<Box<[i8]>>>) -> Self {
        SequenceIter {
            price_changes,
            monkey_index: 0,
            window_start: 0,
            checked: HashSet::new(),
        }
    }
}

impl Iterator for SequenceIter {
    type Item = [i8; 4];

    fn next(&mut self) -> Option<Self::Item> {
        let mut sequence = [0; 4];
        while self.monkey_index < self.price_changes.len() {
            if self.window_start + 3 < self.price_changes[self.monkey_index].len() {
                self.price_changes[self.monkey_index][self.window_start..self.window_start + 4]
                    .iter()
                    .enumerate()
                    .for_each(|(i, &n)| sequence[i] = n);
                self.window_start += 1;
                if !self.checked.contains(&sequence) {
                    self.checked.insert(sequence.clone());
                    break;
                }
            } else {
                self.monkey_index += 1;
                self.window_start = 0;
            }
        }

        (self.monkey_index < self.price_changes.len()).then_some(sequence)
    }
}

fn step_secret(n: &mut Num) {
    *n ^= *n * 64;
    *n %= 16777216;
    *n ^= *n / 32;
    *n %= 16777216;
    *n ^= *n * 2048;
    *n %= 16777216;
}

fn get_prices(mut n: Num) -> [i8; 2001] {
    let mut prices = [0; 2001];
    prices[0] = (n % 10).try_into().unwrap();
    for i in 1..2001 {
        step_secret(&mut n);
        prices[i] = (n % 10).try_into().unwrap();
    }

    prices
}

fn solve(input: String) -> u32 {
    let prices: Arc<Vec<_>> = Arc::new(
        input
            .lines()
            .map(|s| get_prices(s.parse().unwrap()))
            .collect(),
    );
    let price_changes: Arc<Vec<Box<[_]>>> = Arc::new(
        prices
            .iter()
            .map(|v| v.windows(2).map(|s| s[1] - s[0]).collect())
            .collect(),
    );
    let mut most_bananas: u32 = 0;

    let sequence_iter = Arc::new(Mutex::new(SequenceIter::new(Arc::clone(&price_changes))));

    let (tx, rx) = mpsc::channel();
    for _ in 0..16 {
        let prices = Arc::clone(&prices);
        let price_changes = Arc::clone(&price_changes);
        let sequence_iter = Arc::clone(&sequence_iter);
        let tx = tx.clone();
        thread::spawn(move || loop {
            let cur_sequence = match sequence_iter.lock().unwrap().next() {
                Some(sequence) => sequence,
                None => break,
            };

            let bananas = price_changes
                .iter()
                .zip(prices.iter())
                .map(|(monkey_price_changes, monkey_prices)| {
                    monkey_price_changes
                        .windows(4)
                        .enumerate()
                        .filter_map(|(i, sequence)| {
                            if sequence == cur_sequence.as_ref() {
                                Some(monkey_prices[i + 4].try_into().unwrap())
                            } else {
                                None
                            }
                        })
                        .next()
                        .unwrap_or(0)
                })
                .sum();

            tx.send(bananas).unwrap();
        });
    }
    // drop tx so the channel closes when all of the threads end
    drop(tx);

    for bananas in rx {
        if bananas > most_bananas {
            most_bananas = bananas;
        }
    }

    most_bananas
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("{}", solve(input));
}
