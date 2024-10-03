use std::error::Error;
use std::fs;

struct Number {
    word: String,
    digit: u32,
}

impl Number {
    fn new(word: String, digit: u32) -> Result<Self, &'static str>{
        if !word.is_empty() {
            return Ok(Self {
                word, digit
            })
        }

        Err("the word value cannot be empty.")
    }
    fn len(&self) -> usize {
        self.word.len()
    }

    fn are_you(&self, s: &str) -> bool {
        self.word == s.to_lowercase()
    }

    fn first_letter(&self) -> char {
        self.word.chars().next().unwrap()
    }

    fn numbers_by_first_letter(numbers: &[Self], c: char) -> Vec<&Self> {
        let mut nums : Vec<&Self> = Vec::new();
        for num in numbers {
            if num.first_letter() == c {
                nums.push(num);
            }
        }
        nums
    }
}

fn main() {
    let puzzle_input: String = fs::read_to_string("puzzle_input.txt").unwrap().to_lowercase();

    let numbers: [Number; 9] = [
        Number::new(String::from("one"), 1).unwrap(),
        Number::new(String::from("two"), 2).unwrap(),
        Number::new(String::from("three"),3).unwrap(),
        Number::new(String::from("four"),4).unwrap(),
        Number::new(String::from("five"),5).unwrap(),
        Number::new(String::from("six"),6).unwrap(),
        Number::new(String::from("seven"),7).unwrap(),
        Number::new(String::from("eight"),8).unwrap(),
        Number::new(String::from("nine"),9).unwrap()];

    let mut total: u32 = 0;

    for line in puzzle_input.lines() {
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn no_blank_word() {
        Number::new(String::new(), 1).unwrap();
    }

    #[test]
    fn are_you() {
        let number: Number = Number {
            word: String::from("one"),
            digit: 1,
        };

        assert!(number.are_you("one"))
    }

    #[test]
    fn numbers_by_first_letter() {
        let numbers: [Number; 2] = [
            Number::new(String::from("two"), 2).unwrap(),
            Number::new(String::from("three"),3).unwrap()
        ];

        let numbers_found = Number::numbers_by_first_letter(&numbers, 't');

        assert_eq!(numbers_found.len(), 2)
    }
}