
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::option::Option;
use std::str;
use std::iter::Iterator;

fn main() {
    let count = get_input_lines("input.txt")
        .filter(|s| Line::read(&mut s.as_bytes()).validate())
        .count();
    println!("{}", count);
}

#[derive(Debug)]
struct Policy {
    min: u32,
    max: u32,
    c: u8,
}

impl Policy {
    fn read(buf: &mut &[u8]) -> Self {
        let min = get_num(buf);
        get_byte(buf); // hyphen
        let max = get_num(buf);
        get_byte(buf); // space
        let c = get_byte(buf);

        return Policy { min, max, c };
    }

    fn validate(&self, pwd: &[u8]) -> bool {
        let cmin = pwd[(self.min - 1) as usize];
        let cmax = pwd[(self.max - 1) as usize];

        return (cmin == self.c || cmax == self.c) && cmin != cmax;
    }
}


#[derive(Debug)]
struct Line<'a> {
    policy: Policy,
    pwd: &'a [u8],
}

impl<'a> Line<'a> {
    fn read(buf: &mut &'a [u8]) -> Self {
        let policy = Policy::read(buf);
        get_byte(buf); // colon
        get_byte(buf); // space
        let pwd = *buf;
        return Line { policy, pwd };
    }

    fn validate(&self) -> bool {
        self.policy.validate(self.pwd)
    }
}

/// Read a byte from the front of buf
fn get_byte(buf: &mut &[u8]) -> u8 {
    let c = (*buf)[0];
    *buf = &(*buf)[1..];
    return c;
}

/// Read a u32 from the front of buf
fn get_num(buf: &mut &[u8]) -> u32 {
    let mut num: u32 = 0;
    let mut c: u8 = (*buf)[0];

    while '0' as u8 <= c && c <= '9' as u8 {
        let digit = c as u32 - '0' as u32;
        num = num * 10 + digit;

        *buf = &(*buf)[1..];
        c = buf[0];
    }

    return num;
}

fn get_input_lines(filename: &str) -> impl Iterator<Item = String> {
    return BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|line| line.unwrap());
}
