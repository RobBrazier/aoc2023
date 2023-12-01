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
            total += addition;
        }
    }
    Some(total)
}

pub fn part_two(_: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
