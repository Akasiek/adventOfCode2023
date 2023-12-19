use std::collections::HashMap;
use crate::read_file;

pub fn day1_1() -> u32 {
    let mut sum: u32 = 0;
    for line in read_file("day1.txt").lines() {
        let (last, rest) = get_first_number_in_str(line.to_string(), true);
        let first = get_first_number_in_str(rest, false).0;
        sum += last + (first * 10);
    }

    sum
}

pub fn day1_2() -> u32 {
    let mut sum: u32 = 0;
    let mut i = 1;
    for line in read_file("day1.txt").lines() {
        let line_pattern = filter_out_numbers(line.to_string());

        let first = line_pattern.chars().next().unwrap().to_digit(10).unwrap();
        let last = line_pattern.chars().last().unwrap().to_digit(10).unwrap();
        sum += last + (first * 10);

        i += 1;
    }

    sum
}

fn get_first_number_in_str(s: String, from_the_back: bool) -> (u32, String) {
    let mut chars = s.chars();
    let mut c = match from_the_back {
        true => chars.next_back().unwrap(),
        false => chars.next().unwrap(),
    };

    loop {
        if c.is_digit(10) {
            return (c.to_digit(10).unwrap(), s);
        }

        c = match from_the_back {
            true => chars.next_back().unwrap(),
            false => chars.next().unwrap(),
        };
    }
}

fn filter_out_numbers(s: String) -> String {
    let mut chars = s.chars();
    let mut result = String::new();
    let mut i: u32 = 0;

    for c in chars {
        if c.is_digit(10) {
            result.push(c);
        } else if c.is_alphabetic() {
            let spelled_out_number = find_spelled_out_number(c, i as usize, &s);
            if let Some(number) = spelled_out_number {
                result.push_str(&number.to_string());
            }
        }
        i += 1;
    }


    result
}

fn find_spelled_out_number(c: char, c_index: usize, s: &String) -> Option<u32> {
    let spelled_out_numbers = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);

    for (key, value) in spelled_out_numbers {
        if (c == key.chars().next().unwrap()) {
            let fits = s.get(c_index..c_index + key.len()).is_some();
            if (fits && s.get(c_index..c_index + key.len()).unwrap() == key) {
                return Some(value);
            }
        }
    }

    None
}

