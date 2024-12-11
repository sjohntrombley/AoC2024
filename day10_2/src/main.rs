use std::fs::read_to_string;

fn main() {
    let map: Vec<Vec<_>> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().map(|c| u8::try_from(c).unwrap() - b'0').collect())
        .collect();
    let mut path_counts = vec![vec![0; map[0].len()]; map.len()];

    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if map[r][c] == 9 {
                path_counts[r][c] = 1;
            }
        }
    }

    for height in (0..9).rev() {
        for r in 0..map.len() {
            for c in 0..map[r].len() {
                if map[r][c] == height {
                    if r > 0 && map[r - 1][c] == height + 1 {
                        path_counts[r][c] += path_counts[r - 1][c];
                    }
                    if r + 1 < map.len() && map[r + 1][c] == height + 1 {
                        path_counts[r][c] += path_counts[r + 1][c];
                    }
                    if c > 0 && map[r][c - 1] == height + 1 {
                        path_counts[r][c] += path_counts[r][c - 1];
                    }
                    if c + 1 < map[r].len() && map[r][c + 1] == height + 1 {
                        path_counts[r][c] += path_counts[r][c + 1];
                    }
                }
            }
        }
    }

    println!(
        "{}",
        (0..map.len())
            .map(|r| (0..map[r].len())
                .filter(|c| map[r][*c] == 0)
                .map(|c| path_counts[r][c])
                .sum::<u16>())
            .sum::<u16>()
    );
}
