use std::iter::Peekable;
use std::str::Bytes;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let (idx, red, green, blue) = decompose_line(&mut line.bytes().peekable());
                if red > 12 || green > 13 || blue > 14 {
                    return 0;
                }
                idx as u32
            })
            .sum(),
    )
}

// (game_id, red, green, blue)
fn decompose_line(iter: &mut Peekable<Bytes<'_>>) -> (u8, u8, u8, u8) {
    let mut game = (0, 0, 0, 0);
    let mut pull = (0, 0, 0);
    game.0 = parse_next_number(iter);
    'game: loop {
        'pull: loop {
            let num = parse_next_number(iter);
            match parse_next_letter(iter) {
                b'r' => pull.0 += num,
                b'g' => pull.1 += num,
                b'b' => pull.2 += num,
                b => panic!("unexpected color: {}", b as char),
            }

            // skip all letters
            while let Some(&b) = iter.peek() {
                if b.is_ascii_alphabetic() {
                    iter.next();
                } else {
                    break;
                }
            }

            match iter.peek() {
                Some(b';') => {
                    iter.next();
                    update_game(&mut game, pull);
                    pull = (0, 0, 0);
                    break 'pull;
                }
                Some(b',') => {
                    iter.next();
                }
                Some(b'\n') | None => {
                    iter.next();
                    update_game(&mut game, pull);
                    break 'game;
                }
                Some(&b) => panic!("unexpected char: {}", b as char),
            }
        }
    }

    game
}

fn parse_next_number(iter: &mut Peekable<Bytes<'_>>) -> u8 {
    let mut number = 0;
    for b in iter {
        if b.is_ascii_digit() {
            number = number * 10 + (b - b'0');
        } else if number > 0 {
            break;
        }
    }
    number
}

fn parse_next_letter(iter: &mut Peekable<Bytes<'_>>) -> u8 {
    for b in iter {
        if b.is_ascii_alphabetic() {
            return b;
        }
    }
    0
}

fn update_game(game: &mut (u8, u8, u8, u8), pull: (u8, u8, u8)) {
    // replace if number is higher
    if pull.0 > game.1 {
        game.1 = pull.0;
    }
    if pull.1 > game.2 {
        game.2 = pull.1;
    }
    if pull.2 > game.3 {
        game.3 = pull.2;
    }
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
