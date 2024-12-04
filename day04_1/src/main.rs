use std::fs::read_to_string;

const MAS: [char; 3] = ['M', 'A', 'S'];

fn main() {
    let input: Vec<Vec<_>> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let height = input.len();
    let width = input.first().unwrap().len();
    let mut ans = 0;
    for i in 0..height {
        for j in 0..width {
            if input[i][j] != 'X' {
                continue;
            }

            if j > 2 && input[i][j - 3..j] == ['S', 'A', 'M'] {
                ans += 1;
            }
            if j < width - 3 && input[i][j + 1..j + 4] == MAS {
                ans += 1;
            }
            if i > 2 && (1..4).all(|k| input[i - k][j] == MAS[k - 1]) {
                ans += 1;
            }
            if i < width - 3 && (1..4).all(|k| input[i + k][j] == MAS[k - 1]) {
                ans += 1;
            }
            if i > 2 && j > 2 && (1..4).all(|k| input[i - k][j - k] == MAS[k - 1]) {
                ans += 1;
            }
            if i < height - 3 && j > 2 && (1..4).all(|k| input[i + k][j - k] == MAS[k - 1]) {
                ans += 1;
            }
            if i > 2 && j < width - 3 && (1..4).all(|k| input[i - k][j + k] == MAS[k - 1]) {
                ans += 1;
            }
            if i < height - 3 && j < width - 3 && (1..4).all(|k| input[i + k][j + k] == MAS[k - 1])
            {
                ans += 1;
            }
        }
    }

    println!("{ans}");
}
