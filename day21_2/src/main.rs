// The time it takes to find the answer changes a lot depending on the order that the button pairs
// are calucated in. This is probably worth exploring.
use std::collections::HashMap;
use std::fs::read_to_string;

type Coord = (char, char);

mod numpad {
    use crate::{Coord, HashMap};

    const BUTTONS: &str = "7894561230A";

    pub fn get_transition_matrix(
        dpad_transition_matrix: &HashMap<Coord, String>,
    ) -> HashMap<Coord, String> {
        let mut transition_matrix = HashMap::new();
        let mut candidate_cache = HashMap::new();

        for s in BUTTONS.chars() {
            for d in BUTTONS.chars() {
                let sequence_candidates = get_paths(s, d);
                if sequence_candidates.len() == 1 {
                    transition_matrix
                        .insert((s, d), sequence_candidates.into_iter().next().unwrap());
                } else {
                    candidate_cache.insert((s, d), sequence_candidates);
                }
            }
        }

        for (&coord, sequence_candidates) in &candidate_cache {
            let mut sequences = sequence_candidates.clone();
            loop {
                for sequence in sequences.iter_mut() {
                    *sequence = sequence.chars().zip(sequence.chars().skip(1)).fold(
                        dpad_transition_matrix[&('A', sequence.chars().next().unwrap())].clone(),
                        |sequence, coord| sequence + &dpad_transition_matrix[&coord],
                    );
                }

                let sequence_lens: Vec<_> =
                    sequences.iter().map(|sequence| sequence.len()).collect();
                let min_len = sequence_lens.iter().min().unwrap();
                if sequence_lens
                    .iter()
                    .filter(|sequence_len| *sequence_len == min_len)
                    .count()
                    == 1
                {
                    transition_matrix.insert(
                        coord,
                        sequence_lens
                            .iter()
                            .enumerate()
                            .find_map(|(i, l)| {
                                if l == min_len {
                                    Some(sequence_candidates[i].clone())
                                } else {
                                    None
                                }
                            })
                            .unwrap(),
                    );
                    break;
                }
            }
        }

        transition_matrix
    }

    fn get_paths(start: char, end: char) -> Vec<String> {
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
    use crate::{Coord, HashMap};

    const BUTTONS: &str = "^A<v>";

    pub fn get_transition_matrix() -> HashMap<Coord, String> {
        let mut transition_matrix = HashMap::new();
        let mut candidate_cache = HashMap::new();

        for s in BUTTONS.chars() {
            for d in BUTTONS.chars() {
                let sequence_candidates = get_paths(s, d);
                if sequence_candidates.len() == 1 {
                    transition_matrix
                        .insert((s, d), sequence_candidates.into_iter().next().unwrap());
                } else {
                    candidate_cache.insert((s, d), sequence_candidates);
                }
            }
        }

        // For some reason, if, on the first iteration, coord == ('v', 'A'), it takes a lot longer.
        // It probably just takes way longer for one of the sequences to be shorter than the others
        // in that case, but exactly why isn't something I've been able to figure out.
        for (&coord, sequence_candidates) in &candidate_cache {
            let mut candidate_sequences: Vec<_> = sequence_candidates
                .iter()
                .map(|s| vec![s.clone()])
                .collect();
            loop {
                for sequences in candidate_sequences.iter_mut() {
                    *sequences = sequences
                        .iter_mut()
                        .flat_map(|sequence| {
                            sequence.chars().zip(sequence.chars().skip(1)).fold(
                                transition_matrix
                                    .get(&('A', sequence.chars().next().unwrap()))
                                    .map(|s| vec![s.clone()])
                                    .unwrap_or_else(|| {
                                        candidate_cache[&('A', sequence.chars().next().unwrap())]
                                            .clone()
                                    }),
                                |v, coord| {
                                    transition_matrix
                                        .get(&coord)
                                        .map(|s1| v.iter().map(|s0| s0.clone() + s1).collect())
                                        .unwrap_or_else(|| {
                                            v.iter()
                                                .flat_map(|s0| {
                                                    candidate_cache[&coord]
                                                        .iter()
                                                        .map(move |s1| s0.clone() + s1)
                                                })
                                                .collect()
                                        })
                                },
                            )
                        })
                        .collect();
                }

                let sequence_lens: Vec<_> = candidate_sequences
                    .iter()
                    .map(|sequences| {
                        sequences
                            .iter()
                            .map(|sequence| sequence.chars().count())
                            .min()
                            .unwrap()
                    })
                    .collect();
                let min_len = sequence_lens.iter().min().unwrap();
                if sequence_lens.iter().filter(|&len| len == min_len).count() == 1 {
                    transition_matrix.insert(
                        coord,
                        sequence_lens
                            .iter()
                            .enumerate()
                            .find_map(|(i, l)| {
                                if l == min_len {
                                    Some(sequence_candidates[i].clone())
                                } else {
                                    None
                                }
                            })
                            .unwrap(),
                    );
                    break;
                }
            }
        }

        transition_matrix
    }

    fn get_paths(start: char, end: char) -> Vec<String> {
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
    let dpad_tm = dpad::get_transition_matrix();
    let numpad_tm = numpad::get_transition_matrix(&dpad_tm);

    let dpad_tm = (2..=25).fold(
        HashMap::<_, _, std::collections::hash_map::RandomState>::from_iter(
            dpad_tm
                .iter()
                .map(|(&coord, sequence)| ((1, coord), sequence.chars().count())),
        ),
        |mut dptm, i| {
            dpad_tm.iter().for_each(|(&coord, sequence)| {
                dptm.insert(
                    (i, coord),
                    dptm[&(i - 1, ('A', sequence.chars().next().unwrap()))]
                        + sequence
                            .chars()
                            .zip(sequence.chars().skip(1))
                            .map(|coord| dptm[&(i - 1, coord)])
                            .sum::<usize>(),
                );
            });
            dptm
        },
    );

    let input = read_to_string("input.txt").unwrap();
    let codes: Vec<_> = input.lines().collect();
    let mut complexity = 0;
    for code in codes {
        let sequence = code.chars().zip(code.chars().skip(1)).fold(
            numpad_tm[&('A', code.chars().next().unwrap())].clone(),
            |sequence, coord| sequence + &numpad_tm[&coord],
        );

        complexity += (dpad_tm[&(25, ('A', sequence.chars().next().unwrap()))]
            + sequence
                .chars()
                .zip(sequence.chars().skip(1))
                .map(|coord| dpad_tm[&(25, coord)])
                .sum::<usize>())
            * code.strip_suffix('A').unwrap().parse::<usize>().unwrap();
    }

    println!("{complexity}");
}
