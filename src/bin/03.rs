advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let bytes = input.as_bytes();
    let map_width = bytes.iter().position(|&b| b == b'\n').unwrap() as isize;

    Some(
        (0..bytes.len())
            // find all first digits of numbers
            .filter(|&pos| {
                bytes[pos].is_ascii_digit()
                    && !bytes
                        .get(pos.wrapping_sub(1))
                        .map_or(false, u8::is_ascii_digit)
            })
            // extract len and value
            .map(|begin| {
                let mut len = 0;
                let value = bytes[begin..]
                    .iter()
                    .take_while(|&&b| b.is_ascii_digit())
                    .fold(0, |acc, &b| {
                        len += 1;
                        acc * 10 + byte_to_digit(b)
                    });
                (begin as isize, len, value)
            })
            // check if surrounded by symbol
            .filter(|&(begin, len, _)| {
                (-map_width - 2..-map_width + len)
                    .chain([-1, len])
                    .chain(map_width..map_width + len + 2)
                    .any(|j| {
                        bytes
                            .get((begin + j) as usize)
                            .map_or(false, |&b| b != b'.' && b.is_ascii_punctuation())
                    })
            })
            .map(|(_, _, n)| n)
            .sum::<u32>(),
    )
}

#[inline]
fn byte_to_digit(b: u8) -> u32 {
    u32::from(b - b'0')
}

pub fn part_two(input: &str) -> Option<u32> {
    let bytes = input.as_bytes();
    let map_width = bytes.iter().position(|&b| b == b'\n').unwrap();

    Some(
        (0..bytes.len())
            .filter(|&b| bytes[b] == b'*')
            .map(|pos| {
                let s = (pos - map_width - 2..=pos - map_width)
                    .chain(pos - 1..=pos + 1)
                    .chain(pos + map_width..=pos + map_width + 2)
                    .filter(|&i| i != pos && bytes[i].is_ascii_digit())
                    .flat_map(|num_pos| {
                        (num_pos.saturating_sub(2)..=num_pos)
                            .rev()
                            .take_while(|&pos| bytes[pos].is_ascii_digit())
                            .last()
                    })
                    .map(|pos| {
                        bytes[pos..pos + 3]
                            .iter()
                            .take_while(|&&b| b.is_ascii_digit())
                            .fold(0, |acc, b| acc * 10 + byte_to_digit(*b))
                    })
                    .fold((0, 0), |acc, num| {
                        if acc.0 == 0 {
                            (num, acc.1)
                        } else if acc.0 != num && acc.1 == 0 {
                            (acc.0, num)
                        } else {
                            acc
                        }
                    });
                s.0 * s.1
            })
            .sum(),
    )
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
