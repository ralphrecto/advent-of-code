pub fn dec_ith(decnum: i32, i: i32) -> u8 {
    return ((decnum % (10_i32.pow(i as u32))) / 10_i32.pow((i - 1) as u32)) as u8;
}