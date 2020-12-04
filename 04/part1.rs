
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str;
use std::iter::{Iterator,FromIterator};
use std::collections::HashMap;

fn main() {
    let data: Vec<String> = get_input_lines("input.txt").collect();
    let passports = create_passports(data);

    let nvalid = passports.iter().filter(|p| valid_passport(p)).count();
    println!("{}", nvalid);
}

fn valid_passport(passport: &HashMap<String, String>) -> bool {
    passport.contains_key("byr")
    && passport.contains_key("iyr")
    && passport.contains_key("eyr")
    && passport.contains_key("hgt")
    && passport.contains_key("hcl")
    && passport.contains_key("ecl")
    && passport.contains_key("pid")
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
