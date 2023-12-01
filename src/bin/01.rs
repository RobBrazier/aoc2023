advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let items = input.trim_end().split('\n').collect::<Vec<&str>>();
    let mut total = 0;
    for item in &items {
        let digits = item
            .chars()
            .filter(|c| c.is_digit(10))
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

fn find_digits(input: &str) -> Option<String> {
    let mut result = String::new();
    let mut pointer = String::new();
    for c in input.chars().collect::<Vec<char>>().iter() {
        if c.is_digit(10) {
            result.push_str(c.to_string().as_str());
        } else {
            pointer.push_str(c.to_string().as_str());
        }
        let named = match pointer.as_str() {
            x if x.contains("one") => "1",
            x if x.contains("two") => "2",
            x if x.contains("three") => "3",
            x if x.contains("four") => "4",
            x if x.contains("five") => "5",
            x if x.contains("six") => "6",
            x if x.contains("seven") => "7",
            x if x.contains("eight") => "8",
            x if x.contains("nine") => "9",
            _ => "",
        };
        if named != "" {
            result.push_str(named);
            pointer = pointer.chars().last().unwrap().to_string();
        }
    }
    // println!("result: {result}");
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let items = input.trim_end().split('\n').collect::<Vec<&str>>();
    let mut total = 0;
    for item in &items {
        let digits = find_digits(item).unwrap().chars().collect::<Vec<char>>();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
