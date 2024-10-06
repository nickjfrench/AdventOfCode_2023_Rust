use crate::puzzles::day1::number::{Number, instantiate_numbers};

mod number;

pub fn solve(input: String) {
    println!("-- Day 1 --");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input.clone()));
}

fn part1(input: String) -> String {
    let mut total: u32 = 0;

    for line in input.lines() {
        let mut first_num: Option<u32> = None;
        let mut last_num: Option<u32> = None;

        for c in line.chars() {
            if let Some(digit) = c.to_digit(10) {
                if first_num.is_none() {
                    first_num = Some(digit);
                }

                // Always save as the last number even if it's the first.
                last_num = Some(digit)
            }
        }

        total += process_digit(first_num, last_num)
    }

    total.to_string()
}

fn process_digit(first_num: Option<u32>, last_num: Option<u32>) -> u32 {
    // returns 0 if the line contained no digits.
    let mut digit: u32 = 0;

    if let Some(num) = first_num {
        digit = num * 10
    }

    // If there was only one number, then reassign it to the second digit.
    if let Some(num) = last_num {
        digit += num
    }

    digit
}

pub fn part2(input: String) -> String {
    let numbers: [Number; 9] = instantiate_numbers();

    let mut total: u32 = 0;

    for line in input.lines() {
        let mut first_num: Option<u32> = None;
        let mut last_num: Option<u32> = None;

        for (i, c) in line.chars().enumerate() {
            let mut number: Option<u32> = None;

            if let Some(digit) = c.to_digit(10) {
                // Check for number as digit
                number = Some(digit)
            } else {
                // Else check for a word
                let numbers = Number::numbers_by_first_letter(&numbers, c);

                if !numbers.is_empty() {
                    for num in numbers {
                        // Check the rest of line length is long enough and if there's a match
                        let num_len = num.len();
                        let remaining_line = line.len() - i;

                        if num_len <= remaining_line && num.are_you(&line[i..i+num.len()]) {
                            number = Some(num.digit);

                            break; // We take the first possible word
                        }
                    }
                }
            }

            if let Some(digit) = number {
                if first_num.is_none() {
                    first_num = Some(digit);
                }

                // Always save as the last number even if it's the first.
                last_num = Some(digit);
            }
        }

        total += process_digit(first_num, last_num)
    }

    total.to_string()
}

