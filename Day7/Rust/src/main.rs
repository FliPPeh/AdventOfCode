#![feature(convert)]
#![feature(slice_patterns)]

use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Operand {
    Direct(u16),
    Indirect(String),
}

impl Operand {
    fn new(reg: &str) -> Operand {
        if reg.chars().all(|c| "0123456789".contains(c)) {
            Operand::Direct(reg.parse().ok().unwrap_or(0))
        } else {
            Operand::Indirect(reg.to_owned())
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Mov(Operand),
    Not(Operand),

    And(Operand, Operand),
    Or(Operand, Operand),
    Rshift(Operand, Operand),
    Lshift(Operand, Operand),
}

enum WireState {
    Defer(Instruction),
    Done(u16),
}

struct Circuit {
    wires: HashMap<String, WireState>,
}

impl Circuit {
    fn new() -> Circuit {
        Circuit { wires: HashMap::new() }
    }

    fn set_wire(&mut self, reg: &str, ins: Instruction) {
        self.wires.insert(reg.to_owned(), WireState::Defer(ins));
    }

    fn evaluate(&mut self, op: &Operand) -> u16 {
        match op {
            &Operand::Direct(x) => x,
            &Operand::Indirect(ref r) => self.calculate(r),
        }
    }

    fn calculate(&mut self, reg: &str) -> u16 {
        let i = match self.wires.get(reg) {
            None => panic!("unknown wire {}", reg),

            Some(&WireState::Defer(ref r)) => r.clone(),
            Some(&WireState::Done(ref r)) => return *r,
        };

        let r = match i {
            Instruction::Mov(ref op) => self.evaluate(op),
            Instruction::Not(ref a) => !self.evaluate(a),

            Instruction::And(ref a, ref b) => self.evaluate(a) & self.evaluate(b),
            Instruction::Or(ref a, ref b) => self.evaluate(a) | self.evaluate(b),
            Instruction::Lshift(ref a, ref b) => self.evaluate(a) << self.evaluate(b),
            Instruction::Rshift(ref a, ref b) => self.evaluate(a) >> self.evaluate(b),
        };

        self.wires.insert(reg.to_owned(), WireState::Done(r));

        r
    }
}

fn main() {
    let mut instructions = Vec::<(String, Instruction)>::new();

    loop {
        let mut line = String::new();

        match std::io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let r: Vec<&str> = line.trim().split(" ").collect();

                instructions.push(match r.as_slice() {
                    [a, "AND", b, "->", r] => {
                        (r.to_owned(),
                         Instruction::And(Operand::new(a), Operand::new(b)))
                    }

                    [a, "OR", b, "->", r] => {
                        (r.to_owned(),
                         Instruction::Or(Operand::new(a), Operand::new(b)))
                    }

                    [a, "RSHIFT", b, "->", r] => {
                        (r.to_owned(),
                         Instruction::Rshift(Operand::new(a), Operand::new(b)))
                    }

                    [a, "LSHIFT", b, "->", r] => {
                        (r.to_owned(),
                         Instruction::Lshift(Operand::new(a), Operand::new(b)))
                    }

                    ["NOT", a, "->", r] => (r.to_owned(), Instruction::Not(Operand::new(a))),

                    [a, "->", r] => (r.to_owned(), Instruction::Mov(Operand::new(a))),

                    _ => panic!("bad instruction"),
                });
            }

            Err(e) => panic!("error: {}", e),
        }
    }

    let mut c = Circuit::new();
    for &(ref reg, ref instr) in instructions.iter() {
        c.set_wire(&reg, instr.clone());
    }

    let a = c.calculate("a");
    println!("wire a: {}", a);

    for &(ref reg, ref instr) in instructions.iter() {
        c.set_wire(&reg, instr.clone());
    }

    c.wires.insert("b".to_owned(), WireState::Done(a));

    let an = c.calculate("a");
    println!("wire a: {}", an);
}
