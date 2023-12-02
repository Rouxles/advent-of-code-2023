use std::cmp;



fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    input.lines().map(|game| {
        game.splitn(2, ":")
            .nth(1).unwrap().trim()
            .split(";")
        .map(|set| {
            set.trim().split(",")
                .map(|s| s.trim())
                .fold((0, 0, 0), |(r, g, b), part| {
                    let part_split: Vec<&str> = part.split(" ").collect();
                    let amount = part_split[0].parse::<i32>().unwrap();
                    match part_split[1] {
                        "red" => (cmp::max(r, amount), g, b),
                        "green" => (r, cmp::max(g, amount), b),
                        "blue" => (r, g, cmp::max(b, amount)),
                        _ => (r, g, b)
                    }
                })
        })
        .fold((0, 0, 0), |(max_r, max_g, max_b), (r, g, b)| {
            (cmp::max(max_r, r), cmp::max(max_g, g), cmp::max(max_b, b))
        })
    })
    .map(|(r, g, b)| r * g * b)
    .sum()
}


// fn part2(input: &str) -> i32 {
//     let mut result = 0;
//     for game in input.lines() {
//         let parts: Vec<&str> = game.splitn(2, ":").collect();
//         let cube_sets: Vec<(i32, i32, i32)> = parts[1].trim().split(";").map(|set| {
//             let set = set.trim();
//             let mut r = 0;
//             let mut g = 0;
//             let mut b = 0;

//             let parts: Vec<&str> = set.split(",")
//                 .map(|s| s.trim())
//                 .collect();

//             for part in &parts {
//                 let part_split: Vec<&str> = part.split_whitespace().collect();
//                 let amount = part_split[0].parse::<i32>().unwrap();
//                 let color = part_split[1];

//                 match color {
//                     "red" => r = amount,
//                     "green" => g = amount,
//                     "blue" => b = amount,
//                     _ => {}
//                 }
//             }
          
//             (r, g, b)
//         }).collect();
        
//         let mut r = 0;
//         let mut g = 0;
//         let mut b = 0;
//         for set in cube_sets {
//             r = cmp::max(r, set.0);
//             g = cmp::max(g, set.1);
//             b = cmp::max(b, set.2);
//         }

//         result += r * g * b;
//     }

//     result
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, 2286);
    }
}