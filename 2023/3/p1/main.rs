use colored::Colorize;
use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let schematic = read_to_string("../input.txt").unwrap();

    let str_lines: Vec<&str> = schematic.lines().collect::<Vec<_>>();
    let char_lines: Vec<Vec<char>> = schematic
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let symbol_locations = find_symbols(&char_lines);
    let mut part_number_locations: HashSet<(i64, i64)> = HashSet::new();
    println!("Symbol locations: {:?}", symbol_locations);

    let mut sum: u64 = 0;

    for y in 0..=(str_lines.len() - 1) {
        // Logging rows for debugging
        if y > 0 {
            println!("\n     {}", str_lines[y - 1]);
        }
        println!("Row: {}", str_lines[y]);
        if y < (str_lines.len() - 1) {
            println!("     {}", str_lines[y + 1]);
        }

        let matches = find_numbers(str_lines[y]);
        println!("Captures: {:?}", matches);

        let mut row_sum: u64 = 0;
        for m in &matches {
            if check_number(*m, y.try_into().unwrap(), &symbol_locations) {
                row_sum += m.as_str().parse::<u64>().unwrap();

                // Add to the index for printing
                for x in m.start()..=(m.end()-1){
                    part_number_locations.insert((x.try_into().unwrap(), y.try_into().unwrap()));
                }
            }
        }
        println!("Row sum: {}", row_sum);
        sum += row_sum;
    }

    println!("");
    pretty_print_schematic(&char_lines, &symbol_locations, &part_number_locations);

    println!("\nResult: {}", sum);
}

fn pretty_print_schematic(char_lines: &Vec<Vec<char>>, symbol_locations: &HashSet<(i64, i64)>, part_number_locations: &HashSet<(i64, i64)>) {
    for y in 0..=(char_lines.len() - 1) {
        for x in 0..=(char_lines[0].len() - 1) {
            let is_symbol =  symbol_locations.contains(&(x.try_into().unwrap(), y.try_into().unwrap()));
            let is_part_number =  part_number_locations.contains(&(x.try_into().unwrap(), y.try_into().unwrap()));
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

// Finds coordinates of all symbols in a schematic
fn find_symbols(schematic_lines: &Vec<Vec<char>>) -> HashSet<(i64, i64)> {
    let symbols = HashSet::from(['*', '@', '/', '$', '=', '#', '-', '+', '&', '%']);
    let mut symbol_locations: HashSet<(i64, i64)> = HashSet::new();
    for y in 0..=(schematic_lines.len() - 1) {
        for x in 0..=(schematic_lines[0].len() - 1) {
            if symbols.contains(&schematic_lines[y][x]) {
                // Converting to signed value for ease of use afterwards
                symbol_locations.insert((x.try_into().unwrap(), y.try_into().unwrap()));
            }
        }
    }
    return symbol_locations;
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
fn check_number(n: regex::Match<'_>, y: i64, symbol_locations: &HashSet<(i64, i64)>) -> bool {
    let start: i64 = n.start().try_into().unwrap();
    let end: i64 = n.end().try_into().unwrap();

    // Check left
    if symbol_locations.contains(&(start - 1, y)) {
        return true;
    }

    // Check right
    if symbol_locations.contains(&(end, y)) {
        return true;
    }

    // Check above and below including diagonals
    for x in (start - 1)..=(end) {
        if symbol_locations.contains(&(x, y - 1)) {
            return true;
        }
        if symbol_locations.contains(&(x, y + 1)) {
            return true;
        }
    }

    return false;
}
