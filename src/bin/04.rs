use std::{iter::Peekable, ops::Index, str::Bytes};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let mut winning = 0;
                let mut iter = l.bytes().peekable();

                let winning_nums = extract_winning_nums(&mut iter);

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

                if winning <= 1 {
                    winning
                } else {
                    1 << (winning - 1)
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
