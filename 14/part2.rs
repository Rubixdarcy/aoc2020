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
use itertools::Itertools;
use std::mem;

fn main() {
    let ops = parse(include_str!("input.txt")).unwrap().1;

    let mut mask = Vec::new();
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for op in ops {
        match op {
            Op::SetMask(m) => mask = m,
            Op::SetMem(address, n) => {
                for idx in mask_addresses(&mask, address) {
                    memory.insert(idx, n);
                }
            }
        }
    }
    println!("{}", memory.values().sum::<u64>());
}

fn mask_addresses(mask: &Vec<Mask>, address: u64) -> impl Iterator<Item=u64> {
    mask.iter()
        .rev()
        .enumerate()
        .map(|(i, m)| {
            match m {
                Mask::Zero => SmallIter::One(address & (1 << i)),
                Mask::One => SmallIter::One(1 << i),
                Mask::X => SmallIter::Two(0, 1 << i),
            }
        })
        .multi_cartesian_product()
        .map(|v| v.iter().sum::<u64>())
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mask_addresses() {
        let mask = vec![Mask::X, Mask::Zero, Mask::One, Mask::X];
        let address: u64 = 0b0110;
        let result: Vec<u64> = mask_addresses(&mask, address).collect();
        assert_eq!(result, vec![0b0110, 0b1110, 0b0111, 0b1111]);
    }
}



#[derive(Debug)]
enum Op { SetMem(u64, u64), SetMask(Vec<Mask>), }

#[derive(Copy, Clone, Debug)]
enum Mask { Zero, One, X, }

#[derive(Copy, Clone, Debug)]
enum SmallIter { Zero, One(u64), Two(u64, u64) }

impl Iterator for SmallIter {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        match self {
            SmallIter::Zero => None,
            SmallIter::One(a) => {
                let n = *a;
                mem::replace(self, SmallIter::Zero);
                Some(n)
            },
            SmallIter::Two(a, b) => {
                let n = *a;
                let m = *b;
                mem::replace(self, SmallIter::One(m));
                Some(n)
            }
        }
    }
}
