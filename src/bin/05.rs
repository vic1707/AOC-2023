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
                .collect::<Vec<[u64; 3]>>()
        })
        .collect::<Vec<Vec<[u64; 3]>>>();

    seeds_zone
        .as_bytes()
        .split(|&b| b == b' ')
        .skip(1)
        .map(parse_slice_to_number)
        .map(|seed| {
            maps.iter().fold(seed, |seed, ranges| {
                for &[target, source, step] in ranges.iter() {
                    if seed >= source && seed < source + step {
                        return (seed + target) - source;
                    }
                }
                seed
            })
        })
        .min().map(|n| n as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_slice_to_number(slice: &[u8]) -> u64 {
    slice.iter().fold(0, |acc, &b| acc * 10 + (b - b'0') as u64)
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
        assert_eq!(result, None);
    }
}
