
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str;
use std::iter::Iterator;

fn main() {
    let max: u32 = get_input_lines("input.txt")
        .map(|s| s.replace("F", "0")
                  .replace("B", "1")
                  .replace("L", "0")
                  .replace("R", "1"))
        .map(|s| isize::from_str_radix(&s, 2).unwrap() as u32)
        .fold(0, u32::max);
    println!("{}", max);
}

fn get_input_lines(filename: &str) -> impl Iterator<Item = String> {
    return BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|line| line.unwrap());
}


#[cfg(test)]
pub mod test{
    use super::*;

    #[test]
    fn test() {
        println!("Hello world");
    }
}
