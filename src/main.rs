mod puzzles;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 1 { panic!("Expected day as an argument.") }

    let day_arg = args[1].as_str();
    let day = match day_arg.parse::<u32>() {
        Ok(d) => d,
        Err(_) => { panic!("Day {} is not a valid day. Please use a number.", day_arg) }
    };
    
    puzzles::solve_puzzle(day);
}
