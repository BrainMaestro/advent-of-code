use std::collections::HashMap;

static INPUT: &'static str = include_str!("input/2.txt");
//static INPUT: &'static str = "abcdef bababc abbcde abcccd aabcdd abcdee ababab";

fn main() {
    //    println!("1. {}", one());
    assert_eq!(5904, one());
    //    println!("2. {}", two());
}

fn one() -> u32 {
    let mut two_letters = 0;
    let mut three_letters = 0;

    let mut box_ids = INPUT.split_whitespace().peekable();
    let mut map: HashMap<char, u32> = HashMap::with_capacity(box_ids.peek().unwrap().len());

    for box_id in box_ids {
        box_id.chars().for_each(|letter| {
            let count = map.get(&letter).map_or(1, |count| count + 1);
            map.insert(letter, count);
        });

        let mut has_two = false;
        let mut has_three = false;
        for count in map.values() {
            if *count == 2 && !has_two {
                has_two = true;
                two_letters += 1;
            } else if *count == 3 && !has_three {
                has_three = true;
                three_letters += 1;
            }
        }

        map.clear();
    }

    two_letters * three_letters
}

fn two() -> u32 {
    use std::iter::ExactSizeIterator;
    let mut box_ids = INPUT.split_whitespace();
    let box_id_chars = Vec::with_capacity(box_ids.len());

    0
}
