use bits;
use std::io;
use std::io::prelude::*;

#[derive(Clone, Copy, Debug)]
enum ParameterMode {
    Position,
    Immediate
}

#[derive(PartialEq, Debug)]
enum Opcode {
    Plus,
    Mult,
    Store,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt
}

impl Opcode {
    fn arity(&self) -> usize {
        return match self {
            Opcode::Plus |
            Opcode::Mult |
            Opcode::LessThan |
            Opcode::Equals => 3,
            Opcode::Output |
            Opcode::Store => 1,
            Opcode::JumpIfTrue |
            Opcode::JumpIfFalse => 2,
            Opcode::Halt => 0
        }
    }

    fn parse(instr: i32) -> (Opcode, Vec<ParameterMode>) {
        let opcode = match instr % 100 {
            1 => Opcode::Plus,
            2 => Opcode::Mult,
            3 => Opcode::Store,
            4 => Opcode::Output,
            5 => Opcode::JumpIfTrue,
            6 => Opcode::JumpIfFalse,
            7 => Opcode::LessThan,
            8 => Opcode::Equals,
            99 => Opcode::Halt, 
            _ => panic!("unknown opcode {}", instr % 100)
        };

        let mut param_modes = Vec::with_capacity(opcode.arity());
        for i in 0..opcode.arity() {
            let param_mode = match bits::dec_ith(instr, (i + 3) as i32) {
                0 => ParameterMode::Position,
                1 => ParameterMode::Immediate,
                _ => panic!("unknown parameter mode code")
            };

            param_modes.push(param_mode);
        }

        return (opcode, param_modes);
    }

    fn mode(&self, mode: ParameterMode, i: usize) -> ParameterMode {
        match self {
            Opcode::Plus |
            Opcode::Mult |
            Opcode::LessThan |
            Opcode::Equals => if i == 3 { ParameterMode::Immediate } else { mode },
            Opcode::Store => ParameterMode::Immediate,
            Opcode::Output |
            Opcode::JumpIfTrue |
            Opcode::JumpIfFalse => mode,
            Opcode::Halt => panic!("halt has 0 arity")
        }
    }
}

pub fn run_prog(mem: &mut Vec<i32>) -> () {
    let mut prog_count = 0;
    loop {
        let (op, param_modes) = Opcode::parse(mem[prog_count]);

        let mut param_literals: Vec<i32> = Vec::with_capacity(op.arity());
        let mut params: Vec<i32> = Vec::with_capacity(op.arity());
        for i in 0..op.arity() {
            let val = mem[prog_count + i + 1];
            let param = match op.mode(param_modes[i], i + 1) {
                ParameterMode::Position => mem[val as usize],
                ParameterMode::Immediate => val
            };

            param_literals.push(val);
            params.push(param);
        }

        let mut mutated_prog_count = false;
        match op {
            Opcode::Plus => {
                mem[params[2] as usize] = params[0] + params[1];
            },
            Opcode::Mult => {
                mem[params[2] as usize] = params[0] * params[1];
            },
            Opcode::Store => {
                print!("input: ");
                io::stdout().flush();

                let mut input = String::new();
                io::stdin().read_line(&mut input);

                mem[params[0] as usize] = input.trim().parse().unwrap();
            },
            Opcode::Output => {
                println!("{}", params[0]);
            },
            Opcode::JumpIfTrue => {
                if params[0] != 0 {
                    prog_count = params[1] as usize;
                    mutated_prog_count = true;
                }
            },
            Opcode::JumpIfFalse => {
                if params[0] == 0 {
                    prog_count = params[1] as usize;
                    mutated_prog_count = true;
                }
            },
            Opcode::LessThan => {
                mem[params[2] as usize] = if params[0] < params[1] { 1 } else { 0 };
            },
            Opcode::Equals => {
                mem[params[2] as usize] = if params[0] == params[1] { 1 } else { 0 };
            },
            Opcode::Halt => {
                break
            }
        }

        if !mutated_prog_count {
            prog_count += op.arity() + 1;
        }
    }
}

pub fn parse_prog(prog: &str) -> Vec<i32> {
    return prog.split(',')
        .map(|cmp| cmp.parse::<i32>().unwrap())
        .collect();
}