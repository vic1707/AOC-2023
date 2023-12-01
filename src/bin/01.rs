advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut bytes = line.bytes();
                let first = bytes.find(u8::is_ascii_digit).unwrap();
                let last = bytes.rev().find(u8::is_ascii_digit).unwrap_or(first);
                u32::from((first - b'0') * 10 + (last - b'0'))
            })
            .sum(),
    )
}

const NAMED_NUMBERS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| l.as_bytes())
            .map(|line| {
                let mut bytes = line.iter().enumerate();
                let first = bytes.find_map(|byte| find_digit(byte, line)).unwrap();
                let last = bytes
                    .rev()
                    .find_map(|byte| find_digit(byte, line))
                    .unwrap_or(first);

                first * 10 + last
            })
            .sum(),
    )
}

#[inline(always)]
fn find_digit((idx, b): (usize, &u8), line: &[u8]) -> Option<u32> {
    b.is_ascii_digit().then(|| u32::from(b - b'0')).or_else(|| {
        NAMED_NUMBERS
            .iter()
            .position(|&name| line[idx..].starts_with(name))
            .map(|i| i as u32 + 1)
    })
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
