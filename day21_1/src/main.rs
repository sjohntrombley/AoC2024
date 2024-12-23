use std::fs::read_to_string;

mod numpad {
    pub fn get_shortest(code: &str) -> Vec<String> {
        let mut location = 'A';
        let mut sequences = vec!["".to_string()];
        let mut next_sequences = Vec::new();

        for next_location in code.chars() {
            let paths = get_path(location, next_location);
            for sequence in &sequences {
                for path in &paths {
                    next_sequences.push(sequence.clone() + path);
                }
            }
            (sequences, next_sequences) = (next_sequences, sequences);
            next_sequences.clear();
            location = next_location;
        }

        sequences
    }

    fn get_path(start: char, end: char) -> Vec<String> {
        let (sx, sy) = match start {
            '0' => (1, 3),
            'A' => (2, 3),
            '1'..='9' => {
                let c = u8::try_from(start).unwrap() - b'1';
                (c % 3, 2 - c / 3)
            }
            _ => panic!("Invalid button"),
        };
        let (ex, ey) = match end {
            '0' => (1, 3),
            'A' => (2, 3),
            '1'..='9' => {
                let c = u8::try_from(end).unwrap() - b'1';
                (c % 3, 2 - c / 3)
            }
            _ => panic!("Invalid button"),
        };

        let lr_string = if sx < ex {
            ">".repeat((ex - sx).into())
        } else {
            "<".repeat((sx - ex).into())
        };
        let ud_string = if sy < ey {
            "v".repeat((ey - sy).into())
        } else {
            "^".repeat((sy - ey).into())
        };

        if sx == ex || sy == ey || sx == 0 && ey == 3 {
            vec![lr_string + &ud_string + "A"]
        } else if sy == 3 && ex == 0 {
            vec![ud_string + &lr_string + "A"]
        } else {
            vec![
                lr_string.clone() + &ud_string + "A",
                ud_string + &lr_string + "A",
            ]
        }
    }
}

mod dpad {
    pub fn get_shortest(code: &str) -> Vec<String> {
        let mut location = 'A';
        let mut sequences = vec!["".to_string()];
        let mut next_sequences = Vec::new();

        for next_location in code.chars() {
            let paths = get_path(location, next_location);
            for sequence in &sequences {
                for path in &paths {
                    next_sequences.push(sequence.clone() + path);
                }
            }
            (sequences, next_sequences) = (next_sequences, sequences);
            next_sequences.clear();
            location = next_location;
        }

        sequences
    }

    fn get_path(start: char, end: char) -> Vec<String> {
        let (sx, sy): (u8, u8) = match start {
            '^' => (1, 0),
            'A' => (2, 0),
            '<' => (0, 1),
            'v' => (1, 1),
            '>' => (2, 1),
            _ => panic!("Invalid button"),
        };
        let (ex, ey) = match end {
            '^' => (1, 0),
            'A' => (2, 0),
            '<' => (0, 1),
            'v' => (1, 1),
            '>' => (2, 1),
            _ => panic!("Invalid button"),
        };

        let lr_string = if sx < ex {
            ">".repeat((ex - sx).into())
        } else {
            "<".repeat((sx - ex).into())
        };
        let ud_string = if sy < ey {
            "v".repeat((ey - sy).into())
        } else {
            "^".repeat((sy - ey).into())
        };

        if sx == ex || sy == ey || sx == 0 && ey == 0 {
            vec![lr_string + &ud_string + "A"]
        } else if sy == 0 && ex == 0 {
            vec![ud_string + &lr_string + "A"]
        } else {
            vec![
                lr_string.clone() + &ud_string + "A",
                ud_string + &lr_string + "A",
            ]
        }
    }
}

fn main() {
    let codes: Vec<_> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();
    let mut complexity_sum = 0;
    for code in codes {
        let sequences = numpad::get_shortest(&code);
        let min_len = sequences
            .iter()
            .map(|sequence| sequence.len())
            .min()
            .unwrap();
        let sequences = sequences
            .iter()
            .filter(|&sequence| sequence.len() == min_len)
            .flat_map(|sequence| dpad::get_shortest(sequence));
        let min_len = sequences
            .clone()
            .map(|sequence| sequence.len())
            .min()
            .unwrap();
        let sequences = sequences
            .filter(|sequence| sequence.len() == min_len)
            .flat_map(|sequence| dpad::get_shortest(&sequence));
        let min_len = sequences
            .clone()
            .map(|sequence| sequence.len())
            .min()
            .unwrap();
        complexity_sum += min_len * code.strip_suffix('A').unwrap().parse::<usize>().unwrap();
    }
    println!("{complexity_sum}");
}
