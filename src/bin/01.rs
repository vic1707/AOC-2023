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

const NAMED_NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .flat_map(|line| {
                let mut bytes = line.bytes().enumerate();

                let mut first = None;
                while let Some((idx, ch)) = bytes.next() {
                    if ch.is_ascii_digit() {
                        first.replace(ch - b'0');
                        break;
                    }

                    // if ch is the first char of a named number
                    if let Some((val, name)) = NAMED_NUMBERS
                        .iter()
                        .enumerate()
                        .find(|(_, name)| name.starts_with(ch as char) && {
                            let len = name.len();
                            if idx + len >= line.len() {
                                return false;
                            }
                            let slice = &line[idx..idx + len];
                            let n = *name.to_owned();
                            slice == n
                        })
                    {
                        first.replace(val as u8 + 1);
                        break;
                    }
                }

                let mut last = None;
                while let Some((idx, ch)) = bytes.next_back() {
                    if ch.is_ascii_digit() {
                        last.replace(ch - b'0');
                        break;
                    }

                    // if ch is the first char of a named number
                    if let Some((val, _)) = NAMED_NUMBERS.iter().enumerate().find(|(_, name)| {
                        name.ends_with(ch as char) && {
                            let len = name.len();
                            if idx + 1 < len {
                                return false;
                            }
                            let slice = &line[idx + 1 - len..idx + 1];
                            let n = *name.to_owned();
                            slice == n
                        }
                    }) {
                        last.replace(val as u8 + 1);
                        break;
                    }
                }

                if let Some(first) = first {
                    if let Some(last) = last {
                        format!("{}{}", first, last).parse::<u32>().ok()
                    } else {
                        format!("{}{}", first, first).parse::<u32>().ok()
                    }
                } else {
                    format!("{}{}", last.unwrap(), last.unwrap()).parse::<u32>().ok()
                }
            })
            .sum(),
    )
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
