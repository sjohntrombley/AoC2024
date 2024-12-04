use std::fs::read_to_string;

fn main() {
    let input: Vec<Vec<_>> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let height = input.len();
    let width = input.first().unwrap().len();
    let mut ans = 0;
    for i in 1..height - 1 {
        for j in 1..width - 1 {
            if input[i][j] != 'A' {
                continue;
            }

            if (input[i - 1][j - 1] == 'M' && input[i + 1][j + 1] == 'S'
                || input[i - 1][j - 1] == 'S' && input[i + 1][j + 1] == 'M')
                && (input[i - 1][j + 1] == 'M' && input[i + 1][j - 1] == 'S'
                    || input[i - 1][j + 1] == 'S' && input[i + 1][j - 1] == 'M')
            {
                ans += 1;
            }
        }
    }

    println!("{ans}");
}
