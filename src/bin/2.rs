use std::collections::HashMap;

static INPUT: &'static str = include_str!("input/2.txt");

fn main() {
    println!("1. {}", one());
    println!("2. {}", two());
}

fn one() -> u32 {
    let (mut twos, mut threes) = (0, 0);

    let mut box_ids = INPUT.split_whitespace().peekable();
    let mut frequencies: HashMap<char, u32> = HashMap::with_capacity(box_ids.peek().unwrap().len());

    for box_id in box_ids {
        box_id.chars().for_each(|letter| {
            let count = frequencies.get(&letter).map_or(1, |count| count + 1);
            frequencies.insert(letter, count);
        });

        // casting boolean to integer. not pretty :/
        twos += frequencies.values().any(|&count| count == 2) as u32;
        threes += frequencies.values().any(|&count| count == 3) as u32;

        frequencies.clear();
    }

    twos * threes
}

fn two() -> String {
    let box_ids: Vec<_> = INPUT
        .split_whitespace()
        .collect();

    let mut common = Vec::with_capacity(box_ids.first().unwrap().len());

    for box_id in box_ids.iter() {
        for other_box_id in box_ids.iter() {
            let mut different = 0;
            for (c1, c2) in box_id.chars().zip(other_box_id.chars()) {
                if different > 1 {
                    break;
                }

                if c1 == c2 {
                    common.push(c1);
                } else {
                    different += 1;
                }
            }

            if different == 1 {
                return common.iter().cloned().collect();
            }

            common.clear();
        }
    }

    // a proper solution would be to return Option::None instead of an empty string
    String::new()
}
