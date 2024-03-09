use std::collections::VecDeque;

use crate::utils::convert_char_bit_to_int;


struct BitSM {
    registers: VecDeque<char>,
    current_state: u32
}

impl BitSM {
    fn get_first_control_bit(&mut self, next_bit: char) -> u32 {
        let index_sequence = [1, 2, 4, 5];
        self.current_state = convert_char_bit_to_int(next_bit);
        self.sum_control_bits(index_sequence)
    }

    fn sum_control_bits(&mut self, indexes: [usize; 4]) -> u32{
        for index in indexes {
            self.current_state ^= convert_char_bit_to_int(self.registers[index]);
        }
        self.current_state
    }

    fn get_second_control_bit(&mut self, next_bit: char) -> u32 {
        let index_sequence = [0, 1, 3, 5];
        self.current_state = convert_char_bit_to_int(next_bit);
        self.sum_control_bits(index_sequence)
    }

    fn get_next_control_bits(&mut self, next_data_bit: char) -> (u32, u32){
        let (first_bit, second_bit) = (
            self.get_first_control_bit(next_data_bit),
            self.get_second_control_bit(next_data_bit)
        );
        self.registers.pop_back();
        self.registers.push_front(next_data_bit);

        (first_bit, second_bit)
    }

    fn get_register_size(&self) -> usize {self.registers.len()}
}



fn encode(mut frame: String, bit_sm: &mut BitSM) -> String {
    let mut result_frame = String::new();

    frame.push_str("0".repeat(bit_sm.get_register_size()).as_str());
    while !frame.is_empty() {
        let (first_control_bit, second_control_bit) = bit_sm.get_next_control_bits(frame.remove(0));
        result_frame.push_str(format!("{first_control_bit}{second_control_bit}").as_str());
    }
    result_frame
}
fn decode(frame: String) -> String {
    frame
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_encoded_1() {
        let frame = String::from("111");
        let registers = VecDeque::from(['0', '0', '0', '0', '0', '0']);
        let mut bit_sm = BitSM {registers, current_state: 0};
        let encoded_frame = encode(frame, &mut bit_sm);
        assert_eq!(
            encoded_frame,
            "111001000001000111"
        );
        assert_eq!(
            bit_sm.registers,
            VecDeque::from(['0', '0', '0', '0', '0', '0'])
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