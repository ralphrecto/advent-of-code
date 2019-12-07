pub fn run_prog(ops: &mut Vec<usize>) -> () {
    let mut current_op_idx = 0;
    let mut current_op = ops[current_op_idx];
    while current_op != 99 {
        let lhs = ops[ops[current_op_idx + 1]];
        let rhs = ops[ops[current_op_idx + 2]];
        let loc = ops[current_op_idx + 3];

        if current_op == 1 {
            ops[loc] = lhs + rhs; 
        } else if current_op == 2 {
            ops[loc] = lhs * rhs; 
        } else {
            panic!("unknown op code {}", current_op);
        }

        current_op_idx += 4;
        current_op = ops[current_op_idx];
    }

}