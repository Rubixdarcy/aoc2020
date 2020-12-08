use nom::{
  IResult,
  character::complete::{space0, digit1, newline, anychar},
  bytes::complete::{tag},
  combinator::{map, map_res, value, recognize},
  sequence::{separated_pair, pair},
  branch::{alt},
  multi::{separated_list1},
};
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum Op { Nop, Acc, Jmp, }

#[derive(Debug, Copy, Clone)]
struct Inst {
    op: Op,
    arg: i32
}

#[derive(Debug, Copy, Clone)]
struct VM {
    pc: i32,
    acc: i32,
}

fn main() {
    let raw_input = include_str!("input.txt");
    let code = parse_code(raw_input).unwrap().1;

    let mut vm = VM { pc: 0, acc: 0 };
    let mut seen_insts: HashSet<i32> = HashSet::new();

    while !seen_insts.contains(&vm.pc) {
        seen_insts.insert(vm.pc);
        vm = next_vm(&vm, &code);
    }
    
    println!("{}", vm.acc);
}

fn next_vm(vm: &VM, code: &Vec<Inst>) -> VM {
    let inst = code.get(vm.pc as usize).unwrap();
    match inst.op {
        Op::Nop => VM { pc: vm.pc + 1,        acc: vm.acc },
        Op::Acc => VM { pc: vm.pc + 1,        acc: vm.acc + inst.arg },
        Op::Jmp => VM { pc: vm.pc + inst.arg, acc: vm.acc },
    }
}



fn parse_code(i: &str) -> IResult<&str, Vec<Inst>> {
    separated_list1(
        newline,
        map(
            separated_pair(
                alt((value(Op::Nop, tag("nop")),
                     value(Op::Acc, tag("acc")),
                     value(Op::Jmp, tag("jmp")),)),
                space0,
                map_res(recognize(pair(anychar, digit1)), |s: &str| s.parse::<i32>())),
            |(op, arg)| Inst { op, arg } ))(i)
}
