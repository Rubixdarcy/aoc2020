use nom::{
    IResult,
    combinator::{map_res},
    character::complete::{digit1, newline, multispace1},
    bytes::complete::{tag, take_until},
    sequence::{tuple, preceded, separated_pair},
    multi::{separated_list0, separated_list1},
};

fn main() {
    let input = Input::parse(include_str!("input.txt")).unwrap().1;

    let error_rate = input.nearby_tickets.iter()
        .flat_map(|t| t.iter())
        .filter(|n| !is_valid_n(**n, &input.fields))
        .sum::<u32>();

    println!("{}", error_rate);
}

fn is_valid_n(n: u32, fields: &Vec<Field>) -> bool {
    fields.iter()
        .find(|f| f.r1.0 <= n && n <= f.r1.1
                    || f.r2.0 <= n && n <= f.r2.1)
        .is_some()
}

#[derive(Clone, Debug)]
struct Field<'a> {
    name: &'a str,
    r1: (u32, u32),
    r2: (u32, u32),
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

#[derive(Clone, Debug)]
struct Input<'a> {
    fields: Vec<Field<'a>>,
    ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
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
        -> impl Fn(&str) -> IResult<&str, Vec<Vec<u32>>> + 'a {
    move |i: &str| {
        preceded(
            tuple((tag(name), tag(":"), newline)),
            separated_list0(newline,
                separated_list1(tag(","),
                    parse_num))
        )(i)
    }
}

fn parse_num(i: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(i)
}
