use std::{iter::Peekable, str::Bytes};

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

    Some(
        input
            .lines()
            .enumerate()
            .map(|(gid, l)| {
                let mut iter = l.bytes().peekable();

                let winning_nums = extract_winning_nums(&mut iter);
                let winning = number_of_winning(&mut iter, &winning_nums);

                if winning != 0 {
                    ((gid + 1)..=(gid + winning as usize)).for_each(|i| {
                        multiplicators[i] += multiplicators[gid];
                    });
                }

                multiplicators[gid]
            })
            .sum(),
    )
}

fn extract_winning_nums(iter: &mut Peekable<Bytes<'_>>) -> Vec<u8> {
    iter.by_ref().find(|b| b == &b':');

    while let Some(b) = iter.peek() {
        if b.is_ascii_digit() {
            break;
        }
        iter.next();
    }

    let mut winnings = vec![];
    loop {
        let num = iter
            .by_ref()
            .take_while(|b| b.is_ascii_digit())
            // warning: this consumes the trailing byte (here a space so it's fine)
            .fold(0, |acc, b| acc * 10 + (b - b'0'));

        winnings.push(num);

        if iter.peek() == Some(&b'|') {
            break;
        }
    }

    winnings
}

fn number_of_winning(iter: &mut Peekable<Bytes<'_>>, winning_nums: &[u8]) -> u32 {
    let mut winning = 0;

    loop {
        while let Some(b) = iter.peek() {
            if b.is_ascii_digit() {
                break;
            }
            iter.next();
        }
        let num = iter
            .by_ref()
            .take_while(|b| b.is_ascii_digit())
            // warning: this consumes the trailing byte (here a space so it's fine)
            .fold(0, |acc, b| acc * 10 + (b - b'0'));

        if winning_nums.contains(&num) {
            winning += 1;
        }

        if iter.peek().is_none() {
            break;
        }
    }

    winning
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
