use std::ops::Add;

const MAX_FRAME_SYMBOL_SIZE: usize = 256;

/// ## Input example:
/// decimal_to_binary(3)
/// ## Output example
/// String("00000011")
pub fn decimal_to_binary(decimal_number: usize) -> String {
    format!("{:b}", decimal_number)
}


fn get_frame_check_sum(frame: &Vec<&str>) -> String {
    let max_bits = MAX_FRAME_SYMBOL_SIZE.ilog2() as usize;
    let frame_length = frame.len() + 1;

    let binary_checksum = decimal_to_binary(frame_length);
    let binary_checksum_size = binary_checksum.len();

    let zeros_number_to_prepend = max_bits - binary_checksum_size;
    let zeros_to_prepend = "0".repeat(zeros_number_to_prepend);

    zeros_to_prepend.add(&binary_checksum)
}


/// Make raw String of ones and zeros with checksum from fram (Vec<&str>)
fn get_raw_byte_sequence_from_frame<'a>(frame: &'a mut Vec<&'a str>) -> String{
    let frame_checksum = get_frame_check_sum(&frame);
    frame_checksum.add(frame.join("").as_str())
}

fn get_string_from_frames<'a>(frames: &'a mut [Vec<&'a str>]) -> String {
    let mut final_string = String::new();
    for frame in frames.iter_mut() {
        final_string = final_string.add(get_raw_byte_sequence_from_frame(frame).as_str());
    }
    final_string
}


// fn split_raw_byte_sequence()



#[cfg(test)]
mod tests {
    use super::*;

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
            get_string_from_frames(&mut frames),
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