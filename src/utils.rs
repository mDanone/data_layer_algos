pub fn power_of_two(number: usize) -> bool{
    (number != 0) && ((number & (number - 1)) == 0)
}


pub fn convert_char_bit_to_int(bit: char) -> u32 {
    bit as u32 - '0' as u32
}
