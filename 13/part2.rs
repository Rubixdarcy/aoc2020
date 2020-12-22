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
    let (_earliest, schedule) = parse(include_str!("input.txt")).unwrap().1;

    let id_offsets: Vec<(i128, i128)> = schedule.iter()
        .copied()
        .enumerate()
        .filter_map(|(i, entry)| entry.id().map(|id| (id, i as i128)))
        .collect();

    let mut earliest: i128 = 0;
    let mut frequency: i128 = 1;

    for (id, offset) in id_offsets {
        // Shift the earliest departure forward by increments of `offset`
        // until the current bus arrives at the desired time.
        while (earliest + offset) % id != 0 {
            earliest += frequency;
        }
        // Update the frequency to ensure the current bus always arrives at the
        // correct time in future iterations. The new frequency should be the
        // lowest common multiple of the ID and the old frequency. Since the
        // IDs are all prime, just multiplying will suffice.
        frequency *= id;
    }

    println!("{}", earliest);
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
    fn id(self) -> Option<i128>  {
        match self {
            Entry::X => None,
            Entry::ID(n) => Some(n),
        }
    }
}
