use bits;
use std::io;
use std::io::prelude::*;

#[derive(Clone, Copy)]
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
    Halt
}

impl Opcode {
    fn arity(&self) -> usize {
        return match self {
            Opcode::Plus => 3,
            Opcode::Mult => 3,
            Opcode::Store => 1,
            Opcode::Output => 1,
            Opcode::Halt => 0
        }
    }

    fn parse(instr: i32) -> (Opcode, Vec<ParameterMode>) {
        let opcode = match instr % 100 {
            1 => Opcode::Plus,
            2 => Opcode::Mult,
            3 => Opcode::Store,
            4 => Opcode::Output,
            99 => Opcode::Halt, 
            _ => panic!("unknown opcode {}", instr % 100)
        };
        // println!("instr {}, opcode {:?}", instr, opcode);

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
            Opcode::Plus => if i == 3 { ParameterMode::Immediate } else { mode },
            Opcode::Mult => if i == 3 { ParameterMode::Immediate } else { mode },
            Opcode::Output => ParameterMode::Immediate,
            Opcode::Store => ParameterMode::Immediate,
            Opcode::Halt => panic!("halt has 0 arity")
        }
    }
}

pub fn run_prog(mem: &mut Vec<i32>) -> () {
    let mut prog_count = 0;
    loop {
        let (op, param_modes) = Opcode::parse(mem[prog_count]);

        let mut params: Vec<i32> = Vec::with_capacity(op.arity());
        for i in 0..op.arity() {
            let val = mem[prog_count + i + 1];
            let param = match op.mode(param_modes[i], i + 1) {
                ParameterMode::Position => {
                    // println!("val {}, op {:?}, prog_count {}", val, op, prog_count);
                    mem[val as usize]
                }
                ParameterMode::Immediate => val
            };

            params.push(param);
        }

        // println!("op {:?}, prog_count {}, params {:?}", op, prog_count, params);
        match op {
            Opcode::Plus => {
                // println!("writing {} to {}", params[0] + params[1], params[2]);
                mem[params[2] as usize] = params[0] + params[1];
            },
            Opcode::Mult => {
                // println!("writing {} to {}", params[0] * params[1], params[2]);
                mem[params[2] as usize] = params[0] * params[1];
            },
            Opcode::Store => {
                print!("input: ");
                io::stdout().flush();

                let mut input = String::new();
                io::stdin().read_line(&mut input);

                let input_val: i32 = input.trim().parse().unwrap(); 

                // println!("writing {} to {}", input_val, params[0]);
                mem[params[0] as usize] = input_val;
            },
            Opcode::Output => {
                println!("{}", mem[params[0] as usize]);
            }
            Opcode::Halt => {
                break
            }
        }

        prog_count += op.arity() + 1;
    }
}

pub fn parse_prog(prog: &str) -> Vec<i32> {
    return prog.split(',')
        .map(|cmp| cmp.parse::<i32>().unwrap())
        .collect();
}