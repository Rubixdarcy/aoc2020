
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
   println!("{:?}", read_to_string("input.txt").unwrap().split("\n\n")
       .map(|lines| lines.split('\n')
            .flat_map(move |line| line.chars())
            .collect::<HashSet<_>>()
            .len())
       .sum::<usize>());

}
