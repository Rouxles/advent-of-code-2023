use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    dbg!(output);
}


fn part2(input: &str) -> i32 {
    input.lines()
        .map(|line| extract_calibration_value(extract_value(line)))
        .sum::<i32>()
}

fn extract_calibration_value(digits: Vec<i32>) -> i32 {
    let left = digits.first().unwrap();
    let right = digits.last().unwrap();
    left * 10 + right
}

fn extract_value(line: &str) -> Vec<i32> {
    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d");

    let mut digits: Vec<i32> = Vec::new();

    for i in 0..line.len() {
        if let Some(m) = re.clone().expect("invalid input").find(&line[i..]) {
            let digit = match m.as_str() {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => m.as_str().parse::<i32>().unwrap(),
            };
            digits.push(digit)
        }
    }
    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
        "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen");
        assert_eq!(result, 281);
    }
}