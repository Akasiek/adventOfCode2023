use std::fs::File;
use std::io::Read;
mod day1;
mod day2;
mod day3;

fn read_file(file_name: &str) -> String {
    let mut file = File::open(file_name).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");
    contents
}

fn main() {
    // println!("Day 1.1: {}", day1::day1_1());
    // println!("Day 1.2: {}", day1::day1_2());

    // println!("Day 2.1: {}", day2::day2_1());
    // println!("Day 2.2: {}", day2::day2_2());

    println!("Day 3.1: {}", day3::day3_1());
    println!("Day 3.2: {}", day3::day3_2());
}
