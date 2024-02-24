use std::cell::RefCell;
use std::ops::Add;
use std::rc::Rc;


pub const MAX_FRAME_SYMBOL_SIZE: usize = 256;

/// Get number of bits depending on max decimal size
/// ```
/// use data_link_layer_algos::frame_prepare_algos::converters::get_max_bits_from_symbol_size;
/// let input_value = 256;
/// assert_eq!(get_max_bits_from_symbol_size(input_value), 8);
/// ```
pub fn get_max_bits_from_symbol_size(symbol_size: usize) -> usize { symbol_size.ilog2()  as usize}


/// Convert decimal usize number to binary number string
/// ```
/// use data_link_layer_algos::frame_prepare_algos::converters::decimal_to_binary;
/// let input_value = 10;
/// assert_eq!(decimal_to_binary(input_value), String::from("1010"));
/// ```
pub fn decimal_to_binary(decimal_number: usize) -> String {
    format!("{:b}", decimal_number)
}


/// Convert binary string number to decimal usize number
/// ```
/// use data_link_layer_algos::frame_prepare_algos::converters::binary_to_decimal;
/// let input_value = "00001010";
/// assert_eq!(binary_to_decimal(input_value), 10);
/// ```
pub fn binary_to_decimal(binary_string: &str) -> usize
{
    let binary_number = usize::from_str_radix(binary_string, 2);
    match binary_number {
        Ok(val) => val,
        Err(error) => panic!("Value is not binary: {}", error)
    }
}


pub fn get_string_from_frames<'a, 'b, F>(frames: &'a mut [Vec<&'a str>], executable: F) -> String
    where F: Fn(&'a mut Vec<&'a str>) -> String
{
    let mut final_string = String::new();
    for frame in frames.iter_mut() {
        final_string = final_string
            .add(executable(frame).as_str());
    }
    final_string
}


pub fn get_frames_from_vectors<F>(bytes_vec: Rc<RefCell<Vec<&str>>>, executable: F) -> Vec<Vec<&str>>
    where F: Fn(Rc<RefCell<Vec<&str>>>) -> Vec<&str>
{
    let mut frames = Vec::new();
    while !bytes_vec.clone().borrow().is_empty() {
        let frame = executable(bytes_vec.clone());
        frames.push(frame);
    };
    frames
}


pub fn split_raw_byte_sequence(raw_byte_sequence: &str) -> Vec<String>{
    let max_bits = get_max_bits_from_symbol_size(MAX_FRAME_SYMBOL_SIZE);
    raw_byte_sequence.chars()
        .collect::<Vec<char>>()
        .chunks(max_bits)
        .map(|chunk| chunk.iter().collect())
        .collect()
}
