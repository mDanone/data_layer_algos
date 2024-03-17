use std::collections::HashMap;
use crate::utils::convert_char_bit_to_int;

pub struct BitSM {
    registers: String,
}

impl BitSM {

    pub fn new() -> Self { BitSM { registers: String::from("000000") } }

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

    pub fn states_map(&self) -> HashMap<String, HashMap<String, String>> {
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

    pub fn get_register_size(&self) -> usize { self.registers.len() }

    pub fn shift_register(&mut self, next_bit: char) {
        self.registers = format!("{}{}", next_bit, &self.registers[..self.registers.len() - 1]);
    }

    pub fn get_current_state(&self) -> &str {
        &self.registers
    }
}
