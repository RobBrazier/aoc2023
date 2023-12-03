use std::collections::HashMap;

advent_of_code::solution!(3);

#[derive(Debug)]
struct Number {
    value: usize,
    length: usize,
    position: (usize, usize),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Symbol {
    value: char,
    position: (usize, usize),
}

fn parse(input: &str) -> (Vec<Number>, Vec<Symbol>) {
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];

    let lines: Vec<&str> = input.trim().split('\n').collect();

    for (y, line) in lines.iter().enumerate() {
        let mut cursor = String::new();
        let mut characters = line.chars().peekable();
        let mut x: usize = 0;
        while let Some(c) = characters.next() {
            // println!("x: {}, char: {}", x, c);
            if characters.peek().is_none() || !c.is_ascii_digit() {
                // println!("pointer: {:?}, char: {:?}", cursor, c);
                if c != '.' && !c.is_ascii_digit() {
                    // println!("adding symbol {}", c);
                    symbols.push(Symbol {
                        value: c,
                        position: (x, y),
                    });
                } else if c.is_ascii_digit() {
                    cursor.push(c);
                }
                if !cursor.is_empty() {
                    // println!("adding number {}", cursor);
                    numbers.push(Number {
                        value: cursor.parse().unwrap(),
                        length: cursor.len(),
                        position: (x - 1, y),
                    });
                    cursor.clear();
                }
            } else {
                cursor.push(c);
            }
            x += 1;
        }
    }

    (numbers, symbols)
}

fn get_previous_num(num: usize, subtract: usize) -> usize {
    if num > 0 && subtract <= num {
        return num - subtract;
    }
    0
}

fn get_symbols_at_line(symbols: &[Symbol], number: &Number) -> Vec<Symbol> {
    // println!("searching around number {:?}", number);
    let lines: Vec<Symbol> = symbols
        .iter()
        .filter(|s| {
            let y_range = (get_previous_num(number.position.1, 1))..=(number.position.1 + 1);
            y_range.contains(&s.position.1)
        })
        .cloned()
        .collect();

    lines
        .iter()
        .filter(|s| {
            // println!("number: {:?}", number);
            let x_range =
                (get_previous_num(number.position.0, number.length))..=(number.position.0 + 1);
            let symbol_matches = x_range.contains(&s.position.0);
            if symbol_matches {
                // println!("symbol {:?} matches {:?}", s, number);
            }
            symbol_matches
        })
        .cloned()
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (numbers, symbols) = parse(input);

    let mut result: usize = 0;

    for num in numbers {
        let symbols_around = get_symbols_at_line(&symbols, &num);
        if !symbols_around.is_empty() {
            // println!("number: {:?} has symbols: {:?}", num, symbols_around);
            result += num.value;
        } else {
            // println!("number {:?} has no symbols", num);
        }
    }

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (numbers, symbols) = parse(input);

    let mut result: usize = 0;

    let mut gears: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    for num in numbers {
        let symbols_around = get_symbols_at_line(&symbols, &num);
        let gear_symbols: Vec<Symbol> = symbols_around
            .iter()
            .filter(|s| s.value == '*')
            .cloned()
            .collect();
        for gear in gear_symbols {
            if let std::collections::hash_map::Entry::Vacant(e) = gears.entry(gear.position) {
                e.insert(vec![num.value]);
            } else {
                gears.get_mut(&gear.position).unwrap().push(num.value);
            }
        }
    }
    for (_, numbers) in gears.iter() {
        if numbers.len() == 2 {
            // println!("numbers: {:?}, key: {:?}", numbers, key);
            result += numbers[0] * numbers[1];
        }
    }

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
