const BLANK_SYMBOL: char = '.';
const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const SYMBOLS: [char; 11] = ['!', '*', '@', '/', '+', '$', '=', '&', '-', '#', '%'];

pub fn solve(input: String) {
    println!("-- Day 3 --");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", "Not Implemented");
}

fn part1(input: String) -> String {
//     let input = String::from("467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..");

    check_declared_symbols(&input);
    let puzzle_grid = parse_puzzle(input);

    let mut sum:u32 = 0;

    for x in 0..puzzle_grid.grid.len() {
        let mut digit = String::new();

        // Used to track Has Adjacent Symbol
        let mut adjacent_symbol = false;

        for y in 0..puzzle_grid.grid[x].len() {
            let cell = puzzle_grid.grid[x][y];
            // If '.' or a SYMBOL process digit and skip iteration.
            if cell == BLANK_SYMBOL || SYMBOLS.iter().any(|&symbol| symbol == cell) {
                if !digit.is_empty() && adjacent_symbol {
                    sum += parse_digit(&digit);
                }

                digit = String::new();
                adjacent_symbol = false;

                continue;
            }

            if DIGITS.iter().any(|&digit| digit == cell) {
                digit.push(cell);
                if !adjacent_symbol {
                    adjacent_symbol = puzzle_grid.has_adjacent_symbol(x, y);
                }
            }
        }

        if !digit.is_empty() && adjacent_symbol {
            sum += parse_digit(&digit);
        }
    }

    sum.to_string()
}

fn parse_digit(digit: &str) -> u32 {
    digit.parse::<u32>().unwrap_or(0)
}

fn check_declared_symbols(input: &String) {
    // Will print any symbols not declared in SYMBOLS.
    // Useful to ensure all symbols have been entered from the input.
    let mut missing_symbols: Vec<char> = Vec::new();
    for c in input.chars() {
        // Check c is not any of the following:
        // '/n', '.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
        // And check c is not already declared in SYMBOLS or missing_symbols.
        if  c != '\n'
            && c != BLANK_SYMBOL
            && !DIGITS.iter().any(|&digit| digit == c)
            && !SYMBOLS.iter().any(|&symbol| symbol == c)
            && !missing_symbols.iter().any(|&symbol| symbol == c)
        {
            missing_symbols.push(c);
        }
    }

    if !missing_symbols.is_empty() {
        panic!("Missing symbols from SYMBOLS: {:?}", missing_symbols);
    } else {
        println!("No missing symbols from SYMBOLS.");
    }
}

fn parse_puzzle(input: String) -> PuzzleGrid {
    let puzzle_grid = PuzzleGrid::new(input);

    puzzle_grid.print_size();
    puzzle_grid.print();

    puzzle_grid
}

#[derive(Debug)]
pub struct PuzzleGrid {
    grid: Vec<Vec<char>>,
}

impl PuzzleGrid {
    pub fn new(input: String) -> Self {
        let mut grid: Vec<Vec<char>> = Vec::new();

        for line in input.lines() {
            grid.push(line.chars().collect());
        }

        Self {
            grid,
        }
    }

    pub fn print_size(&self) {
        println!("Grid: width = {}, height = {}", self.grid.len(), self.grid[0].len());
    }

    pub fn print(&self) {
        for x in &self.grid {
            for y in x.iter() { print!("{}", y) }
            println!();
        }
    }

    pub fn has_adjacent_symbol(&self, x: usize, y: usize) -> bool {
        let above = if x != 0 {
            SYMBOLS.iter().any(|&symbol| symbol == self.grid[x-1][y])
        } else {
            false
        };

        let below = if x != self.grid.len()-1 {
            SYMBOLS.iter().any(|&symbol| symbol == self.grid[x+1][y])
        } else {
            false
        };

        let left = if y != 0 {
            SYMBOLS.iter().any(|&symbol| symbol == self.grid[x][y-1])
        } else {
            false
        };

        let right = if y != self.grid[x].len()-1 {
            SYMBOLS.iter().any(|&symbol| symbol == self.grid[x][y+1])
        } else {
            false
        };

        let top_left = if x != 0 && y != 0 {
            SYMBOLS.iter().any(|&symbol| symbol == self.grid[x-1][y-1])
        } else {
            false
        };

        let top_right = if x != 0 && y != self.grid[x].len()-1  {
            SYMBOLS.iter().any(|&symbol| symbol == self.grid[x-1][y+1])
        } else {
            false
        };

        let bottom_left = if y != 0 && x != self.grid.len()-1 {
            SYMBOLS.iter().any(|&symbol| symbol == self.grid[x+1][y-1])
        } else {
            false
        };

        let bottom_right = if y != self.grid[x].len()-1 && x != self.grid.len()-1 {
            SYMBOLS.iter().any(|&symbol| symbol == self.grid[x+1][y+1])
        } else {
            false
        };

        above || below || left || right || top_left || top_right || bottom_left || bottom_right
    }
}