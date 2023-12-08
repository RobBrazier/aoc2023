use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum StrengthType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Debug)]
struct Hand {
    bid: u16,
    values: (u8, u8, u8, u8, u8),
    strength: StrengthType,
}

pub fn part_one(input: &str) -> Option<u32> {
    let strength = "23456789TJQKA";
    let hands: Vec<(&str, u16)> = input
        .lines()
        .map(|line| {
            let mut data = line.split_ascii_whitespace();
            let hand = data.next().unwrap();
            let bid = data.next().map(|bid| bid.parse::<u16>().unwrap()).unwrap();
            (hand, bid)
        })
        .collect();

    let mut hand_data: Vec<Hand> = Vec::new();
    for (hand, bid) in hands {
        let mut data: HashMap<char, u8> = HashMap::new();
        for card in hand.chars() {
            if data.contains_key(&card) {
                let count = data.get(&card).unwrap();
                data.insert(card, count + 1);
            } else {
                data.insert(card, 1);
            }
        }

        let v: Vec<u8> = hand
            .chars()
            .map(|card| strength.find(card).unwrap() as u8)
            .collect();

        let mut scores: Vec<u8> = data
            .keys()
            .map(|key| data.get(key).unwrap().to_owned())
            .collect();
        scores.sort_by(|a, b| b.cmp(a));

        let strength = match scores[0] {
            5 => StrengthType::FiveOfAKind,
            4 => StrengthType::FourOfAKind,
            3 => {
                if scores[1] == 2 {
                    StrengthType::FullHouse
                } else {
                    StrengthType::ThreeOfAKind
                }
            }
            2 => {
                if scores[1] == 2 {
                    StrengthType::TwoPair
                } else {
                    StrengthType::OnePair
                }
            }
            _ => StrengthType::HighCard,
        };

        let hand = Hand {
            bid,
            values: (v[0], v[1], v[2], v[3], v[4]),
            strength,
        };
        hand_data.push(hand);
    }
    hand_data.sort_unstable_by_key(|hand| (hand.strength, hand.values));

    let mut total = 0;
    for (i, hand) in hand_data.iter().enumerate() {
        total += (hand.bid as usize * (i + 1)) as u32;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let strength = "23456789TJQKA";
    let hands: Vec<(&str, u16)> = input
        .lines()
        .map(|line| {
            let mut data = line.split_ascii_whitespace();
            let hand = data.next().unwrap();
            let bid = data.next().map(|bid| bid.parse::<u16>().unwrap()).unwrap();
            (hand, bid)
        })
        .collect();

    let mut hand_data: Vec<Hand> = Vec::new();
    for (hand, bid) in hands {
        let mut data: HashMap<char, u8> = HashMap::new();
        for card in hand.chars() {
            if data.contains_key(&card) {
                let count = data.get(&card).unwrap();
                data.insert(card, count + 1);
            } else {
                data.insert(card, 1);
            }
        }

        let v: Vec<u8> = hand
            .chars()
            .map(|card| {
                if card == 'J' {
                    0
                } else {
                    strength.find(card).unwrap() as u8
                }
            })
            .collect();

        let mut scores: Vec<u8> = data
            .keys()
            .map(|key| {
                if *key == 'J' {
                    0_u8
                } else {
                    data.get(key).unwrap().to_owned()
                }
            })
            .collect();
        scores.sort_by(|a, b| b.cmp(a));

        scores[0] += hand.chars().filter(|a| a == &'J').count() as u8;

        let strength = match scores[0] {
            5 => StrengthType::FiveOfAKind,
            4 => StrengthType::FourOfAKind,
            3 => {
                if scores[1] == 2 {
                    StrengthType::FullHouse
                } else {
                    StrengthType::ThreeOfAKind
                }
            }
            2 => {
                if scores[1] == 2 {
                    StrengthType::TwoPair
                } else {
                    StrengthType::OnePair
                }
            }
            _ => StrengthType::HighCard,
        };

        let hand = Hand {
            bid,
            values: (v[0], v[1], v[2], v[3], v[4]),
            strength,
        };
        hand_data.push(hand);
    }
    hand_data.sort_unstable_by_key(|hand| (hand.strength, hand.values));

    let mut total = 0;
    for (i, hand) in hand_data.iter().enumerate() {
        total += (hand.bid as usize * (i + 1)) as u32;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
