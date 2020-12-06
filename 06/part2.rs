
use std::collections::HashSet;
use std::fs::read_to_string;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn alphabet() -> HashSet<char> { ALPHABET.chars().collect() }

fn main() {
    println!("{:?}", read_to_string("input.txt").unwrap().split("\n\n")
        .map(|lines| {
            let mut all = alphabet();
            for line in lines.split('\n') {
                for c in alphabet().difference(&line.chars().collect::<HashSet<_>>()) {
                    all.remove(&c);
                }
            }
            all.len()
        })
        .sum::<usize>());
}
