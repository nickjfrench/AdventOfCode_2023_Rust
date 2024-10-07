
pub fn solve(input: String) {
    println!("-- Day 2 --");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", "Not Implemented");
}

fn part1(input: String) -> String{
//     let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let mut games: Vec<Game> = Vec::new();
    for (i, line) in input.lines().enumerate(){
        println!("\nParsing line {}:", i+1);
        games.push(Game::new(&line));
    }

    String::from("Not Implemented")
}

#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn new(line: &str) -> Self {
        let line_split: Vec<&str> = line.split(":").collect();

        if line_split.len() != 2 {
            panic!("Game line has incorrect format: {:?}", line);
        }

        let game_id = Self::parse_game_id(line_split[0]);

        let rounds_split: Vec<&str> = line_split[1].split(";").collect();

        let mut rounds: Vec<Round> = Vec::new();
        
        for round_data in rounds_split {
            let round_scores: Vec<&str> = round_data.split(",").collect();
            
            let mut round: Round = Round {
                red: 0,
                green: 0,
                blue: 0,
            };
            
            for score in round_scores {
                let score = score.trim();
                
                let score_split:Vec<&str> = score.split(" ").collect();
                
                let digit: u32 = match score_split[0].parse() {
                    Ok(n) => n,
                    Err(_) => panic!("Error parsing score digit: {:?}", score_split),
                };
                
                match score_split[1] {
                    "red" => round.red = digit,
                    "green" => round.green = digit,
                    "blue" => round.blue = digit,
                    _ => panic!("Unknown score entry: {:?}", score_split[1]),
                }
            }
            
            rounds.push(round);
        }
        
        println!("Game: {:?}\nRounds: {:#?}", game_id, rounds);
        Self { id: game_id, rounds }
    }

    fn parse_game_id(s: &str) -> u32 {
        let id_str: &str = &s[5..]; // Skip "Game "

        match id_str.parse::<u32>() {
            Ok(id) => id,
            Err(_) => panic!("Failed to parse game id: {}", id_str),
        }
    }
}


