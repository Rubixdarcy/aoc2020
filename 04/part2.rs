
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str;
use std::iter::Iterator;
use std::collections::HashMap;

use regex::Regex;
use regex::Captures;

fn main() {
    let data: Vec<String> = get_input_lines("input.txt").collect();
    let passports = create_passports(data);

    let nvalid = passports.iter().filter(|p| valid_passport(p)).count();
    println!("{}", nvalid);
}

fn valid_passport(passport: &HashMap<String, String>) -> bool {
    valid_byr(passport)
    && valid_iyr(passport)
    && valid_eyr(passport)
    && valid_hgt(passport)
    && valid_hcl(passport)
    && valid_ecl(passport)
    && valid_pid(passport)
}

fn valid_byr(passport: &HashMap<String, String>) -> bool {
    let caps = match key_regex_match(passport, "byr", r"^(\d\d\d\d)$") {
        Some(c) => c,
        None => return false,
    };
    let n: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
    return 1920 <= n && n <= 2002;
}
fn valid_iyr(passport: &HashMap<String, String>) -> bool {
    let caps = match key_regex_match(passport, "iyr", r"^(\d\d\d\d)$") {
        Some(c) => c,
        None => return false,
    };
    let n: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
    return 2010 <= n && n <= 2020;
}
fn valid_eyr(passport: &HashMap<String, String>) -> bool {
    let caps = match key_regex_match(passport, "eyr", r"^(\d\d\d\d)$") {
        Some(c) => c,
        None => return false,
    };
    let n: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
    return 2020 <= n && n <= 2030;
}
fn valid_hgt(passport: &HashMap<String, String>) -> bool {
    let caps = match key_regex_match(passport, "hgt", r"^(\d+)(cm|in)$") {
        Some(c) => c,
        None => return false,
    };
    let n: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let unit = caps.get(2).unwrap().as_str();
    return unit == "cm" && 150 <= n && n <= 193
        || unit == "in" && 59 <= n && n <= 76;
}
fn valid_hcl(passport: &HashMap<String, String>) -> bool {
    return match key_regex_match(passport, "hcl", r"^#[a-f0-9]{6}$") {
        Some(c) => true,
        None => false,
    };
}
fn valid_ecl(passport: &HashMap<String, String>) -> bool {
    return match key_regex_match(
            passport, "ecl",
            r"^(amb|blu|brn|gry|grn|hzl|oth)$") {
        Some(c) => true,
        None => false,
    };
}
fn valid_pid(passport: &HashMap<String, String>) -> bool {
    return match key_regex_match(passport, "pid", r"^\d{9}$") {
        Some(c) => true,
        None => false,
    };
}

fn key_regex_match<'a>(passport: &'a HashMap<String, String>,
                   key: &'a str,
                   regex: &str) -> Option<Captures<'a>> {

    let string = passport.get(key)?;
    let re = Regex::new(regex).unwrap();
    return re.captures(string);
}

fn create_passports(data: Vec<String>) -> Vec<HashMap<String, String>> {
    let mut result: Vec<HashMap<String, String>> = vec![];
    let mut iter = data.into_iter();

    while let Some(first_line) = iter.next() {
        if first_line.len() == 0 {
            continue;
        }

        let mut passport: HashMap<String, String> = HashMap::new();
        let mut line = first_line;

        loop {
            for entry in line.split(" ") {
                let mut kv = entry.split(":").into_iter();
                let key = kv.next().unwrap().to_string();
                let value = kv.next().unwrap().to_string();
                passport.insert(key, value);
            }

            match iter.next() {
                None => break,
                Some(l) if l == "" => break,
                Some(l) => line = l,
            }
        }
        result.push(passport);
    }
    return result;
}

fn get_input_lines(filename: &str) -> impl Iterator<Item = String> {
    return BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|line| line.unwrap());
}

#[cfg(test)]
pub mod test{
    use super::*;

    fn map_singleton(key: &str, value: &str) -> HashMap<String,String> {
        [(key, value)].iter()
            .map(|&(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn test_byr() {
        assert!(valid_byr(&map_singleton("byr", "1929")));
        assert!(!valid_byr(&map_singleton("byr", "19293")));
        assert!(!valid_byr(&map_singleton("byr", "01")));
        assert!(!valid_byr(&map_singleton("byr", "2050")));
        assert!(!valid_byr(&map_singleton("byr", "1500")));
    }
    #[test]
    fn test_iyr() {
        assert!(valid_iyr(&map_singleton("iyr", "2012")));
        assert!(!valid_iyr(&map_singleton("iyr", "19293")));
        assert!(!valid_iyr(&map_singleton("iyr", "01")));
        assert!(!valid_iyr(&map_singleton("iyr", "2050")));
        assert!(!valid_iyr(&map_singleton("iyr", "2005")));
    }
    #[test]
    fn test_eyr() {
        assert!(valid_eyr(&map_singleton("eyr", "2022")));
        assert!(!valid_eyr(&map_singleton("eyr", "19293")));
        assert!(!valid_eyr(&map_singleton("eyr", "01")));
        assert!(!valid_eyr(&map_singleton("eyr", "2050")));
        assert!(!valid_eyr(&map_singleton("eyr", "2015")));
    }
    #[test]
    fn test_hgt() {
        assert!(valid_hgt(&map_singleton("hgt", "60in")));
        assert!(valid_hgt(&map_singleton("hgt", "190cm")));
        assert!(!valid_hgt(&map_singleton("hgt", "190in")));
        assert!(!valid_hgt(&map_singleton("hgt", "190")));
    }
    #[test]
    fn test_hcl() {
        assert!(valid_hcl(&map_singleton("hcl", "#123abc")));
        assert!(!valid_hcl(&map_singleton("hcl", "#123abz")));
        assert!(!valid_hcl(&map_singleton("hcl", "123abc")));
    }
    #[test]
    fn test_ecl() {
        assert!(valid_ecl(&map_singleton("ecl", "brn")));
        assert!(!valid_ecl(&map_singleton("ecl", "wat")));
    }
    #[test]
    fn test_pid() {
        assert!(valid_pid(&map_singleton("pid", "000000001")));
        assert!(!valid_pid(&map_singleton("pid", "0123456789")));
    }
}
