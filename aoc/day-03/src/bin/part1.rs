use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct GridNumber {
    number: usize,
    row: usize,
    start_col: usize,
    end_col: usize
}

#[derive(Debug)]
struct Symbol {
    row: usize,
    col: usize
}

fn get_numbers_from_grid(input: &str) -> Vec<GridNumber> {
    let mut numbers = Vec::new();

    for (row, line) in input.lines().enumerate() {
        let mut current_num = String::new();
        let mut start_col = 0;

        for (col, ch) in line.chars().enumerate() {
            if ch.is_digit(10) {
                if current_num.is_empty() {
                    start_col = col;
                }
                current_num.push(ch.clone());
            } else if !current_num.is_empty() {
                let end_col = start_col + current_num.len() - 1;
                if let Ok(num) = current_num.parse::<usize>() {
                    numbers.push(GridNumber {number: num, row, start_col, end_col});
                }
                current_num = String::new();
            }
        } 
    }

    numbers
}

fn get_symbols_from_grid(input: &str) -> Vec<Symbol> {
    input.lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, ch)| !ch.is_digit(10) && ch != '.')
                .map(move |(col, _)| Symbol {row, col})
        })
        .collect()
}

fn symbols_to_hashset(symbols: Vec<Symbol>) -> HashSet<(usize, usize)> {
    let mut set = HashSet::new();

    for symbol in symbols {
        set.insert((symbol.row, symbol.col));
    }

    set
}

fn number_near_symbol(number: &GridNumber, symbols: HashSet<(usize, usize)>) -> bool {
    let row_start = if number.row > 0 { number.row - 1 } else { number.row };
    let col_start = if number.start_col > 0 { number.start_col - 1 } else { number.start_col };

    for row in row_start..=(number.row + 1) {
        for col in col_start..=(number.end_col + 1) {
            dbg!(row, col);
            if symbols.contains(&(row, col)) {
                return true;
            }
        }
    }

    false

}

fn part1(input: &str) -> usize {
    let numbers = get_numbers_from_grid(&input);
    let symbols = get_symbols_from_grid(&input);

    dbg!(&symbols);

    let sym_set = symbols_to_hashset(symbols);

    numbers.iter()
        .filter(|&num| number_near_symbol(num, sym_set.clone()))
        .map(|number| number.number)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");
        assert_eq!(result, 4361);
    }
}