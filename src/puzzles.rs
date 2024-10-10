use std::fs;

mod day1;
mod day2;
mod day3;

pub fn solve_puzzle(day: u32) {
    let data = match get_data(day) {
        Ok(data) => data,
        Err(error) => panic!("{}", error),
    };

    match day {
        // add each day as needed
        1 => day1::solve(data),
        2 => day2::solve(data),
        3 => day3::solve(data),
        _ => panic!("day{} does not exist yet.", &day)
    }
}

fn get_data(day: u32) -> Result<String, String> {
    match fs::read_to_string(format!("data/day{}-puzzle-input.txt", &day)) {
        Ok(data) => Ok(data),
        Err(_) => Err(format!("Could not read puzzle data for day{}.", &day)),
    }
}