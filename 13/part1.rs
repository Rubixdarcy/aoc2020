use nom::IResult;
use nom::character::complete::{newline, digit1};
use nom::combinator::{map_res, value};
use nom::sequence::{separated_pair};
use nom::multi::{separated_list0};
use nom::branch::alt;
use nom::bytes::complete::{tag};

fn main() {
    let (earliest, schedule) = parse(include_str!("input.txt")).unwrap().1;

    let (bus, wait) = schedule.iter()
        .filter_map(Entry::id)
        .map(|bus| (bus, (((earliest * -1) % bus) + bus) % bus))
        .min_by_key(|(_bus, wait)| *wait)
        .unwrap();

    println!("{}", bus * wait);
}

fn parse(i: &str) -> IResult<&str, (i32, Vec<Entry>)> {
    separated_pair(map_res(digit1, |s: &str| s.parse::<i32>()),
                   newline,
                   separated_list0(tag(","), alt((
                       value(Entry::X, tag("x")),
                       map_res(digit1, |s: &str| s.parse::<i32>().map(|i| Entry::ID(i)))))))(i)
}

#[derive(Copy, Clone, Debug)]
enum Entry {
    X,
    ID(i32),
}

impl Entry {
    fn id(&self) -> Option<i32>  {
        match self {
            Entry::X => None,
            Entry::ID(n) => Some(*n),
        }
    }
}
