use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Add;

use crate::frame_prepare_algos::converters::{
    binary_to_decimal,
    decimal_to_binary,
    get_max_bits_from_symbol_size,
    split_raw_byte_sequence,
    MAX_FRAME_SYMBOL_SIZE,
    get_frames_from_vectors
};


fn get_frame_check_sum(frame: &Vec<&str>) -> String {
    let max_bits = get_max_bits_from_symbol_size(MAX_FRAME_SYMBOL_SIZE);
    let frame_length = frame.len() + 1;

    let binary_checksum = decimal_to_binary(frame_length);
    let binary_checksum_size = binary_checksum.len();

    let zeros_number_to_prepend = max_bits - binary_checksum_size;
    let zeros_to_prepend = "0".repeat(zeros_number_to_prepend);

    zeros_to_prepend.add(&binary_checksum)
}


pub fn get_raw_byte_sequence_from_frame<'a>(frame: &'a mut Vec<&'a str>) -> String
{
    let frame_checksum = get_frame_check_sum(&frame);
    frame_checksum.add(frame.join("").as_str())
}



fn pop_check_sum_from_frame(bytes_vec: Rc<RefCell<Vec<&str>>>) -> usize{
    let checksum_byte = bytes_vec.borrow_mut().remove(0);
    binary_to_decimal(checksum_byte)
}

pub fn pop_frame_from_bytes_vec(bytes_vec: Rc<RefCell<Vec<&str>>>) -> Vec<&str>{
    let mut checksum = pop_check_sum_from_frame(bytes_vec.clone()) - 1; // -1 Here because we pop one element inside
    let mut frame = Vec::new();
    while checksum > 0 {
        frame.push(bytes_vec.borrow_mut().remove(0));
        checksum -= 1;
    };
    frame
}


#[cfg(test)]
mod tests {
    use crate::frame_prepare_algos::converters::get_string_from_frames;
    use super::*;

    #[test]
    fn from_bytes_to_frames() {
        let vec_of_bytes = Rc::new(
            RefCell::from(
                Vec::from(
                    [
                        "00000100", "00000101", "00001101", "00111111",
                        "00000101", "00001111", "01001101", "00000011", "00100001"
                    ]
                )
            )
        );

        let ready_frames = get_frames_from_vectors(vec_of_bytes.clone(), pop_frame_from_bytes_vec);
        assert_eq!(
            ready_frames,
            Vec::from([
                Vec::from(["00000101", "00001101", "00111111"]),
                Vec::from(["00001111", "01001101", "00000011", "00100001"])
            ])
        );
    }


    #[test]
    fn frame_popped_from_check_summed_frames_successfully() {
        let raw_bytes_vec = Rc::new(RefCell::from(Vec::from(["00000010", "10000000", "10001000"])));
        let popped_frame = pop_frame_from_bytes_vec(raw_bytes_vec.clone());
        assert_eq!(popped_frame, Vec::from(["10000000"]));
        assert_eq!(*raw_bytes_vec.borrow(), Vec::from(["10001000"]));
    }

    #[test]
    fn checksum_from_frame_success() {
        let frame = Rc::new(RefCell::from(Vec::from(["00001000"])));
        let checksum = pop_check_sum_from_frame(frame.clone());
        assert_eq!(checksum, 8);
    }

    #[test]
    fn vector_of_bytes_from_raw_sequence() {
        let mut frames = [
            Vec::from(["00000101", "00001101", "00111111"]),
            Vec::from(["00001111", "01001101", "00000011", "00100001"])
        ];

        let raw_sequence = get_string_from_frames(&mut frames, get_raw_byte_sequence_from_frame);

        let vec_of_bytes = split_raw_byte_sequence(raw_sequence.as_str());

        assert_eq!(
            vec_of_bytes,
            Vec::from(
                [
                    "00000100", "00000101", "00001101", "00111111",
                    "00000101", "00001111", "01001101", "00000011", "00100001"
                ]
            )
        );
    }

    #[test]
    fn checksum_appended() {
        let mut frame1 = Vec::from(["00000101", "00001101", "00111111"]);
        let mut frame2 = Vec::from(["00001111", "01001101", "00000011", "00100001"]);

        let raw_sequence1 = get_raw_byte_sequence_from_frame(&mut frame1);
        let raw_sequence2 = get_raw_byte_sequence_from_frame(&mut frame2);

        assert_eq!(raw_sequence1, String::from("00000100000001010000110100111111"));
        assert_eq!(raw_sequence2, String::from("0000010100001111010011010000001100100001"));
    }

    #[test]
    fn frames_to_string() {
        let mut frames = [
            Vec::from(["00000101", "00001101", "00111111"]),
            Vec::from(["00001111", "01001101", "00000011", "00100001"])
        ];

        assert_eq!(
            get_string_from_frames(&mut frames, get_raw_byte_sequence_from_frame),
            "000001000000010100001101001111110000010100001111010011010000001100100001"
        );
    }

    #[test]
    fn frame_checksum_is_valid() {
        let frame = Vec::from(["10000000", "00011000"]);
        let frame_check_sum = get_frame_check_sum(&frame);

        assert_eq!(frame_check_sum, String::from("00000011"))
    }
}
