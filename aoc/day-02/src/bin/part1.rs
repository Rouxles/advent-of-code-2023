

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    dbg!(output);
}

fn all_leq(a: (i32, i32, i32), b: (i32, i32, i32)) -> bool {
    a.0 <= b.0 && a.1 <= b.1 && a.2 <= b.2
}


fn part1(input: &str) -> i32 {
    let counts = (12, 13, 14);
    let mut result = 0;

    for game in input.lines() {
        let parts: Vec<&str> = game.splitn(2, ":").collect();
        let game_number = parts[0].trim().split(" ").nth(1).unwrap().parse::<i32>().unwrap();
        let cube_sets: Vec<(i32, i32, i32)> = parts[1].trim().split(";").map(|set| {
            let set = set.trim();
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;

            let parts: Vec<&str> = set.split(",")
                .map(|s| s.trim())
                .collect();

            for part in &parts {
                let part_split: Vec<&str> = part.split_whitespace().collect();
                let amount = part_split[0].parse::<i32>().unwrap();
                let color = part_split[1];

                match color {
                    "red" => r = amount,
                    "green" => g = amount,
                    "blue" => b = amount,
                    _ => {}
                }
            }
          
            (r, g, b)
        }).collect();
        

        let valid_game = cube_sets.iter().all(|&set| all_leq(set, counts));

        if valid_game {
            result += game_number;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, 8);
    }
}