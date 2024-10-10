const BLANK_SYMBOL: char = '.';
const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const SYMBOLS: [char; 11] = ['!', '*', '@', '/', '+', '$', '=', '&', '-', '#', '%'];

pub fn solve(input: String) {
    println!("-- Day 3 --");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", "Not Implemented");
}

fn part1(input: String) -> String {
    check_declared_symbols(&input);
    let puzzle_grid = parse_puzzle(input);

    let mut sum:u32 = 0;

    let is_symbol_or_blank = |cell: char| cell == BLANK_SYMBOL || SYMBOLS.contains(&cell);

    let process_digit = |digit: &String, adjacent_symbol: bool, sum: &mut u32| {
        if !digit.is_empty() && adjacent_symbol {
            *sum += parse_digit(digit);
        }
    };
    
    let reset_digit = |digit: &mut String, adjacent_symbol: &mut bool| {
        digit.clear();
        *adjacent_symbol = false;
    };

    let mut digit = String::new();
    // Used to track Has Adjacent Symbol
    let mut adjacent_symbol = false;

    for (x, row) in puzzle_grid.grid.iter().enumerate() {
        for (y, &cell) in row.iter().enumerate() {
            // If '.' or a SYMBOL process digit and skip iteration.
            if is_symbol_or_blank(cell) {
                process_digit(&digit, adjacent_symbol, &mut sum);
                reset_digit(&mut digit, &mut adjacent_symbol);
                continue;
            }

            if DIGITS.iter().any(|&digit| digit == cell) {
                digit.push(cell);
                if !adjacent_symbol {
                    adjacent_symbol = puzzle_grid.has_adjacent_symbol(x, y);
                }
            }
        }

        process_digit(&digit, adjacent_symbol, &mut sum);
        reset_digit(&mut digit, &mut adjacent_symbol);
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
        // Convert to isize to prevent overflow errors on negative values.
        let x = x as isize;
        let y = y as isize;

        // Helper function to check if a symbol exists at a specific position
        let is_symbol_at = |x: isize, y: isize| {
            // Bounds Checking
            if x >= 0 && x < self.grid.len() as isize && y >= 0 && y < self.grid[x as usize].len() as isize {
                SYMBOLS.iter().any(|&symbol| symbol == self.grid[x as usize][y as usize])
            } else {
                false
            }
        };

        // Check all 8 possible directions
        is_symbol_at(x - 1, y)     || // above
        is_symbol_at(x + 1, y)     || // below
        is_symbol_at(x, y - 1)     || // left
        is_symbol_at(x, y + 1)     || // right
        is_symbol_at(x - 1, y - 1) || // top-left
        is_symbol_at(x - 1, y + 1) || // top-right
        is_symbol_at(x + 1, y - 1) || // bottom-left
        is_symbol_at(x + 1, y + 1)    // bottom-right
    }
}