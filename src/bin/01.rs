advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let items = input.trim_end().split('\n').collect::<Vec<&str>>();
    let mut total = 0;
    for item in &items {
        let digits = item
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<Vec<char>>();
        let len = digits.len();
        if len > 0 {
            let value = format!("{}{}", digits[0], digits[len - 1]);
            let addition = value.parse::<u32>().unwrap();
            // println!("addition: {addition}");
            total += addition;
        }
    }
    Some(total)
}

fn find_digits(input: &str, names: bool) -> String {
    let mut result = String::new();
    let mut pointer = String::new();
    for c in input.chars().collect::<Vec<char>>().iter() {
        if c.is_ascii_digit() {
            result.push_str(c.to_string().as_str());
        } else {
            pointer.push_str(c.to_string().as_str());
        }
        if names {
            let named = match pointer.as_str() {
                x if x.contains("one") => Some("1"),
                x if x.contains("two") => Some("2"),
                x if x.contains("three") => Some("3"),
                x if x.contains("four") => Some("4"),
                x if x.contains("five") => Some("5"),
                x if x.contains("six") => Some("6"),
                x if x.contains("seven") => Some("7"),
                x if x.contains("eight") => Some("8"),
                x if x.contains("nine") => Some("9"),
                _ => None,
            };
            if let Some(val) = named {
                result.push_str(val);
                pointer = pointer.chars().last().unwrap().to_string();
            }
        }
    }
    // println!("result: {result}");
    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let items = input.trim_end().split('\n').collect::<Vec<&str>>();
    let result = items
        .iter()
        .map(|item| find_digits(item, true))
        .filter(|item| !item.is_empty())
        .map(|x| x.chars().collect::<Vec<char>>())
        .map(|x| format!("{}{}", x[0], x[x.len() - 1]))
        .map(|x| x.parse::<u32>().unwrap())
        .sum::<u32>();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
