#![feature(slice_patterns)]

use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug)]
enum Instr {
    Hlf(char),
    Tpl(char),
    Inc(char),
    Jmp(i32),
    Jie(char, i32),
    Jio(char, i32),
}

fn parse_register(r: &str) -> char {
    r.chars().nth(0).expect("register expected")
}

fn parse_offset(o: &str) -> i32 {
    o.parse().expect("offset expected")
}

type Registers = HashMap<char, i64>;

fn map_register<F>(rs: &mut Registers, r: char, mut f: F)
    where F: FnMut(i64) -> i64
{
    if !rs.contains_key(&r) {
        rs.insert(r, 0);
    }

    let v = rs.get_mut(&r).unwrap();
    *v = f(*v);
}

fn main() {
    let mut rs = Registers::new();

    let f = io::stdin();
    let prog = f.lock()
                .lines()
                .filter_map(|l| l.ok())
                .filter_map(|l| {
                    match &l.split(" ").collect::<Vec<_>>()[..] {
                        ["hlf", r] => Some(Instr::Hlf(parse_register(r))),
                        ["tpl", r] => Some(Instr::Tpl(parse_register(r))),
                        ["inc", r] => Some(Instr::Inc(parse_register(r))),
                        ["jmp", o] => Some(Instr::Jmp(parse_offset(o))),
                        ["jie", r, o] => Some(Instr::Jie(parse_register(r), parse_offset(o))),
                        ["jio", r, o] => Some(Instr::Jio(parse_register(r), parse_offset(o))),

                        _ => None,
                    }
                })
                .collect::<Vec<_>>();

    let mut i: i64 = 0;

    while i < prog.len() as i64 {
        match prog[i as usize] {
            Instr::Hlf(r) => map_register(&mut rs, r, |v| v / 2),
            Instr::Tpl(r) => map_register(&mut rs, r, |v| v * 3),
            Instr::Inc(r) => map_register(&mut rs, r, |v| v + 1),
            Instr::Jmp(o) => i += o as i64 - 1,
            Instr::Jie(r, o) => {
                map_register(&mut rs, r, |v| {
                    if v % 2 == 0 {
                        i += o as i64 - 1;
                    }
                    v
                })
            }
            Instr::Jio(r, o) => {
                map_register(&mut rs, r, |v| {
                    if v == 1 {
                        i += o as i64 - 1;
                    }
                    v
                })
            }
        }

        i += 1;
    }

    println!("Value of register b: {}", rs[&'b']);
}
