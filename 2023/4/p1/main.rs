use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::process;

fn main() {
    let result: u64 = read_to_string("../input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .map(process)
        .sum();

    println!("Result: {}", result);
}

fn process(s: String) -> u64 {
    let cards_regex =
        Regex::new(r"^Card +(?<card_number>\d+): (?<winning>[ +\d+]+) \| (?<have>[ +\d+]+)$")
            .unwrap();

    let Some(cards_info) = cards_regex.captures(&s) else {
        println!("BAD REGEX!");
        process::exit(0x0100);
    };

    let card_number: &u64 = &cards_info["card_number"].parse::<u64>().unwrap();

    let winning: HashSet<u64> = cards_info["winning"]
        .split(" ")
        .map(str::trim)
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let have: Vec<u64> = cards_info["have"]
        .split(" ")
        .map(str::trim)
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut count: u64 = 0;

    for h in &have {
        if winning.contains(&h) {
            if count != 0 {
                count *= 2;
            } else {
                count = 1;
            };
        }
    }

    println!("{}\n{:?} | {:?} \nScore: {}\n", s, &winning, &have, count);

    return count;
}
