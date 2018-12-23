use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

static INPUT: &'static str = include_str!("input/3.txt");
lazy_static! {
    static ref CLAIMS_REGEX: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
}

type POINTS = HashMap<(u16, u16), u32>;

fn main() {
    let mut points = HashMap::new();

    println!("1. {}", one(&mut points));
    println!("2. {}", two(&mut points));
}

fn one(points: &mut POINTS) -> usize {
    claims(|_id, point| {
        let count: u32 = points.get(&point).map_or(1, |&count| count + 1);
        points.insert(point, count);
        Outcome::Continue
    });

    points.values().filter(|&&count| count > 1).count()
}

fn two(points: &mut POINTS) -> u16 {
    let (mut current_id, mut overlapped) = (1, false);
    claims(|id, point| {
        if current_id != id {
            if !overlapped {
                return Outcome::Stop;
            }

            current_id = id;
            overlapped = false;
        }

        overlapped = points.get(&point).filter(|&&count| count == 1).is_none();
        if overlapped {
            return Outcome::Next;
        }

        Outcome::Continue
    });

    current_id
}

fn claims(mut operation: impl FnMut(u16, (u16, u16)) -> Outcome) {
    for cap in INPUT
        .lines()
        .filter_map(|claim| CLAIMS_REGEX.captures(claim))
    {
        let (id, left, top, width, height): (u16, u16, u16, u16, u16) = (
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[3].parse().unwrap(),
            cap[4].parse().unwrap(),
            cap[5].parse().unwrap(),
        );

        'claim: for x in left..left + width {
            for y in top..top + height {
                match operation(id, (x, y)) {
                    Outcome::Continue => {}
                    Outcome::Next => break 'claim,
                    Outcome::Stop => return,
                }
            }
        }
    }
}

enum Outcome {
    Continue,
    Next,
    Stop,
}
