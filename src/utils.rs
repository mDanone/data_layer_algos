pub fn number_odd(number: usize) -> bool{
    (number != 0) && ((number & (number - 1)) == 0)
}
