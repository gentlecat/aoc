use colored::Colorize;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    let schematic = read_to_string("../input.txt").unwrap();

    let str_lines: Vec<&str> = schematic.lines().collect::<Vec<_>>();
    let char_lines: Vec<Vec<char>> = schematic
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let gear_locations = find_gears(&char_lines);
    let mut gear_to_number: HashMap<(i64, i64), Vec<u64>> = HashMap::new();
    let mut part_number_locations: HashSet<(i64, i64)> = HashSet::new();

    for y in 0..=(str_lines.len() - 1) {
        let matches = find_numbers(str_lines[y]);

        for m in &matches {
            let matching_gear_locations = check_number(*m, y.try_into().unwrap(), &gear_locations);
            for loc in matching_gear_locations {
                gear_to_number
                    .entry(loc)
                    .or_insert(vec![])
                    .push(m.as_str().parse::<u64>().unwrap())
            }

            if !matches.is_empty() {
                // Add to the index for printing
                for x in m.start()..=(m.end() - 1) {
                    part_number_locations.insert((x.try_into().unwrap(), y.try_into().unwrap()));
                }
            }
        }
    }
    pretty_print_schematic(&char_lines, &gear_locations, &part_number_locations);

    let mut sum: u64 = 0;
    for v in gear_to_number.into_values() {
        if v.len() == 2 {
            sum += v.get(0).unwrap() * v.get(1).unwrap()
        }
    }

    println!("\nResult: {}", sum);
}

fn pretty_print_schematic(
    char_lines: &Vec<Vec<char>>,
    symbol_locations: &HashSet<(i64, i64)>,
    part_number_locations: &HashSet<(i64, i64)>,
) {
    for y in 0..=(char_lines.len() - 1) {
        for x in 0..=(char_lines[0].len() - 1) {
            let is_symbol =
                symbol_locations.contains(&(x.try_into().unwrap(), y.try_into().unwrap()));
            let is_part_number =
                part_number_locations.contains(&(x.try_into().unwrap(), y.try_into().unwrap()));
            if is_symbol && is_part_number {
                print!("{}", String::from(char_lines[y][x]).blue());
            } else if is_symbol {
                print!("{}", String::from(char_lines[y][x]).red());
            } else if is_part_number {
                print!("{}", String::from(char_lines[y][x]).green());
            } else {
                print!("{}", char_lines[y][x]);
            }
        }
        print!("\n");
    }
}

// Finds coordinates of all gears in a schematic
fn find_gears(schematic_lines: &Vec<Vec<char>>) -> HashSet<(i64, i64)> {
    let mut gear_locations: HashSet<(i64, i64)> = HashSet::new();
    for y in 0..=(schematic_lines.len() - 1) {
        for x in 0..=(schematic_lines[0].len() - 1) {
            if schematic_lines[y][x] == '*' {
                // Converting to signed value for ease of use afterwards
                gear_locations.insert((x.try_into().unwrap(), y.try_into().unwrap()));
            }
        }
    }
    return gear_locations;
}

// Finds all numbers in a row
fn find_numbers(s: &str) -> Vec<regex::Match<'_>> {
    return Regex::new(r"(?<number>\d+)")
        .unwrap()
        .captures_iter(&s)
        .map(|y| y.get(0).unwrap())
        .collect::<Vec<_>>();
}

// Number is part of the match
fn check_number(
    n: regex::Match<'_>,
    y: i64,
    symbol_locations: &HashSet<(i64, i64)>,
) -> Vec<(i64, i64)> {
    let mut matching_locations: Vec<(i64, i64)> = vec![];

    let start: i64 = n.start().try_into().unwrap();
    let end: i64 = n.end().try_into().unwrap();

    // Check left
    if symbol_locations.contains(&(start - 1, y)) {
        matching_locations.push((start - 1, y));
    }

    // Check right
    if symbol_locations.contains(&(end, y)) {
        matching_locations.push((end, y));
    }

    // Check above and below including diagonals
    for x in (start - 1)..=(end) {
        if symbol_locations.contains(&(x, y - 1)) {
            matching_locations.push((x, y - 1));
        }
        if symbol_locations.contains(&(x, y + 1)) {
            matching_locations.push((x, y + 1));
        }
    }

    return matching_locations;
}
