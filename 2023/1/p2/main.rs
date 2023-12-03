use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let result: u64 = read_to_string("../input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .map(get_numbers_from_string)
        .filter(|x| x.is_some())
        .map(Option::unwrap)
        .sum();

    println!("Result: {}", result);
}

fn get_numbers_from_string(s: String) -> Option<u64> {
    println!("Row: {}", &s);

    let parts = split(s);
    println!("  Parts: {:?}", &parts);

    let mut first: Option<String> = None;
    let mut last: Option<String> = None;

    for p in parts {
        if p.parse::<u64>().is_ok() {
            if first.is_none() {
                first = Some(p)
            } else {
                last = Some(p)
            }
        } else {
            let numsOpt = find_numbers(p);
            if numsOpt.is_none() {
                continue;
            }
            let nums = numsOpt.unwrap();
            if first.is_none() {
                // We care about first and last one here
                first = Some(nums.0.to_string());
                last = Some(nums.1.to_string());
            } else {
                // We care about the last one only
                last = Some(nums.1.to_string());
            }
        }
    }

    if last.is_none() {
        last = first.clone()
    }

    let first_val = first.unwrap_or(String::new());
    let last_val = last.unwrap_or(String::new());

    println!("  Number: {}{}\n", first_val, last_val);
    return Some(format!("{}{}", first_val, last_val).parse::<u64>().unwrap());
}

fn split(s: String) -> Vec<String> {
    let mut parts = Vec::<String>::new();

    // Split string into characters and digits
    let mut current_substring = String::new();

    for c in s.chars() {
        if c.is_digit(10) {
            if current_substring.len() > 0 {
                parts.push(current_substring);
                current_substring = String::new();
            }
            parts.push(String::from(c));
        } else {
            current_substring.push(c);
        }
    }

    if current_substring.len() > 0 {
        parts.push(current_substring);
    }

    return parts;
}

// Finds first and last digit in a given string if there are any.
fn find_numbers(s: String) -> Option<(u64, u64)> {
    let digits: HashMap<&str, u64> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    // Map with sorted keys
    let mut locations_to_val = BTreeMap::<u64, u64>::new();

    for (digit_string, val) in digits.iter() {
        let matches: Vec<_> = s.match_indices(digit_string).collect();

        for (idx, _) in matches {
            locations_to_val.insert(idx.try_into().unwrap(), *val);
        }
    }

    println!("  Matches: {:?}", locations_to_val);

    if locations_to_val.len() < 1 {
        // No digits digit substrings found.
        return None;
    }

    return Some((
        *locations_to_val.first_key_value().unwrap().1,
        *locations_to_val.last_key_value().unwrap().1,
    ));
}
