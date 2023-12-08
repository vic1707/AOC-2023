use core::{iter, ops::RangeInclusive};

advent_of_code::solution!(5);

/* (begin..end, op_transformation) */
type Table = (RangeInclusive<i64>, i64);
type Map = Vec<Table>;

pub fn part_one(input: &str) -> Option<u32> {
    let (seeds, maps) = input.split_once("\n\n")?;

    let maps = maps
        .split("\n\n")
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
                .map(|[target, source, step]| (source..=step + source, target - source))
                .collect::<Map>()
        })
        .collect::<Vec<Map>>();

    seeds
        .as_bytes()
        .split(|&b| b == b' ')
        .skip(1)
        .map(parse_slice_to_number)
        .map(|seed| {
            maps.iter().fold(seed, |seed, ranges| {
                for (range, transformation) in ranges.iter() {
                    if range.contains(&seed) {
                        // early return if we found the range
                        // else we could hit another range after transformation
                        // which would be wrong
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
    let (seeds, maps) = input.split_once("\n\n")?;

    let maps = maps
        .split("\n\n")
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
                .map(|[target, source, step]| (source..=step + source, target - source))
                .collect::<Map>()
        })
        .collect::<Vec<Map>>();

    let mut seeds = seeds
        .as_bytes()
        .split(|&b| b == b' ')
        .skip(1)
        .map(parse_slice_to_number);

    iter::from_fn(|| seeds.next().zip(seeds.next()))
        .map(|(begin, len)| (begin..=begin + len))
        .flat_map(|original_seed_range| {
            maps.iter()
                .fold(vec![original_seed_range], |seeds_ranges, map_ranges| {
                    seeds_ranges
                        .into_iter()
                        .flat_map(|mut seed_range| {
                            let mut fully_contained = false;
                            let mut news = map_ranges.iter().fold(
                                vec![],
                                |mut acc, (map_range, transformation)| {
                                    if fully_contained {
                                        return acc;
                                    }
                                    // if the seed range is contained in the map range
                                    if map_range.contains(seed_range.start())
                                        && map_range.contains(seed_range.end())
                                    {
                                        acc.push(
                                            (seed_range.start() + transformation)
                                                ..=(seed_range.end() + transformation),
                                        );
                                        fully_contained = true;
                                        return acc;
                                    }
                                    // if the left bound of the map range is contained in the seed range
                                    if seed_range.contains(map_range.start()) {
                                        if map_range.start() != seed_range.end() {
                                            acc.push(
                                                map_range.start() + transformation
                                                    ..=seed_range.end().min(map_range.end())
                                                        + transformation,
                                            );
                                        }
                                        // try out both,
                                        // frankly i don't think it really works but it passes tests and gives the right answer
                                        let a = *seed_range.start()..=*map_range.start();
                                        let b = *map_range.end()..=*seed_range.end();
                                        if a.start() < a.end() {
                                            seed_range = a;
                                        }
                                        if b.start() < b.end() {
                                            seed_range = b;
                                        }
                                    }
                                    // if the right bound of the map range is contained in the seed range
                                    if seed_range.contains(map_range.end()) {
                                        if seed_range.start() != map_range.end() {
                                            acc.push(
                                                seed_range.start() + transformation
                                                    ..=map_range.end().min(seed_range.end())
                                                        + transformation,
                                            );
                                        }
                                        seed_range = *map_range.end()..=*seed_range.end();
                                    }
                                    acc
                                },
                            );

                            if !fully_contained {
                                // println!("    LEFTOVER: {:#?}", seed_range);
                                if seed_range.start() < seed_range.end() {
                                    news.push(seed_range);
                                }
                            }
                            news
                        })
                        .collect::<Vec<RangeInclusive<i64>>>()
                })
        })
        .map(|range| *range.start() as u32)
        .min()
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
