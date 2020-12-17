use nom::{
    IResult,
    combinator::{map_res},
    character::complete::{digit1, newline, multispace1},
    bytes::complete::{tag, take_until},
    sequence::{tuple, preceded, separated_pair},
    multi::{separated_list0, separated_list1},
};
use std::collections::HashSet;

const NNUMS: usize = 20;
//const NNUMS: usize = 3;

fn main() {
    let Input { fields, ticket, nearby_tickets } = Input::parse(include_str!("input.txt")).unwrap().1;
    //let Input { fields, ticket, nearby_tickets } = Input::parse(include_str!("input-test.txt")).unwrap().1;

    let valid_tickets: Vec<Vec<u64>> = nearby_tickets.into_iter()
        .filter(|t| valid_ticket(t, &fields))
        .collect();

    let elligible_fields: Vec<Vec<usize>> = (0..NNUMS)
        .map(|i|
            fields.iter()
                .enumerate()
                .filter(|(j, f)| valid_field(f, valid_tickets.iter().map(|t| t[i])))
                .map(|(j, _f)| j)
                .collect::<Vec<usize>>()
        )
        .collect();

    let mut ordered_fields: Vec<usize> = Vec::new();
    let mut seen_fields: HashSet<usize> = HashSet::new();

    for i in 0..NNUMS {
        let fields_i = elligible_fields.iter().find(|v| v.len() == i + 1).unwrap();
        for field in fields_i.iter().cloned() {
            if !seen_fields.contains(&field) {
                ordered_fields.push(field);
                seen_fields.insert(field);
                break;
            }
        }
    }
    
    println!("Field ids: {:?}", ordered_fields);

    let result = ticket.iter()
        .cloned()
        .enumerate()
        .map(|(j, v)| (fields[ordered_fields[j]].name, v))
        .inspect(|x| println!("{:?}", x))
        .filter(|(name, v)| name.starts_with("departure"))
        .inspect(|x| println!("Only using: {:?}", x))
        .map(|(_name, v)| v)
        .product::<u64>();
    println!("{}", result);
}

fn valid_field<T: IntoIterator<Item=u64>>(field: &Field, ns: T) -> bool {
    ns.into_iter()
        .all(|n| field.r1.0 <= n && n <= field.r1.1
                    || field.r2.0 <= n && n <= field.r2.1)
}

fn valid_ticket(ticket: &Vec<u64>, fields: &Vec<Field>) -> bool {
    ticket.iter().all(|&n| is_valid_n(n, fields))
}

fn is_valid_n(n: u64, fields: &Vec<Field>) -> bool {
    fields.iter()
        .find(|f| f.r1.0 <= n && n <= f.r1.1
                    || f.r2.0 <= n && n <= f.r2.1)
        .is_some()
}

#[derive(Clone, Debug, PartialEq, Hash)]
struct Field<'a> {
    name: &'a str,
    r1: (u64, u64),
    r2: (u64, u64),
}

impl<'a> Field<'a> {
    fn parse(i: &'a str) -> IResult<&'a str, Self> {
        let (i, name) = take_until(":")(i)?;
        let (i, _) = tag(": ")(i)?;
        let (i, range_vec) = separated_list1(tag(" or "),
            separated_pair(parse_num, tag("-"), parse_num))(i)?;
        Ok((i, Field { name, r1: range_vec[0], r2: range_vec[1] }))
    }
}

#[derive(Clone, Debug, PartialEq, Hash)]
struct Input<'a> {
    fields: Vec<Field<'a>>,
    ticket: Vec<u64>,
    nearby_tickets: Vec<Vec<u64>>,
}

impl<'a> Input<'a> {
    fn parse(i: &'a str) -> IResult<&'a str, Self> {
        let (i, fields) = separated_list1(newline, Field::parse)(i)?;
        let (i, _) = multispace1(i)?;
        let (i, my_tickets) = ticket_list("your ticket")(i)?;
        let (i, _) = multispace1(i)?;
        let (i, nearby_tickets) = ticket_list("nearby tickets")(i)?;

        Ok((i, Input {
            fields,
            ticket: my_tickets.into_iter().next().unwrap(),
            nearby_tickets,
        }))
    }
}

fn ticket_list<'a>(name: &'a str) 
        -> impl Fn(&str) -> IResult<&str, Vec<Vec<u64>>> + 'a {
    move |i: &str| {
        preceded(
            tuple((tag(name), tag(":"), newline)),
            separated_list0(newline,
                separated_list1(tag(","),
                    parse_num))
        )(i)
    }
}

fn parse_num(i: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse::<u64>())(i)
}
