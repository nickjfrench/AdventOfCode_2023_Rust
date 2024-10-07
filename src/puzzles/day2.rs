
pub fn solve(input: String) {
    println!("-- Day 2 --");
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input.clone()));
}

fn part1(input: String) -> String{
    let bag: BagSet = BagSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games: Vec<Game> = parse_games(&input);

    let mut games_possible: Vec<Game> = Vec::new();
    let mut total: u32 = 0;

    for game in games {
        if game.possible_with_bag(&bag) {
            total += game.id;
            games_possible.push(game);
        }
    }

    format!("{}", total)
}

fn part2(input: String) -> String{
    let games = parse_games(&input);

    let mut sum: u32 = 0;

    for game in games {
        let bag = game.minimum_bag_possible();
        sum += calculate_power(&bag);
    }

    format!("{}", sum)
}

fn parse_games(input: &str) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();

    for line in input.lines(){
        games.push(Game::new(&line));
    }
    // println!("Games: {:#?}", games);

    games
}

fn calculate_power(bag_set: &BagSet) -> u32 {
    bag_set.red.max(1) * bag_set.green.max(1) * bag_set.blue.max(1)
}

#[derive(Debug)]
struct BagSet {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<BagSet>,
}

impl Game {
    fn new(line: &str) -> Self {
        let line_split: Vec<&str> = line.split(":").collect();

        if line_split.len() != 2 {
            panic!("Game line has incorrect format: {:?}", line);
        }

        let game_id = Self::parse_game_id(line_split[0]);

        let rounds_data: Vec<&str> = line_split[1].split(";").collect();

        let rounds: Vec<BagSet> = Self::parse_rounds(rounds_data);

        Self { id: game_id, rounds }
    }

    fn parse_game_id(s: &str) -> u32 {
        let id_str: &str = &s[5..]; // Skip "Game "

        match id_str.parse::<u32>() {
            Ok(id) => id,
            Err(_) => panic!("Failed to parse game id: {}", id_str),
        }
    }

    fn parse_rounds(rounds_data: Vec<&str>) -> Vec<BagSet>{
        let mut rounds: Vec<BagSet> = Vec::new();

        for round_data in rounds_data {
            let round_scores: Vec<&str> = round_data.split(",").collect();

            let mut round: BagSet = BagSet {
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

        rounds
    }

    fn possible_with_bag(&self, bag: &BagSet) -> bool{
        let mut is_possible = true;

        for round in &self.rounds {
            if bag.red < round.red || bag.green < round.green || bag.blue < round.blue {
                is_possible = false;
            }
        }

        is_possible
    }

    fn minimum_bag_possible(&self) -> BagSet {
        let mut minimum_bag = BagSet {
            red: 0,
            green: 0,
            blue: 0,
        };

        for round in &self.rounds {
            if round.red > minimum_bag.red {
                minimum_bag.red = round.red;
            }
            if round.green > minimum_bag.green {
                minimum_bag.green = round.green;
            }
            if round.blue > minimum_bag.blue {
                minimum_bag.blue = round.blue;
            }
        }

        minimum_bag
    }
}


