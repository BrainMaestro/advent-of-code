static INPUT: &'static str = include_str!("input/1.txt");

fn main() {
    println!("1. {}", one());
    println!("2. {}", two());
}

fn one() -> i32 {
    INPUT
        .split_whitespace()
        .fold(0, |acc, freq: &str| acc + freq.parse::<i32>().unwrap())
}

fn two() -> i32 {
    use std::collections::HashSet;

    let mut frequencies = HashSet::new();
    let mut current = 0;
    loop {
        for freq in INPUT.split_whitespace() {
            if frequencies.contains(&current) {
                return current;
            }
            frequencies.insert(current);

            let freq: i32 = freq.parse().unwrap();
            current += freq;
        }
    }
}
