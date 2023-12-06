advent_of_code::solution!(6);

use rayon::prelude::*;

fn get_values(line: &str) -> Vec<u32> {
    line.split(':')
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.trim_end().split('\n').collect();
    let times = get_values(lines.first().unwrap());
    let mut distances = get_values(lines.last().unwrap());
    let mut total: u32 = 1;
    for (race_time, race_distance) in times.iter().zip(distances.iter_mut()) {
        println!("race time: {}, race distance: {}", race_time, race_distance);
        let held_range = 1..=*race_time;
        let winners: Vec<u32> = held_range
            .map(|speed| {
                let remaining_time = race_time - speed;
                // println!("remainng time: {}", remaining_time);
                (speed, remaining_time * speed)
            })
            .filter(|x| x.1 > *race_distance)
            .map(|x| x.0)
            .collect();

        let race_result: u32 = winners.len() as u32;
        total *= race_result;
        println!("race result: {}", race_result);
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.trim_end().split('\n').collect();
    let race_time: usize = get_values(lines.first().unwrap())
        .iter()
        .map(|i| i.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let race_distance: usize = get_values(lines.last().unwrap())
        .iter()
        .map(|i| i.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    println!("race time: {}, race distance: {}", race_time, race_distance);
    let held_range = race_distance / race_time;
    println!("held range: {}", held_range);
    let held_range = held_range..=(race_time - held_range);
    let winners: Vec<usize> = held_range
        .into_par_iter()
        .map(|speed| {
            let remaining_time = race_time - speed;
            // println!("remaining time: {}", remaining_time);
            (speed, remaining_time * speed)
        })
        .filter(|x| x.1 > race_distance)
        .map(|x| x.0)
        .collect();

    let race_result: u32 = winners.len() as u32;
    // let race_result = held_range.into_iter().count() as u32;
    Some(race_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
