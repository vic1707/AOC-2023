advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let (times, distances) = input.split_once('\n').unwrap();

    let times = times
        .split_whitespace()
        .skip(1)
        .map(|s| s.bytes().fold(0, |acc, f| acc * 10 + (f - b'0') as u32));

    let distances = distances
        .split_whitespace()
        .skip(1)
        .map(|s| s.bytes().fold(0, |acc, f| acc * 10 + (f - b'0') as u32));

    Some(
        times
            .zip(distances)
            .map(|(t, d)| {
                let x1 = (t - ((t * t - (d << 2)) as f32).sqrt() as u32) / 2;
                t - 2 * x1 + 1 - 2 * u32::from(x1 * (t - x1) <= d)
            })
            .product(),
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
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
