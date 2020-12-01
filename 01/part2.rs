
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::option::Option;

fn main() {
    let nums = get_input("input.txt");
    match calculate_result(2020, &nums) {
        Some(ans) => println!("{}", ans),
        None => println!("Not found")
    }
}

fn calculate_result(target: u64, nums: &Vec<u64>) -> Option<u64> {
    for a in nums.iter() {
        for b in nums.iter() {
            for c in nums.iter() {
                if a + b + c == target {
                    return Some(a * b * c);
                }
            }
        }
    }
    return None;
}

fn get_input(filename: &str) -> Vec<u64> {
    return BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
}
