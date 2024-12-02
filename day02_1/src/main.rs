use std::fs::read_to_string;
fn main() {
    let reports = parse_input("input.txt");
    println!("{}", reports.into_iter().filter(|r| is_safe(r)).count());
}

fn is_safe(report: &[i8]) -> bool {
    let mut diff_iter = report.iter().zip(report.iter().skip(1)).map(|(x, y)| x - y);
    let diff = diff_iter.next().unwrap();
    if diff == 0 || diff.abs() > 3 {
        return false;
    }
    let s = sign(&diff);
    diff_iter.all(|d| s * d > 0 && d.abs() < 4)
}

fn sign(n: &i8) -> i8 {
    if n == &0 {
        return 0;
    }
    n / n.abs()
}

fn parse_input(path: &str) -> Vec<Vec<i8>> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.split(' ').map(|n| n.parse().unwrap()).collect())
        .collect()
}
