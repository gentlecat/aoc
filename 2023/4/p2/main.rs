use regex::Regex;
use std::collections::{HashSet, VecDeque, BTreeMap};
use std::fs::read_to_string;
use std::process;

#[derive(Clone)]
struct Card {
    number: u64,
    winning: HashSet<u64>,
    have: HashSet<u64>,
}

fn main() {
    let cards: BTreeMap<u64, Card> = read_to_string("../input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .map(process)
        .collect();

    let mut remaining_cards: VecDeque<Card> = VecDeque::from_iter(cards.clone().into_values());

    let mut total_card_count: u64 = 0;

    while let Some(card) = remaining_cards.pop_back() {
        total_card_count += 1;
        let matches = count_matches(&card);
        for n in (&card.number + 1)..(&card.number + &matches + 1) {
            let extra_card: Card = cards.get(&n).unwrap().clone();
            remaining_cards.push_front(extra_card);
        }
    }

    println!("Result: {}", total_card_count);
}

fn process(s: String) -> (u64, Card) {
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

    let have: HashSet<u64> = cards_info["have"]
        .split(" ")
        .map(str::trim)
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    return (
        *card_number,
        Card {
            number: *card_number,
            winning,
            have,
        },
    );
}

fn count_matches(c: &Card) -> u64 {
    return c.winning.intersection(&c.have).count() as u64;
}
