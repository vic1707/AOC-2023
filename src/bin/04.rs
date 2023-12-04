advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut input_iter = input.as_bytes().iter();
    let colon_index = input_iter.position(|b| b == &b':').unwrap();
    let pipe_index = input_iter.position(|b| b == &b'|').unwrap() + colon_index + 1;

    Some(
        input
            .lines()
            .map(|l| {
                let line = l.as_bytes();
                let winning_nums = &line[colon_index + 1..pipe_index];
                let winning = line[pipe_index + 1..]
                    .chunks_exact(3)
                    .filter(|&n| winning_nums.chunks_exact(3).any(|w| w == n))
                    .count();

                2 << winning >> 2
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut multiplicators = [1_u32; 200];
    let mut input_iter = input.as_bytes().iter();
    let colon_index = input_iter.position(|b| b == &b':').unwrap();
    let pipe_index = input_iter.position(|b| b == &b'|').unwrap() + colon_index + 1;

    Some(
        input
            .lines()
            .enumerate()
            .map(|(gid, l)| {
                let line = l.as_bytes();
                let winning_nums = &line[colon_index + 1..pipe_index];
                line[pipe_index + 1..]
                    .chunks_exact(3)
                    .filter(|&n| winning_nums.chunks_exact(3).any(|w| w == n))
                    .enumerate()
                    .for_each(|(i, _)| {
                        multiplicators[gid + i + 1] += multiplicators[gid];
                    });

                multiplicators[gid]
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
