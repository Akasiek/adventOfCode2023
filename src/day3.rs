use crate::read_file;

#[derive(Debug, Clone)]
struct Number {
    value: i32,
    coords: Vec<(i32, i32)>,
    is_part_of_code: bool,
}

impl Number {
    fn new(value: i32, coord: (i32, i32)) -> Number {
        let coords = vec!(coord);

        Number {
            value,
            coords,
            is_part_of_code: false,
        }
    }

    fn check_if_beside_and_add_number(&mut self, new_value: i32, new_coord: (i32, i32)) -> bool {
        let last_coord = self.coords.last().unwrap();

        if check_if_coords_are_beside(last_coord, &new_coord) {
            self.value = (self.value * 10) + new_value;
            self.coords.push(new_coord);

            return true;
        }

        false
    }

    /// Check all possibilities
    /// ***
    /// *0*
    /// ***
    fn check_if_has_symbol_around(&self, symbols: &Vec<Symbol>) -> bool {
        for coord in &self.coords {
            for symbol in symbols {
                if check_if_coords_around(&symbol.coord, coord) {
                    return true;
                }
            }
        }

        false
    }

    fn set_is_part_of_code(&mut self, symbols: &Vec<Symbol>) {
        self.is_part_of_code = self.check_if_has_symbol_around(symbols)
    }
}


struct Symbol {
    coord: (i32, i32),
    char: char,
}

impl Symbol {
    fn new(coord: (i32, i32), char: char) -> Symbol {
        Symbol {
            coord,
            char,
        }
    }

    fn check_if_gear(&self, nums: Vec<Number>) -> Option<Vec<Number>> {
        if self.char != '*' {
            return None;
        }

        let mut found: Vec<Number> = Vec::new();

        for num in nums {
            if check_if_few_coords_around_one(&num.coords, &self.coord) {
                found.push(num);
            }
        }

        if found.len() == 2 {
            Some(found)
        } else {
            None
        }
    }

    fn calculate_gear_ratio(&self, nums: Vec<Number>) -> Option<i32> {
        if let Some(founds) = self.check_if_gear(nums) {
            let mut res = 1;
            for found in founds {
                res *= found.value;
            }

            Some(res)
        } else {
            None
        }
    }
}

fn check_if_few_coords_around_one(coords: &Vec<(i32, i32)>, target: &(i32, i32)) -> bool {
    for coord in coords {
        if check_if_coords_around(coord, target) {
            return true;
        }
    }

    false
}

fn check_if_coords_around(one: &(i32, i32), two: &(i32, i32)) -> bool {
    (one.0 - two.0).abs() < 2 && (one.1 - two.1).abs() < 2
}

fn check_if_coords_are_beside(one: &(i32, i32), two: &(i32, i32)) -> bool {
    (one.0 - two.0).abs() == 1 && one.1 == two.1
}

fn get_numbers_and_symbols() -> (Vec<Number>, Vec<Symbol>) {
    let input = read_file("day3.txt");
    let mut nums: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let new_coords = (x as i32, y as i32);

            if char == '.' {
                continue;
            } else if char.is_ascii_digit() {
                let new_value = char.to_digit(10).unwrap() as i32;
                if let Some(last_num) = nums.last_mut() {
                    if !last_num.check_if_beside_and_add_number(new_value, new_coords) {
                        nums.push(Number::new(new_value, new_coords));
                    }
                } else {
                    nums.push(Number::new(new_value, new_coords));
                }

                continue;
            } else {
                symbols.push(Symbol::new(new_coords, char));
            }
        }
    }

    (nums, symbols)
}

pub fn day3_1() -> i32 {
    let mut sum = 0;
    let (nums, symbols) = get_numbers_and_symbols();

    for mut n in nums {
        n.set_is_part_of_code(&symbols);

        if n.is_part_of_code {
            sum += n.value;
        }
    }

    sum
}

pub fn day3_2() -> i32 {
    let mut sum = 0;
    let (nums, symbols) = get_numbers_and_symbols();

    for sym in symbols {
        if let Some(ratio) = sym.calculate_gear_ratio(nums.clone()) {
            sum += ratio;
        }
    }

    sum
}