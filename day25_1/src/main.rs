use std::fs::read_to_string;

#[derive(Debug)]
enum Trie {
    Node(Box<[Option<Trie>; 6]>),
    Leaf,
}

fn parse_lock(schematic: &str) -> [u8; 5] {
    let mut pin_heights = [0; 5]; // Actually 5 minus the height of the pin to make matching keys
                                  // easier.
    for (i, row) in schematic.lines().skip(1).take(5).enumerate() {
        for (j, pin_height) in pin_heights.iter_mut().enumerate() {
            if *pin_height == 0 && row.chars().nth(j).unwrap() == '.' {
                *pin_height = 5 - u8::try_from(i).unwrap();
            }
        }
    }

    pin_heights
}

fn parse_key(schematic: &str) -> [u8; 5] {
    let mut key_heights = [0; 5];

    for (i, row) in schematic.lines().skip(1).take(5).enumerate() {
        for (j, key_height) in key_heights.iter_mut().enumerate() {
            if *key_height == 0 && row.chars().nth(j).unwrap() == '#' {
                *key_height = 5 - u8::try_from(i).unwrap();
            }
        }
    }

    key_heights
}

fn build_trie(locks: &[[u8; 5]], depth: usize) -> Trie {
    let mut children = [const { None }; 6];

    if depth == 4 {
        for lock in locks {
            children[usize::from(lock[4])] = Some(Trie::Leaf);
        }
    } else {
        let mut i = 0;
        let mut j = 1;
        while i < locks.len() {
            let cur_height = locks[i][depth];
            // probably should be a binary search
            while j < locks.len() && locks[j][depth] == cur_height {
                j += 1;
            }

            children[usize::from(cur_height)] = Some(build_trie(&locks[i..j], depth + 1));
            i = j;
            j += 1;
        }
    }

    Trie::Node(Box::new(children))
}

fn parse_input(input: &str) -> (Vec<[u8; 5]>, Trie) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for schematic in input.split("\n\n") {
        if schematic.starts_with('#') {
            locks.push(parse_lock(schematic));
        } else {
            keys.push(parse_key(schematic));
        }
    }

    locks.sort();

    (keys, build_trie(&locks, 0))
}

fn count_compatible_locks(key: &[u8], locks: &Trie) -> u16 {
    let mut count = 0;
    let cur_height = key[0].into();
    let Trie::Node(children) = locks else {
        panic!("count_compatible_locks called with locks == Tire::Leaf.")
    };
    for (i, child) in children.iter().enumerate().rev() {
        if i < cur_height {
            break;
        }

        if let Some(child) = child {
            count += match child {
                Trie::Node(..) => count_compatible_locks(&key[1..], child),
                Trie::Leaf => 1,
            };
        }
    }

    count
}

fn solve(keys: &[[u8; 5]], locks: &Trie) -> u16 {
    keys.iter()
        .map(|key| count_compatible_locks(key, locks))
        .sum()
}

fn main() {
    let (keys, locks) = parse_input(&read_to_string("input.txt").unwrap());
    println!("{}", solve(&keys, &locks));
}
