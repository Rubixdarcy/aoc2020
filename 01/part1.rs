
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

fn calculate_result(target: u32, nums: &Vec<u32>) -> Option<u32> {
    for a in nums.iter() {
        for b in nums.iter() {
            if a + b == target {
                return Some(a * b);
            }
        }
    }
    return None;
}

fn get_input(filename: &str) -> Vec<u32> {
    return BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
}
