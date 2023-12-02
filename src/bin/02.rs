advent_of_code::solution!(2);

#[derive(Debug, Eq, PartialEq)]
struct SetResult {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<SetResult>,
}

fn parse_game_data(input: &str) -> Vec<Game> {
    let gamelines: Vec<&str> = input.trim().split('\n').collect();
    let mut games: Vec<Game> = vec![];
    for line in gamelines {
        let first_split: Vec<&str> = line.split(':').collect();
        let game_id = first_split[0]
            .replace("Game", "")
            .trim()
            .parse::<u32>()
            .unwrap();
        let mut game_sets: Vec<SetResult> = vec![];
        let sets: Vec<&str> = first_split[1].split(';').collect();
        for set in sets {
            let colours: Vec<&str> = set.split(',').collect();
            let mut red = 0;
            let mut blue = 0;
            let mut green = 0;
            for colour in colours {
                let colour_info: Vec<&str> = colour.trim().split(' ').collect();
                let count = colour_info[0].parse::<u32>().unwrap();
                let colour_name = colour_info[1].to_lowercase();
                match colour_name.as_str() {
                    "red" => red += count,
                    "green" => green += count,
                    "blue" => blue += count,
                    _ => {}
                }
            }
            let set_result = SetResult { red, blue, green };
            game_sets.push(set_result);
        }
        let game = Game {
            id: game_id,
            sets: game_sets,
        };
        games.push(game);
    }
    games
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;
    let games = parse_game_data(input);
    for game in games {
        let search = SetResult {
            red: 12,
            green: 13,
            blue: 14,
        };
        let mut valid: Option<bool> = None;
        for set in game.sets {
            println!("game {} set: {:?}", game.id, set);
            match set.green <= search.green && set.blue <= search.blue && set.red <= search.red {
                true => {
                    valid = Some(true);
                }
                false => {
                    println!("game {} disqualified with set {:?}", game.id, set);
                    valid = None;
                    break;
                }
            }
        }
        if let Some(val) = valid {
            if val {
                println!("{} matches", game.id);
                result += game.id;
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;
    let games = parse_game_data(input);
    for game in games {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for set in game.sets {
            if set.red > red {
                red = set.red;
            }
            if set.blue > blue {
                blue = set.blue;
            }
            if set.green > green {
                green = set.green;
            }
        }
        result += red * green * blue;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
