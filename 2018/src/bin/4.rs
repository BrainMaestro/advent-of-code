use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

static INPUT: &'static str = include_str!("input/4.txt");
lazy_static! {
    static ref RECORD_REGEX: Regex = Regex::new(r".+:(\d\d)\] (.+)").unwrap();
}

type GUARDS = HashMap<u16, [u32; 60]>;

fn main() {
    let guards = parse(INPUT).unwrap();
    println!("1. {}", one(&guards));
    println!("2. {}", two(&guards));
}

fn parse(input: &str) -> Option<GUARDS> {
    let mut guards: GUARDS = HashMap::new();
    let mut current_guard: u16 = 0;
    let mut sleep_start: u8 = 0;
    let input = {
        let mut input = input.lines().collect::<Vec<_>>();
        input.sort();
        input.into_iter()
    };

    for cap in input.filter_map(|record| RECORD_REGEX.captures(record)) {
        let (minute, action) = (cap[1].parse::<u8>().ok()?, &cap[2]);

        match action.get(..1)? {
            "G" => {
                let mut parts = action.split_whitespace();
                parts.next(); // discard first entry 'Guard'
                current_guard = parts.next()?.get(1..)?.parse::<u16>().ok()?;
            }
            "f" => {
                sleep_start = minute;
            }
            "w" => {
                let mut sleep_time = match guards.get_mut(&current_guard) {
                    Some(sleep_time) => *sleep_time,
                    None => [0; 60],
                };

                for min in sleep_start..minute {
                    sleep_time[min as usize] += 1;
                }

                guards.insert(current_guard, sleep_time);
            }
            _ => unreachable!(),
        }
    }

    Some(guards)
}

fn one(guards: &GUARDS) -> u32 {
    guards
        .iter()
        .map(|(id, sleep_time)| {
            let (slot, _, total) =
                sleep_time
                    .iter()
                    .enumerate()
                    .fold((0, &0, 0), |max, current| {
                        let total = max.2 + current.1;
                        if current.1 > max.1 {
                            (current.0, current.1, total)
                        } else {
                            (max.0, max.1, total)
                        }
                    });

            (id, slot, total)
        })
        .max_by(|guard, other_guard| guard.2.cmp(&other_guard.2))
        .map_or(0, |(&id, max, _)| u32::from(id) * max as u32)
}

fn two(guards: &GUARDS) -> u32 {
    guards
        .iter()
        .map(|(id, sleep_time)| {
            let (slot, _) = sleep_time.iter().enumerate().fold((0, &0), |max, current| {
                if current.1 > max.1 {
                    current
                } else {
                    max
                }
            });

            (id, slot)
        })
        .max_by(|guard, other_guard| guard.1.cmp(&other_guard.1))
        .map_or(0, |(&id, max)| u32::from(id) * max as u32)
}

#[cfg(test)]
mod test {
    use super::*;

    static BASIC_INPUT: &'static str = "
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    #[test]
    fn parses_correctly() {
        let guards = parse(BASIC_INPUT).unwrap();

        assert!(guards.get(&10).is_some());
        assert!(guards.get(&99).is_some());
    }

    #[test]
    fn part_one() {
        let guards = parse(BASIC_INPUT).unwrap();
        assert_eq!(240, one(&guards));

        let guards = parse(INPUT).unwrap();
        assert_eq!(131469, one(&guards));
    }

    #[test]
    fn part_two() {
        let guards = parse(BASIC_INPUT).unwrap();
        assert_eq!(4455, two(&guards));

        let guards = parse(INPUT).unwrap();
        assert_eq!(96951, two(&guards));
    }
}
