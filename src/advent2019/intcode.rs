use bits;
use std::io::stdin;

enum ParameterMode {
    Position,
    Immediate
}

#[derive(PartialEq)]
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
            Opcode::Plus => 2,
            Opcode::Mult => 2,
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
            99 => Opcode::Halt, 
            _ => panic!("unknown opcode {}", instr % 100)
        };

        let mut param_modes = Vec::with_capacity(opcode.arity());
        for i in 0..opcode.arity() {
            let param_mode = match bits::dec_ith(instr, (i + 2) as i32) {
                0 => ParameterMode::Position,
                1 => ParameterMode::Immediate,
                _ => panic!("unknown parameter mode code")
            };

            param_modes[i] = param_mode;
        }

        return (opcode, param_modes);
    }

    fn can_be_immediate(&self) -> bool {
        return *self != Opcode::Store;
    }
}

pub fn run_prog(mem: &mut Vec<i32>) -> () {
    let mut prog_count = 0;
    loop {
        let (op, param_modes) = Opcode::parse(mem[prog_count]);

        let mut params: Vec<i32> = Vec::with_capacity(op.arity());
        for i in 0..op.arity() {
            let val = mem[prog_count + i + 1];
            let param = match param_modes[i] {
                ParameterMode::Position => mem[val as usize],
                ParameterMode::Immediate => val
            };

            params[i] = param;
        }

        match op {
            Opcode::Plus => {
                mem[params[2] as usize] = params[0] + params[1];
            }
            Opcode::Mult => {
                mem[params[2] as usize] = params[0] * params[1];
            },
            Opcode::Store => {
                print!("input: ");
                let mut input = String::new();
                stdin().read_line(&mut input);

                let input_val: i32 = input.parse().unwrap(); 
                mem[params[0] as usize] = input_val;
            },
            Opcode::Output => {
                println!("{}", mem[params[0] as usize]);
            }
            Opcode::Halt => {
                break
            }
        }
    }

}