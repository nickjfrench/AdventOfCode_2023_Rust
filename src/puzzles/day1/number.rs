pub struct Number {
    pub digit: u32,
    pub word: String,
}

impl Number {
    pub fn new(word: String, digit: u32) -> Result<Self, &'static str>{
        if !word.is_empty() {
            return Ok(Self {
                word, digit
            })
        }

        Err("the word value cannot be empty.")
    }
    pub fn len(&self) -> usize {
        self.word.len()
    }

    pub fn are_you(&self, s: &str) -> bool {
        self.word == s.to_lowercase()
    }

    pub fn first_letter(&self) -> char {
        self.word.chars().next().unwrap()
    }

    pub fn numbers_by_first_letter(numbers: &[Self], c: char) -> Vec<&Self> {
        let mut nums : Vec<&Self> = Vec::new();
        for num in numbers {
            if num.first_letter() == c {
                nums.push(num);
            }
        }
        nums
    }
}

pub fn instantiate_numbers() -> [Number; 9] {
    let numbers: [Number; 9] = [
        Number::new(String::from("one"), 1).unwrap(),
        Number::new(String::from("two"), 2).unwrap(),
        Number::new(String::from("three"),3).unwrap(),
        Number::new(String::from("four"),4).unwrap(),
        Number::new(String::from("five"),5).unwrap(),
        Number::new(String::from("six"),6).unwrap(),
        Number::new(String::from("seven"),7).unwrap(),
        Number::new(String::from("eight"),8).unwrap(),
        Number::new(String::from("nine"),9).unwrap()
    ];

    numbers
}