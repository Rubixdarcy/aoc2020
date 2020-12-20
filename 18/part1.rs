use nom::{
    IResult,
    combinator::{map_res, map, value},
    character::complete::{digit1, space1},
    bytes::complete::{tag,},
    multi::{separated_list0,},
    branch::{alt},
    sequence::{delimited},
};
use std::io;
use std::fs;
use std::io::BufRead;
use std::cmp::Ordering;
use core::iter::Peekable;
use std::iter::Fuse;

fn main() {
    let reader = io::BufReader::new(fs::File::open("input.txt").unwrap());

    let result = reader.lines()
        .map(|l| parse_line(&l.unwrap()).unwrap().1)
        .map(|tokens| eval_line(tokens))
        .sum::<u64>();

    println!("{}", result);
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Token { Open, Close, Sum, Prod, Num(u64) }

impl Token {
    fn precedence(&self) -> i32 {
        match self {
            Token::Open => 1,
            Token::Close => 1,
            Token::Sum => 2,
            Token::Prod => 2,
            Token::Num(_) => 3,
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        self.precedence().cmp(&other.precedence())
    }
}

struct ShuntingYard<I: Iterator> {
    input: Peekable<Fuse<I>>,
    ops: Vec<Token>,
}

impl<I: Iterator> ShuntingYard<I> {
    fn new<T>(input: T) -> Self where
            T: IntoIterator<IntoIter = I, Item = Token> {
        let input = input.into_iter().fuse().peekable();
        let ops: Vec<Token> = Vec::new();

        ShuntingYard { input, ops }
    }
}

impl<I> Iterator for ShuntingYard<I>
        where I: Iterator<Item = Token> {

    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let l = self.ops.last().copied();
            let r = self.input.peek().copied();

            match (l, r) {
                (None, None) => return None,
                (Some(_t), None) => return self.ops.pop(),
                (None, Some(_t)) => self.ops.push(self.input.next().unwrap()),
                (Some(Token::Open), Some(Token::Close)) => {
                    self.ops.pop();
                    self.input.next();
                },
                (_, Some(Token::Open)) => self.ops.push(self.input.next().unwrap()),
                (Some(tl), Some(tr))
                        if tl.cmp(&tr) != Ordering::Less => return self.ops.pop(),
                _ => self.ops.push(self.input.next().unwrap())
            };
        }
    }

}

fn eval_line(line: Vec<Token>) -> u64 {
    let sy = ShuntingYard::new(line);
    let mut stack: Vec<u64> = Vec::new();

    for token in sy {
        match token {
            Token::Num(n) => stack.push(n),
            Token::Sum => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
            },
            Token::Prod => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a * b);
            },
            x => panic!("Can't eval token: {:?}", x),
        }
    }

    return stack[0];
}

fn parse_line(i: &str) -> IResult<&str, Vec<Token>> {
    map(separated_list0(space1, alt((
        map(parse_num, |n| vec![Token::Num(n)]),
        value(vec![Token::Sum], tag("+")),
        value(vec![Token::Prod], tag("*")),
        map(delimited(tag("("), parse_line, tag(")")), |v| {
            let mut new_v = vec![Token::Open];
            new_v.extend(v);
            new_v.push(Token::Close);
            new_v
        }),
    ))),
        |l_o_l| l_o_l.into_iter().flat_map(|l| l.into_iter()).collect::<Vec<Token>>())(i)
}

fn parse_num(i: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse::<u64>())(i)
}
