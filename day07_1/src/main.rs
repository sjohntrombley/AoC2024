use std::fs::read_to_string;

fn main() {
    let input = parse_input(read_to_string("input.txt").unwrap());
    let mut total_calibration_result = 0;
    for (result, numbers) in input {
        for mut i in 0..2_u32.pow(u32::try_from(numbers.len() - 1).unwrap()) {
            let mut numbers = numbers.iter();
            let mut test_result = *numbers.next().unwrap();
            for n in numbers {
                if i & 1 == 0 {
                    test_result += n;
                } else {
                    test_result *= n;
                }
                i >>= 1;
            }

            if test_result == result {
                total_calibration_result += result;
                break;
            }
        }
    }

    println!("{total_calibration_result}");
}

fn parse_input(input: String) -> Vec<(u64, Vec<u64>)> {
    let mut output = Vec::new();
    for line in input.lines() {
        let (result, numbers) = line.split_once(": ").unwrap();
        output.push((
            result.parse().unwrap(),
            numbers.split(' ').map(|s| s.parse().unwrap()).collect(),
        ));
    }

    output
}
