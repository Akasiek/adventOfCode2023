use std::fmt::format;
use regex::Regex;
use crate::read_file;

struct Game {
    id: u32,
    // (red, green, blue)
    sets: Vec<(u32, u32, u32)>,
    power: u32,
    is_possible: bool,
}

impl Game {
    fn new(id: u32) -> Game {
        Game {
            id,
            sets: Vec::new(),
            power: 0,
            is_possible: true,
        }
    }

    fn add_set(&mut self, set: (u32, u32, u32)) {
        self.sets.push(set);
    }

    /// Is possible when:
    /// Red <= 12
    /// Green <= 13
    /// Blue <= 14
    fn check_if_possible(&mut self) {
        for set in &self.sets {
            if set.0 > 12 || set.1 > 13 || set.2 > 14 {
                self.is_possible = false;
            }
        }
    }

    fn set_power(&mut self) {
        let mut max_values: (u32, u32, u32) = (0, 0, 0);
        for set in &self.sets {
            if set.0 > max_values.0 {
                max_values.0 = set.0;
            }
            if set.1 > max_values.1 {
                max_values.1 = set.1;
            }
            if set.2 > max_values.2 {
                max_values.2 = set.2;
            }
        }

        self.power = (max_values.0 * max_values.1 * max_values.2);
    }
}

pub fn day2_2() -> u32 {
    let mut sum: u32 = 0;
    for line in read_file("day2.txt").lines() {
        let mut game = read_game_from_str(line.to_string());
        game.set_power();

        sum += game.power;
    }

    sum
}

pub fn day2_1() -> u32 {
    let mut sum = 0;

    for line in read_file("day2.txt").lines() {
        let game = read_game_from_str(line.to_string());

        if (game.is_possible) {
            sum += game.id;
        }
    }

    sum
}

/// Games look like this:
/// Game 1: 2 blue, 3 red; 3 green, 3 blue, 6 red; 4 blue, 6 red; 2 green, 2 blue, 9 red; 2 red, 4 blue
///
/// First is the game id
/// Then each set is separated by a semicolon
fn read_game_from_str(s: String) -> Game {
    // Remove the "Game {id}" part with regex
    let re = Regex::new(r"Game (\d+): ").unwrap();
    let caps = re.captures(&s).unwrap();
    let id = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();

    // Remove the "Game {id}" part with regex
    let s = re.replace(&s, "");

    let sets: Vec<&str> = s.split(';').map(|s| s.trim()).collect();

    let mut game = Game::new(id);
    for set in sets {
        game.add_set(read_set_from_str(set.to_string()));
    }
    game.check_if_possible();

    game
}

fn read_set_from_str(s: String) -> (u32, u32, u32) {
    let cubes = s.split(',').map(|s| s.trim()).collect::<Vec<&str>>();

    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for cube in cubes {
        if let Some((number, color)) = cube.split_once(' ') {
            let number = number.parse::<u32>().unwrap();
            match (color) {
                "red" => red = number,
                "green" => green = number,
                "blue" => blue = number,
                _ => panic!("Wrong color!"),
            }
        } else {
            panic!("Wrong format!");
        }
    }

    (red, green, blue)
}