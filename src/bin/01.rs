advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .flat_map(|line| {
                let mut digits = line.chars().filter(|c| c.is_ascii_digit());
                let first = digits.next();
                let last = digits.last();
                first
                    .map(|c| match last {
                        Some(last) => format!("{}{}", c, last),
                        None => format!("{}{}", c, c),
                    })
                    .and_then(|s| s.parse::<u32>().ok())
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
