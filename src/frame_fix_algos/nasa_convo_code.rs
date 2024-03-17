use std::cell::{RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use crate::utils::{binary_to_decimal, convert_char_bit_to_int};
use crate::frame_fix_algos::nodes::{StateStep};


struct BitSM {
    registers: String,
}

impl BitSM {
    fn build_empty_states_map(&self) -> HashMap<String, HashMap<String, String>> {
        let k = self.get_register_size() + 1;
        let mut states_map = HashMap::new();
        for val in 0..usize::pow(2, (k - 1) as u32) {
            let mut binary_number = format!("{:b}", val);
            binary_number =
                "0".repeat(k - binary_number.len() - 1)
                    + &binary_number.clone();
            states_map.insert(binary_number, HashMap::new());
        }
        states_map
    }

    fn states_map(&self) -> HashMap<String, HashMap<String, String>> {
        let mut empty_states_map = self.build_empty_states_map();
        for (key, val) in &mut empty_states_map {
            let zero_transition = format!("{}{}", "0", key);
            let (first_control_bit, second_control_bit) = self.get_next_control_bits('0', key);
            val.insert(zero_transition[..key.len()].to_string(), format!("{first_control_bit}{second_control_bit}"));

            let one_transition = format!("{}{}", "1", key);
            let (first_control_bit, second_control_bit) = self.get_next_control_bits('1', key);
            val.insert(one_transition[0..key.len()].to_string(), format!("{first_control_bit}{second_control_bit}"));
        }
        empty_states_map
    }


    fn get_first_control_bit(&self, next_bit: char, state: &String) -> usize {
        let index_sequence = vec![1, 2, 4, 5];
        self.sum_control_bits(index_sequence, next_bit, &state)
    }

    fn sum_control_bits(&self, indexes: Vec<usize>, next_bit: char, state: &String) -> usize {
        let mut transition = 0;
        transition ^= convert_char_bit_to_int(next_bit);
        for index in indexes {
            let x = convert_char_bit_to_int(state.as_bytes()[index] as char);
            transition ^= x;
        }
        transition
    }

    fn get_second_control_bit(&self, next_bit: char, state: &String) -> usize {
        let index_sequence = vec![0, 1, 2, 5];
        self.sum_control_bits(index_sequence, next_bit, state)
    }

    fn get_next_control_bits(&self, next_bit: char, state: &String) -> (usize, usize) {
        let (first_bit, second_bit) = (
            self.get_first_control_bit(next_bit, &state),
            self.get_second_control_bit(next_bit, &state)
        );

        (first_bit, second_bit)
    }

    fn get_register_size(&self) -> usize { self.registers.len() }

    fn shift_register(&mut self, next_bit: char) {
        self.registers = format!("{}{}", next_bit, &self.registers[..self.registers.len() - 1]);
    }

    fn get_current_state(&self) -> &str {
        &self.registers
    }
}


fn encode(mut frame: String, bit_sm: &mut BitSM, states_map: &HashMap<String, HashMap<String, String>>) -> String {
    let mut result_frame = String::new();
    if frame.len() < bit_sm.registers.len() {
        frame.push_str("0".repeat(bit_sm.registers.len() - frame.len()).as_str())
    }

    while !frame.is_empty() {
        let next_frame_bit = frame.remove(0);
        let current_state = bit_sm.get_current_state();
        let current_state_map = states_map.get(current_state).unwrap();
        bit_sm.shift_register(next_frame_bit);
        let next_state = bit_sm.get_current_state();
        let output_bits = current_state_map.get(next_state).unwrap();
        result_frame = format!("{}{}", result_frame, output_bits);
    }
    result_frame
}


fn decode(frame: String, states_map: &HashMap<String, HashMap<String, String>>) -> String {
    let mut all_state_steps = states_map.keys().map(
        |key| { vec![Rc::new(RefCell::new(StateStep::new(0, key.clone())))] }
    ).collect::<Vec<Vec<Rc<RefCell<StateStep>>>>>();
    let result_frame_len = frame.len() / 2;

    for index in (0..frame.len()).step_by(2) {
        let current_transition_bits = &frame[index..index+2];
        for state_step in &mut all_state_steps {
            let next_key = {
                let mut extreme_node = state_step[state_step.len() - 1].borrow_mut();
                let next_states = states_map.get(&extreme_node.state).unwrap();
                let first_key = format!("{}{}", "1", &extreme_node.state[..extreme_node.state.len() - 1]);
                let second_key= format!("{}{}", "0", &extreme_node.state[..extreme_node.state.len() - 1]);

                let first_hd = binary_to_decimal(next_states.get(&first_key).unwrap()) ^ binary_to_decimal(current_transition_bits);
                let second_hd = binary_to_decimal(next_states.get(&second_key).unwrap()) ^ binary_to_decimal(current_transition_bits);

                let first_hd_ones = first_hd.count_ones();
                let second_hd_ones = second_hd.count_ones();
                if first_hd_ones < second_hd_ones {
                    extreme_node.sum_hd += first_hd_ones as usize;
                    first_key
                } else {
                    extreme_node.sum_hd += second_hd_ones as usize;
                    second_key
                }
            };

            let extreme_node_sum_hd = state_step[state_step.len() - 1].borrow().sum_hd;
            state_step.push(Rc::new(RefCell::new(StateStep::new(extreme_node_sum_hd, next_key))));
        }
    }

    let mut result_frame = String::new();
    for state_steps in all_state_steps {
        if state_steps[state_steps.len() - 1].borrow().sum_hd == 0 {
            for step in &state_steps[1..result_frame_len + 1] {
                result_frame.push(step.borrow().state.as_bytes()[0] as char)
            }
        }
    }
    result_frame
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_encoded_1() {
        let frame = String::from("111010111010");
        let mut bit_sm = BitSM { registers: String::from("000000") };
        let states_map = bit_sm.states_map();
        let encoded_frame = encode(frame, &mut bit_sm, &states_map);
        assert_eq!(
            encoded_frame,
            "111001011100010010110000"
        );
    }

    #[test]
    fn frame_encoded_2() {
        let frame = String::from("0101001101111");
        let mut bit_sm = BitSM { registers: String::from("000000") };
        let states_map = bit_sm.states_map();
        let encoded_frame = encode(frame, &mut bit_sm, &states_map);
        assert_eq!(
            encoded_frame,
            "00110100101110010000011111"
        );
    }

    #[test]
    fn frame_encoded_3() {
        let frame = String::from("1110110101000011110111001010101010101001001001001101111111101001101010010101010101001001001001010101001010100100110010111110101010010111010");
        let mut bit_sm = BitSM { registers: String::from("000000") };
        let states_map = bit_sm.states_map();
        let encoded_frame = encode(frame, &mut bit_sm, &states_map);
        assert_eq!(
            encoded_frame,
            "11100101111111100110110001110101011001010000001110000101000011001100110011000010100111010111010111100000011111010011110010111001010000101011001010100100001100110011000010100111010111010111011001000011000010101001000000101001111000110011011001000111101101111100001010101001110000"
        );
    }

    #[test]
    fn frame_encoded_4() {
        let frame = String::from("111");
        let mut bit_sm = BitSM { registers: String::from("000000") };
        let states_map = bit_sm.states_map();
        let encoded_frame = encode(frame, &mut bit_sm, &states_map);
        assert_eq!(
            encoded_frame,
            "111001010001"
        );
    }

    #[test]
    fn frame_decoded_1() {
        let frame = String::from("111001011100010010110000");
        let bit_sm = BitSM { registers: String::from("000000") };
        let decoded_frame = decode(frame, &bit_sm.states_map());
        assert_eq!(
            decoded_frame,
            "111010111010"
        );
    }

    #[test]
    fn frame_decoded_2() {
        let frame = String::from("00110100101110010000011111");
        let bit_sm = BitSM { registers: String::from("000000") };
        let decoded_frame = decode(frame, &bit_sm.states_map());
        assert_eq!(
            decoded_frame,
            "0101001101111"
        );
    }

    #[test]
    fn frame_decoded_3() {
        let frame = String::from("11100101111111100110110001110101011001010000001110000101000011001100110011000010100111010111010111100000011111010011110010111001010000101011001010100100001100110011000010100111010111010111011001000011000010101001000000101001111000110011011001000111101101111100001010101001110000");
        let bit_sm = BitSM { registers: String::from("000000") };
        let decoded_frame = decode(frame, &bit_sm.states_map());
        assert_eq!(
            decoded_frame,
            "1110110101000011110111001010101010101001001001001101111111101001101010010101010101001001001001010101001010100100110010111110101010010111010"
        );
    }


    #[test]
    fn frame_decoded_4() {
        let frame = String::from("111001010001");
        let bit_sm = BitSM { registers: String::from("000000") };
        let decoded_frame = decode(frame, &bit_sm.states_map());
        assert_eq!(
            decoded_frame,
            "111000"
        );
    }

}