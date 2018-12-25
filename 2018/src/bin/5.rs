use std::collections::HashSet;

static INPUT: &'static str = include_str!("input/5.txt");

fn main() {
    println!("1. {}", one(INPUT));
    println!("2. {}", two(INPUT));
}

fn one(input: &str) -> usize {
    let mut polymer = input.chars().collect::<Vec<_>>();

    react(&mut polymer)
}

fn two(input: &str) -> usize {
    let mut min = input.len();
    let chars = input
        .chars()
        .map(|letter| letter.to_ascii_lowercase())
        .collect::<HashSet<_>>();

    for unit in chars.into_iter() {
        let mut polymer = input
            .chars()
            .filter(|letter| !letter.eq_ignore_ascii_case(&unit))
            .collect::<Vec<_>>();

        let count = react(&mut polymer);
        if count < min {
            min = count;
        }
    }

    min
}

fn react(polymer: &mut Vec<char>) -> usize {
    loop {
        let mut reacted = false;
        let len = polymer.len();
        for idx in 0..len {
            let (mut start, mut end) = (idx, idx + 1);

            loop {
                let (left, right) = polymer.split_at_mut(end);
                let (letter, next) = match (left.get_mut(start), right.first_mut()) {
                    (Some(letter), Some(next)) => (letter, next),
                    _ => break,
                };

                if *letter == '.' {
                    break;
                }

                if *next == '.' && end < len {
                    end += 1;
                    continue;
                }

                if !letter.eq_ignore_ascii_case(&next) || letter == next {
                    break;
                }

                reacted = true;
                *letter = '.';
                *next = '.';

                if start > 0 && end < len {
                    start -= 1;
                    end += 1;
                    continue;
                }

                break;
            }
        }

        if !reacted {
            break;
        }

        polymer.retain(|&letter| letter != '.');
    }

    polymer
        .into_iter()
        .filter(|&&mut letter| letter != '.')
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT_1: &'static str = "dabAcCaCBAcCcaDA";
    static TEST_INPUT_2: &'static str = "aaAbdDDdeEbbbBcAaC";
    static TEST_INPUT_3: &'static str =
        "QqGgPfHhOiIoFMmUXxOooBbOqXxcCbBQboOBusSTTtjJATtafFkKZzxEeeE";

    #[test]
    fn part_one() {
        assert_eq!(10, one(TEST_INPUT_1));
        assert_eq!(4, one(TEST_INPUT_2));
        assert_eq!(3, one(TEST_INPUT_3));
        assert_eq!(10368, one(INPUT));
    }

    #[test]
    fn part_two() {
        assert_eq!(4, two(TEST_INPUT_1));
        assert_eq!(1, two(TEST_INPUT_2));
        assert_eq!(2, two(TEST_INPUT_3));
        assert_eq!(4122, two(INPUT));
    }
}
