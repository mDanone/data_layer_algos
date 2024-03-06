use std::collections::VecDeque;

use crate::utils::convert_char_bit_to_int;

fn encode(mut frame: String, registers: &mut VecDeque<char>) -> String {
    let mut result_frame = String::new();

    while !frame.is_empty() {
        if let Some(next_bit) = frame.pop() {
            let (first_control_bit, second_control_bit) = {
                (
                    convert_char_bit_to_int(next_bit)
                        ^ convert_char_bit_to_int(registers[1])
                        ^ convert_char_bit_to_int(registers[2])
                        ^ convert_char_bit_to_int(registers[4])
                        ^ convert_char_bit_to_int(registers[5]),
                    convert_char_bit_to_int(next_bit)
                        ^ convert_char_bit_to_int(registers[0])
                        ^ convert_char_bit_to_int(registers[1])
                        ^ convert_char_bit_to_int(registers[3])
                        ^ convert_char_bit_to_int(registers[5]),

               )
            };
            registers.pop_back();
            registers.push_front(next_bit);
            result_frame.push_str(format!("{first_control_bit}{second_control_bit}").as_str());
        }
    }
    result_frame
}
fn decode(frame: String) -> String {frame}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_encoded_1() {
        let frame = String::from("111");
        let mut registers = VecDeque::from(['0', '0', '0', '0', '0', '0']);
        let encoded_frame = encode(frame, &mut registers);
        assert_eq!(
            encoded_frame,
            "111001"
        );
        assert_eq!(
            registers,
            VecDeque::from(['1', '1', '1', '0', '0', '0'])
        );
    }

    #[test]
    fn frame_decoded_1() {
        let frame = String::from("");
        let decoded_frame = decode(frame);
        assert_eq!(
            decoded_frame,
            ""
        );
    }
}