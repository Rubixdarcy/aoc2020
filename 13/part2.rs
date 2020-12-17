use nom::IResult;
use nom::character::complete::{newline, digit1};
use nom::combinator::{map_res, value};
use nom::sequence::{separated_pair};
use nom::multi::{separated_list0};
use nom::branch::alt;
use nom::bytes::complete::{tag};

// Problem Description:
//
// Given a set {(b_0, t_0), ..., (b_n, t_n)} find the smallest k s.t.
// (k + t_0 mod b_0) = 0, ..., (k + t_n mod b_n) = 0.
//
// So k + t_0 = m_0 b_0, ..., k + t_n = m_n b_n

fn main() {
    let (earliest, schedule) = parse(include_str!("input.txt")).unwrap().1;

    let mut n: i128 = 1;
    for (i, entry) in schedule.iter().enumerate() {
        let id = match entry { Entry::ID(id) => id, _ => continue };
        println!("n={}, i={}, id={}", n, i, id);
        let mut j: i128 = 1;
        while (n * j + i as i128) % id != 0 {
            //println!("    n * j + i = {}, % = {}", n * j + i as i128, n * j + i as i128 % id);
            j += 1;
        }
        println!("    j={}", j);
        n *= j;
    }

    //let (bus, wait) = schedule.iter()
    //    .filter_map(Entry::id)
    //    .map(|bus| (bus, (((earliest * -1) % bus) + bus) % bus))
    //    .min_by_key(|(_bus, wait)| *wait)
    //    .unwrap();

    println!("{}", n);
}

fn parse(i: &str) -> IResult<&str, (i128, Vec<Entry>)> {
    separated_pair(map_res(digit1, |s: &str| s.parse::<i128>()),
                   newline,
                   separated_list0(tag(","), alt((
                       value(Entry::X, tag("x")),
                       map_res(digit1, |s: &str| s.parse::<i128>().map(|i| Entry::ID(i)))))))(i)
}

#[derive(Copy, Clone, Debug)]
enum Entry {
    X,
    ID(i128),
}

impl Entry {
    fn id(&self) -> Option<i128>  {
        match self {
            Entry::X => None,
            Entry::ID(n) => Some(*n),
        }
    }
}
