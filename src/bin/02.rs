use std::str::Bytes;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .enumerate()
            .map(|(idx, line)| {
                let game = decompose_line(&mut line.bytes());
                if game[0] <= 12 && game[1] <= 13 && game[2] <= 14 {
                    return (idx + 1) as u32;
                }
                0
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| decompose_line(&mut line.bytes()).iter().product::<u8>() as u32)
            .sum(),
    )
}

// [red, green, blue]
fn decompose_line(iter: &mut Bytes<'_>) -> [u8; 3] {
    iter.by_ref().find(|b| b == &b':');
    let mut game: [u8; 3] = [0; 3];
    loop {
        // next because current is a space
        iter.next();
        let num = iter
            .by_ref()
            .take_while(|b| b.is_ascii_digit())
            // warning: this consumes the trailing byte (here a space so it's fine)
            .fold(0, |acc, b| acc * 10 + (b - b'0'));
        let letter_idx = usize::from(iter.next().unwrap() % 3);
        game[letter_idx] = game[letter_idx].max(num);

        // `;` appears to be worthless (same behavior as `,`)
        if !iter.by_ref().any(|b| b == b';' || b == b',') {
            break;
        }
    }

    game
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
