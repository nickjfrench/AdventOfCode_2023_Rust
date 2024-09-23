use std::fs;

fn main() {
    let puzzle_input: String = fs::read_to_string("puzzle_input.txt").unwrap();

    let mut total: u32 = 0;

    for line in puzzle_input.lines() {
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

        total += process_newline(first_num, last_num)
    }

    println!("\n-- Total --");
    println!("{}", total)
}

fn process_newline(first_num: Option<u32>, last_num: Option<u32>) -> u32 {
    let mut digit: u32 = 0;

    println!("-- New Line Reached --");
    println!("First Number: {}", first_num.unwrap());
    println!("Last Number: {}", last_num.unwrap());

    if let Some(num) = first_num {
        digit = calc_first_digit(num)
    }

    // If there was only one number, then reassign it to the second digit.
    if let Some(num) = last_num {
        digit += num
    }

    println!("Digit: {}", digit);

    digit
}

fn calc_first_digit(num: u32) -> u32 {
    num * 10
}
