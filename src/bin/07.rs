advent_of_code::solution!(7);

// we have 5 cards with a value of 0..12
// we can represent each card with 4 bits
// so we can represent a hand with 20 bits
// we can use the 3 most significant bits to represent the hand ranking
// (high card, pair, two pairs, three of a kind, full, four of a kind, five of a kind)
// and the 20 least significant bits to represent the cards
pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = input
        .lines()
        .map(|l| l.as_bytes())
        .map(parse_hand)
        .collect::<Vec<_>>();

    hands.sort_unstable();

    Some(
        hands
            .iter()
            .enumerate()
            .map(|(rank, (_, bid))| (rank + 1) as u32 * bid)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_hand(hand_bytes: &[u8]) -> (u32, u32) {
    let bid = hand_bytes[6..]
        .iter()
        .fold(0, |acc, &b| acc * 10 + (b - b'0') as u32);

    let mut hand = [0_u8; 13];
    let mut power: u32 = 0;
    hand_bytes.iter().enumerate().take(5).for_each(|(i, &b)| {
        let ord = match b {
            b'2'..=b'9' => b - b'0' - 2,
            b'T' => 8,
            b'J' => 9,
            b'Q' => 10,
            b'K' => 11,
            b'A' => 12,
            _ => unreachable!(),
        } as u32;
        hand[ord as usize] += 1;
        // save the value of each card in a 4 bits chunk
        power += ord << (4 * (4 - i));
    });

    // a hand can be a
    // - five of a kind
    // - four of a kind
    // - full (three of a kind + pair)
    // - three of a kind
    // - two pairs
    // - pair
    // - high card
    // so hand ranking has 7 levels => 3 bits
    // and we have 5 cards => 20 bits
    // we will put the hand ranking in the 3 most significant bits
    // and the 39 other bits will be used to represent the cards

    // sort from highest number of copies to lowest
    hand.sort_unstable_by(|a, b| b.cmp(a));

    let rank = match hand[0] {
        5 => 6,                 // five of a kind
        4 => 5,                 // four of a kind
        3 if hand[1] == 2 => 4, // full
        3 => 3,                 // three of a kind
        2 if hand[1] == 2 => 2, // two pairs
        2 => 1,                 // pair
        _ => 0,                 // high card
    };

    power += rank << (32 - 3);

    (power, bid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
