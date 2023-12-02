use regex::Regex;



fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    dbg!(output);
}


fn part1(input: &str) -> u32 {
    let lines: Vec<&str> = input.split("\n").collect();

    let mut result = 0;
    let re = Regex::new(r"\D").unwrap();

    for line in lines {
        let numbers_only = re.replace_all(&line, "").to_string();

        let left = numbers_only.chars().next().unwrap().to_digit(10).unwrap();
        let right = numbers_only.chars().last().unwrap().to_digit(10).unwrap();
        
        result = result + left * 10 + right;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet");
        assert_eq!(result, 142);
    }
}