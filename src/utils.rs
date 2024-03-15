pub fn power_of_two(number: usize) -> bool{
    (number != 0) && ((number & (number - 1)) == 0)
}


pub fn convert_char_bit_to_int(bit: char) -> usize {
    bit as usize - '0' as usize
}

pub fn binary_to_decimal(bits: &str) -> usize {
    let mut result = 0;
    if !bits.is_empty() {
        for index in (0..bits.len()).rev() {
            let char_bit = bits.as_bytes()[bits.len() - index - 1] as char;
            let int_bit = convert_char_bit_to_int(char_bit) as u32;
            result +=  int_bit * u32::pow(2, index as u32);
        };
    }
    result as usize
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_to_decimal() {
        assert_eq!(
            2,
            binary_to_decimal("10")
        );
        assert_eq!(
            10,
            binary_to_decimal("1010")
        );
        assert_eq!(
            256,
            binary_to_decimal("100000000")
        );
        let x: u32 = 256;
        assert_eq!(1, x.count_ones());
        let x: u32 = 259;
        assert_eq!(3, x.count_ones());
    }
}