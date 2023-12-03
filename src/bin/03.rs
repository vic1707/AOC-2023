use std::{
    collections::HashSet,
    iter::{Enumerate, Peekable},
    ops::RangeInclusive,
    str::Bytes,
};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines_el = input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            get_line_numbers_and_symbols(idx, &mut line.bytes().enumerate().peekable())
        })
        .peekable();

    let mut numbers: HashSet<Number> = HashSet::new();
    while let Some((symbols_l1, numbers_l1)) = lines_el.next() {
        if let Some((symbols_l2, numbers_l2)) = lines_el.peek() {
            numbers_l1.into_iter().for_each(|num| {
                let ok_range = extend_range(&num.1 .0);
                symbols_l1.iter().for_each(|(_, pos_s1)| {
                    if ok_range.contains(&pos_s1.0) {
                        numbers.insert(num.clone());
                    }
                });
                symbols_l2.iter().for_each(|(_, pos_s2)| {
                    if ok_range.contains(&pos_s2.0) {
                        numbers.insert(num.clone());
                    }
                });
            });

            symbols_l1.iter().for_each(|(_, pos_s1)| {
                numbers_l2.iter().for_each(|num| {
                    let ok_range = extend_range(&num.1 .0);
                    if ok_range.contains(&pos_s1.0) {
                        numbers.insert(num.clone());
                    }
                });
            });
        }
    }

    Some(numbers.iter().map(|(val, _)| val).sum::<usize>() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

// (symbol, (x, line))
type Symbol = (u8, (usize, usize));
// (number, (range_x, line))
type Number = (usize, (RangeInclusive<usize>, usize));

fn get_line_numbers_and_symbols(
    line_idx: usize,
    iter: &mut Peekable<Enumerate<Bytes<'_>>>,
) -> (Vec<Symbol>, Vec<Number>) {
    let mut line_symbols = Vec::new();
    let mut line_numbers = Vec::new();

    loop {
        let Some(begin) = iter
            .by_ref()
            .find(|b| !b.1.is_ascii_alphabetic() && b.1 != b'.')
        else {
            break;
        };

        match begin.1 {
            b'0'..=b'9' => {
                let mut num = usize::from(begin.1 - b'0');
                let mut end = begin.0;
                while let Some(b) = iter.peek() {
                    if b.1.is_ascii_digit() {
                        num = num * 10 + (b.1 - b'0') as usize;
                        end = b.0;
                        iter.next();
                    } else {
                        break;
                    }
                }
                line_numbers.push((num, (begin.0..=end, line_idx)))
            }
            _ => line_symbols.push((begin.1, (begin.0, line_idx))),
        };
    }

    (line_symbols, line_numbers)
}

// if range is 0..=2 then ok_range is 0..=3
// if range is 1..=3 then ok_range is 0..=4
fn extend_range(range: &RangeInclusive<usize>) -> RangeInclusive<usize> {
    let start = range.start();
    if start == &0 {
        0..=range.end() + 1
    } else {
        start - 1..=range.end() + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
