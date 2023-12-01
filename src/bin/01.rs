use std::{convert::Infallible, str::FromStr};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .flat_map(|line| {
                let mut chars = line.chars();
                let first = chars.find(char::is_ascii_digit)?;
                let last = chars.rfind(char::is_ascii_digit).unwrap_or(first);
                format!("{}{}", first, last).parse::<u32>().ok()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    const NUMBER_PATTERNS: &[&str] = &[
        "1", "2", "3", "4", "5", "6", "7", "8", "9", // 1-9
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", // 1-9
    ];

    Some(
        input
            .lines()
            .flat_map(|line| {
                let mut find_first = NUMBER_PATTERNS
                    .iter()
                    .map(|pattern| (pattern, line.find(pattern)))
                    .filter(|(_, index)| index.is_some())
                    .collect::<Vec<_>>();
                find_first.sort_by(|(_, a), (_, b)| a.cmp(b));
                let first = find_first.first().unwrap();
                let mut find_last = NUMBER_PATTERNS
                    .iter()
                    .map(|pattern| (pattern, line.rfind(pattern)))
                    .filter(|(_, index)| index.is_some())
                    .collect::<Vec<_>>();
                find_last.sort_by(|(_, a), (_, b)| a.cmp(b));
                let last = find_last.last().unwrap();

                format!(
                    "{}{}",
                    first.0.parse::<Num>().unwrap().0,
                    last.0.parse::<Num>().unwrap().0
                )
                .parse::<u32>()
            })
            .sum(),
    )
}

struct Num(u32);

impl FromStr for Num {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" | "one" => Num(1),
            "2" | "two" => Num(2),
            "3" | "three" => Num(3),
            "4" | "four" => Num(4),
            "5" | "five" => Num(5),
            "6" | "six" => Num(6),
            "7" | "seven" => Num(7),
            "8" | "eight" => Num(8),
            "9" | "nine" => Num(9),
            _ => panic!("Invalid number"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // example
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
        // real input
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(55172));
    }

    #[test]
    fn test_part_two() {
        // example
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
        // real input
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(54925));
    }
}
