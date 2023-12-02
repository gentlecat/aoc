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

    println!("\nResult: {}", result);
}

fn get_numbers_from_string(s: String) -> Option<u64> {
    print!("{}", &s);

    let mut first: Option<char> = None;
    let mut last: Option<char> = None;

    for c in s.chars() {
        if c.is_digit(10) {
            if first.is_none() {
                first = Some(c)
            } else {
                last = Some(c)
            }
        }
    }

    if last.is_none() {
        last = first
    }

    println!(" - {}{}", first.unwrap(), last.unwrap());

    return Some(
        format!("{}{}", first.unwrap(), last.unwrap())
            .parse::<u64>()
            .unwrap(),
    );
}
