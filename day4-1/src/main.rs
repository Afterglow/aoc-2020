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

fn validate_passport(passport: HashMap<&str, &str>) -> bool {
    for key in REQUIRED_FIELDS.iter() {
      if !passport.contains_key(key) {
          println!("Passport {:?} missing {}", passport, key);
          return false;
      }
    }
    return true;
}

fn main() {
    let mut total_passports = 0;
    let mut valid_passports = 0;
    let file = std::fs::read_to_string("input.txt").unwrap();
    let mut passport: HashMap<&str, &str> = HashMap::new();

    for line in file.lines() {
        if line == "" {
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
