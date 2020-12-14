use nom::{
    IResult,
    multi::{separated_list0, many0},
    character::complete::{newline, multispace0, digit1},
    sequence::{tuple, delimited},
    branch::{alt},
    combinator::{value, map, map_res},
    bytes::complete::{tag},
};
use std::collections::HashMap;

fn main() {
    let ops = parse(include_str!("input.txt")).unwrap().1;

    let mut mask = Vec::new();
    let mut mem: HashMap<u64, u64> = HashMap::new();

    for op in ops {
        match op {
            Op::SetMask(m) => mask = m,
            Op::SetMem(idx, n) => {
                mem.insert(idx, use_mask(&mask, n));
            }
        }
    }
    println!("{}", mem.values().sum::<u64>());
}

fn use_mask(mask: &Vec<Mask>, mut n: u64) -> u64 {
    for (i, m) in mask.iter().rev().enumerate() {
        n = match m {
            Mask::Zero => n & !(1 << i),
            Mask::One =>  n | (1 << i),
            _ => n,
        }
    }
    return n;
}

fn parse(i: &str) -> IResult<&str, Vec<Op>> {
    separated_list0(newline, alt((
        map(tuple((tag("mem"),
                   parse_idx,
                   delimited(multispace0, tag("="), multispace0),
                   parse_n)), |(_, idx, _, n)| Op::SetMem(idx, n)),
        map(tuple((tag("mask"),
                   delimited(multispace0, tag("="), multispace0),
                   parse_mask)), |(_, _, mask)| Op::SetMask(mask)),
    )))(i)
}

fn parse_idx(i: &str) -> IResult<&str, u64> {
    delimited(tag("["), parse_n, tag("]"))(i)
}

fn parse_n(i: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse::<u64>())(i)
}

fn parse_mask(i: &str) -> IResult<&str, Vec<Mask>> {
    many0(alt((value(Mask::X,    tag("X")),
               value(Mask::Zero, tag("0")),
               value(Mask::One,  tag("1")))))(i)
}



#[derive(Debug)]
enum Op { SetMem(u64, u64), SetMask(Vec<Mask>), }

#[derive(Copy, Clone, Debug)]
enum Mask { Zero, One, X, }
