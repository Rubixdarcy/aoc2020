use nom::{
    IResult,
    combinator::{map_res, map, value, peek},
    character::complete::{digit1, space1, alpha1},
    bytes::complete::{tag,},
    multi::{separated_list0, separated_list1},
    branch::{alt},
    sequence::{delimited, separated_pair, tuple},
};
use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let mut lines = io::BufReader::new(fs::File::open("input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap());

    let rules: HashMap<u64, Rule> = (&mut lines)
        .take_while(|l| l != "")
        .map(|s| parse_rule_line(&s).unwrap().1)
        .collect();

    println!("{:?}", rules);

    match_rule(&lines.next().unwrap(), 0, &rules);
    
    //let result = lines
    //    .filter(|s| match_rule(s, 0, &rules).is_ok())
    //    .count();

    //println!("{}", result);
}

fn match_rule<'a>(mut i: &'a str, rule_n: &[u64], rules: &HashMap<u64, Rule>) -> IResult<&'a str, ()> {
    for &n in rule_n {
        let rule = rules.get(&n).unwrap();

        println!("Trying {:?} {} against \"{}\"", rule, rule_n, i);

        let v1 = match rule {
            Rule::Literal(s) => return value((), tag(&s[..]))(i),
            Rule::Branch(v) => v,
            Rule::Branches(v, _) => v,
        };
        for &n in v1 {
            let (new_i, _) = match_rule(i, n, rules)?;
            i = new_i;
        }
        if let Rule::Branches(_, v2) = rule {
            for &n in v2 {
                let (new_i, _) = match_rule(i, n, rules)?;
                i = new_i;
            }
        }
        return Ok((i, ()));
    }
}

fn parse_rule_line(i: &str) -> IResult<&str, (u64, Rule)> {
    separated_pair(parse_num, tag(": "), Rule::parse)(i)
}

#[derive(Clone, Debug, PartialEq)]
enum Rule {
    Literal(String),
    Branch(Vec<u64>),
    Branches(Vec<u64>, Vec<u64>),
}

impl Rule {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            map(delimited(tag("\""), alpha1, tag("\"")), |s: &str| Rule::Literal(s.to_owned())),
            map(separated_pair(parse_nums, tag(" | "), parse_nums),
                |(b1, b2)| Rule::Branches(b1, b2)),
            map(parse_nums, |b1| Rule::Branch(b1)),
        ))(i)
    }
}

fn parse_num(i: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse::<u64>())(i)
}

fn parse_nums(i: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tuple((space1, peek(digit1))), parse_num)(i)
}
