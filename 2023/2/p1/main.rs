use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let result: u64 = read_to_string("../input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .map(process)
        .filter(|x| x.is_some())
        .map(Option::unwrap)
        .sum();

    println!("Result: {}", result);
}

fn process(s: String) -> Option<u64> {
    print!("{}", &s);

    let game_info_regex = Regex::new(r"^Game (?<game_number>\d+): (?<reveals>.+)$").unwrap();

    let Some(game_info) = game_info_regex.captures(&s) else {
        return None;
    };

    let game_number: &u64 = &game_info["game_number"].parse::<u64>().unwrap();
    let reveals: Vec<&str> = game_info["reveals"].split(";").map(str::trim).collect();
    let valid = is_game_valid(reveals.clone());

    println!("Game: {:#?}", game_number);
    println!("Reveals: {:#?}", reveals);
    println!("Valid?: {:#?}\n", valid);

    if !valid {
        return None;
    }

    return Some(*game_number);
}

fn is_game_valid(reveals: Vec<&str>) -> bool {
    for reveal in reveals {
        let cubes: Vec<&str> = reveal.split(",").map(str::trim).collect();
        if !is_reveal_valid(cubes) {
            return false;
        }
    }

    return true;
}

fn is_reveal_valid(cubes: Vec<&str>) -> bool {
    let limits: HashMap<&str, u64> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    for cube in cubes {
        let cube_data: Vec<&str> = cube.split(" ").map(str::trim).collect();

        let count = cube_data[0].parse::<u64>().unwrap();
        let color = cube_data[1];

        if count > *limits.get(color).unwrap() {
            return false;
        }
    }

    return true;
}
