advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut zones = input.split("\n\n");
    let seeds_zone = zones.next().unwrap();

    let maps = zones
        .map(|map| {
            map.as_bytes()
                .split(|&b| b == b'\n')
                .skip(1)
                .map(|range| {
                    range
                        .split(|&b| b == b' ')
                        .enumerate()
                        .fold([0; 3], |mut acc, (idx, n)| {
                            acc[idx] = parse_slice_to_number(n);
                            acc
                        })
                })
                .map(|[target, source, step]| [target - source, source, step + source])
                .collect::<Vec<[_; 3]>>()
        })
        .collect::<Vec<Vec<[_; 3]>>>();

    seeds_zone
        .as_bytes()
        .split(|&b| b == b' ')
        .skip(1)
        .map(parse_slice_to_number)
        .map(|seed| {
            maps.iter().fold(seed, |seed, ranges| {
                for &[transformation, begin_range, end_range] in ranges.iter() {
                    if seed >= begin_range && seed < end_range {
                        return seed + transformation;
                    }
                }
                seed
            })
        })
        .min()
        .map(|n| n as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_slice_to_number(slice: &[u8]) -> i64 {
    slice.iter().fold(0, |acc, &b| acc * 10 + (b - b'0') as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
