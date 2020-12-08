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

    let n = (0..code.len())
        .map(|i| terminating_acc_value(&code, i as i32))
        .flatten()
        .next()
        .unwrap();

    println!("{}", n);
}

fn terminating_acc_value(code: &Vec<Inst>, toggle_id: i32) -> Option<i32> {
    let mut vm = VM { pc: 0, acc: 0 };
    let mut seen_insts: HashSet<i32> = HashSet::new();
    let terminating_pc = code.len() as i32;

    while !seen_insts.contains(&vm.pc) {
        if vm.pc == terminating_pc {
            return Some(vm.acc);
        }
        seen_insts.insert(vm.pc);
        vm = next_vm(&vm, &code, toggle_id);
    }
    return None;
}

fn next_vm(vm: &VM, code: &Vec<Inst>, toggle_id: i32) -> VM {
    let mut inst: Inst = *code.get(vm.pc as usize).unwrap();

    // toggle if required
    if vm.pc == toggle_id {
        inst.op = match inst.op {
            Op::Nop => Op::Jmp,
            Op::Acc => Op::Acc,
            Op::Jmp => Op::Nop,
        }
    }

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
