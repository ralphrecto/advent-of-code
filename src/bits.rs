pub fn dec_ith(decnum: i32, i: i32) -> u8 {
    return ((decnum % (10_i32.pow(i as u32))) / 10_i32.pow((i - 1) as u32)) as u8;
}

#[cfg(test)]
mod tests {

    use bits::*;

    #[test]
    fn test_dec_ith() {
        assert_eq!(dec_ith(4002, 1), 2);
        assert_eq!(dec_ith(4002, 2), 0);
        assert_eq!(dec_ith(4002, 3), 0);
        assert_eq!(dec_ith(4002, 4), 4);
    }

}