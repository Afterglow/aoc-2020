use hex;
use std::collections::HashMap;

const REQUIRED_FIELDS: [&str; 7] = [
  "byr",
  "iyr",
  "eyr",
  "hgt",
  "hcl",
  "ecl",
  "pid"
];

fn validate_passport(p: HashMap<&str, &str>) -> bool {
    for key in REQUIRED_FIELDS.iter() {
      if !p.contains_key(key) {
          println!("Passport {:?} missing {}", p, key);
          return false;
      }
    }
    if byr_valid(p["byr"]) && iyr_valid(p["iyr"]) && eyr_valid(p["eyr"])
       && hgt_valid(p["hgt"]) && hcl_valid(p["hcl"]) && ecl_valid(p["ecl"])
       && pid_valid(p["pid"]) {
           println!("Valid passport");
           return true;
    }
    return false;
}

fn byr_valid(input: &str) -> bool {
    let byr: u16 = input.parse().unwrap();
    let result: bool = byr >=1920 && byr <= 2002;
    if !result {
        println!("byr {} invalid", input);
    }
    return result;
}

fn iyr_valid(input: &str) -> bool {
    let iyr: u16 = input.parse().unwrap();
    let result: bool = iyr >= 2010 && iyr <= 2020;
    if !result {
        println!("iyr {} invalid", input);
    }
    return result;
}

fn eyr_valid(input: &str) -> bool {
    let eyr: u16 = input.parse().unwrap();
    let result: bool = eyr >= 2020 && eyr <= 2030;
    if !result {
        println!("eyr {} invalid", input);
    }
    return result;
}

fn hgt_valid(input: &str) -> bool {
    let result: bool = if input.ends_with("cm") {
        let hgt: u16 = match input[0..3].parse() {
            Ok(hgt) => hgt,
            Err(_e) => return false
        };
        println!("Checking hgt in cm of {}", hgt);
        hgt >= 150 && hgt <= 193
    } else if input.ends_with("in") {
        let hgt: u16 = match input[0..2].parse() {
            Ok(hgt) => hgt,
            Err(_e) => return false
        };
        hgt >= 59 && hgt <= 76
    } else {
        false
    };
    if !result {
        println!("hgt {} invalid", input);
    }
    return result;
}

fn hcl_valid(input: &str) -> bool {
    if input.chars().nth(0) == Some('#') {
        match hex::decode(input.split_at(1).1) {
            Ok(_n) => return true,
            Err(e) => {
                println!("hcl {} invalid: {:?}", input, e);
                return false
            }
        };
    }
    println!("hcl {} invalid", input);
    return false;
}

fn ecl_valid(input: &str) -> bool {
    let valid_colours = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    for c in valid_colours.iter() {
        if &input == c {
            return true;
        }
    }
    println!("ecl {} invalid", input);
    return false;
}

fn pid_valid(input: &str) -> bool {
    if input.len() != 9 {
        println!("pid {} wrong length", input);
        return false;
    }
    match input.parse::<u32>() {
        Ok(_n) => return true,
        Err(_e) => {
            println!("pid {} not a number", input);
            return false
        }
    };
}

fn main() {
    let mut total_passports = 0;
    let mut valid_passports = 0;
    let file = std::fs::read_to_string("input.txt").unwrap();
    let mut passport: HashMap<&str, &str> = HashMap::new();

    for line in file.lines() {
        if line == "" {
            println!("---------------------------------");
            println!("Validating {:?}", passport);
            if validate_passport(passport) {
                valid_passports += 1;
            }
            total_passports += 1;
            passport = HashMap::new();
            continue;
        }
        let parts = line.split_whitespace();
        for part in parts {
            let fields: Vec<&str> = part.split(":").collect();
            passport.insert(fields[0], fields[1]);
        }
    }

    // Deal with the last passport that didn't have a blank line after it
    if validate_passport(passport) {
        valid_passports += 1;
    }
    total_passports += 1;

    println!("Found {} valid passports out of {} total", valid_passports, total_passports);
}
