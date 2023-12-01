fn part1() {
    let input = include_str!("../input.txt").split('\n');
    let mut sum = 0;
    for line in input {
        let first = line.find(|c: char| c.is_ascii_digit());
        let last = line.rfind(|c: char| c.is_ascii_digit());
        if let (Some(f), Some(l)) = (first, last) {
            let one = line.chars().nth(f).unwrap();
            let two = line.chars().nth(l).unwrap();
            sum += one.to_digit(10).unwrap() * 10 + two.to_digit(10).unwrap();
        }
    }

    println!("{}", sum);
}

fn part2() {
    let numbers: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let input = include_str!("../input.txt").split('\n');
    let mut sum = 0;
    for line in input {
        let mut first_number_index: i32 = -1;
        let mut last_number_index: i32 = -1;
        let mut first_number = String::new();
        let mut last_number = String::new();
        for (pos, n) in numbers.iter().enumerate() {
            if let Some(f) = line.find(n) {
                if first_number_index == -1 || (first_number_index >= f as i32) {
                    first_number_index = f as i32;
                    first_number = pos.to_string();
                }
            }
            if let Some(l) = line.rfind(n) {
                if last_number_index == -1 || (last_number_index <= l as i32) {
                    last_number_index = l as i32;
                    last_number = pos.to_string();
                }
            }
        }

        let first_digit = line.find(|c: char| c.is_ascii_digit());
        let last_digit = line.rfind(|c: char| c.is_ascii_digit());
        if let (Some(f), Some(l)) = (first_digit, last_digit) {
            let one = if !first_number.is_empty() && (first_number_index < f as i32) {
                first_number
            } else {
                String::from(line.chars().nth(f).unwrap())
            };

            let two = if !last_number.is_empty() && (last_number_index > l as i32) {
                last_number
            } else {
                String::from(line.chars().nth(l).unwrap())
            };

            let mut calibration_value = one;
            calibration_value.push_str(&two);
            sum += calibration_value.parse::<i32>().unwrap();
        }
    }

    println!("{}", sum);
}

fn main() {
    part1();
    part2();
}
