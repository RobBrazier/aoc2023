use std::collections::HashMap;

advent_of_code::solution!(4);

fn parse_numbers(input: &str) -> Vec<u32> {
    input
        .split(' ')
        .filter(|x| x.trim() != "")
        .filter_map(|x| x.parse::<u32>().ok())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let mut total: u32 = 0;
    for line in lines {
        let split1: Vec<&str> = line.split(':').map(|x| x.trim()).collect();
        let card_split: Vec<&str> = split1[1].split('|').collect();
        let winning = parse_numbers(card_split[0]);
        let answers = parse_numbers(card_split[1]);

        let mut winning_numbers: Vec<u32> = vec![];
        for w_num in winning {
            if answers.contains(&w_num) {
                winning_numbers.push(w_num);
            }
        }
        let mut last_num: u32 = 0;
        // println!("winning numbers: {:?}", winning_numbers);
        let mut score: u32 = 0;
        for (i, _) in winning_numbers.iter().enumerate() {
            if i <= 1 {
                last_num = 1;
                score += 1;
            } else {
                last_num *= 2;
                score += last_num
            }
        }
        // println!("score: {}", score);
        total += score;
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let mut total: u32 = lines.len() as u32;
    let mut won: HashMap<u32, u32> = HashMap::new();
    for line in lines {
        let split1: Vec<&str> = line.split(':').map(|x| x.trim()).collect();
        let card_index: u32 = split1[0].split(' ').map(|x| x.trim()).fold(0, |_, e| {
            if let Ok(val) = e.parse::<u32>() {
                return val;
            }
            0
        });
        // println!("card index: {}", card_index);
        let card_split: Vec<&str> = split1[1].split('|').collect();
        let winning = parse_numbers(card_split[0]);
        let answers = parse_numbers(card_split[1]);

        let mut matching_numbers: Vec<u32> = vec![];
        let copies = *won.get(&card_index).unwrap_or(&0);
        // println!("copies of {}: {}, total won: {:?}", card_index, copies, won);
        for w_num in winning {
            if answers.contains(&w_num) {
                matching_numbers.push(w_num);
            }
        }
        // println!("matching numbers: {:?}", matching_numbers);
        let range_start = card_index + 1;
        let range_end = card_index + matching_numbers.len() as u32;
        for n in range_start..=range_end {
            // println!(
            //     "adding card {}, won by {} with copies {}",
            //     n, card_index, copies
            // );
            if let Some(count) = won.get(&n) {
                won.insert(n, count + copies + 1);
            } else {
                won.insert(n, copies + 1);
            }
        }
        // println!("copies after {}: {:?}", card_index, won);
    }
    won.iter().for_each(|(_, count)| {
        total += count;
    });
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
