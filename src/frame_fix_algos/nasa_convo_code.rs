use std::cell::{RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use crate::utils::convert_char_bit_to_int;
use crate::frame_fix_algos::nodes::{Tree, TreeNode};


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
            val.insert(one_transition[..key.len()].to_string(), format!("{first_control_bit}{second_control_bit}"));
        }
        empty_states_map
    }


    fn get_first_control_bit(&self, next_bit: char, state: &String) -> u32 {
        let index_sequence = [1, 2, 4, 5];
        self.sum_control_bits(index_sequence, next_bit, &state)
    }

    fn sum_control_bits(&self, indexes: [usize; 4], next_bit: char, state: &String) -> u32 {
        let mut transition = 0;
        transition ^= convert_char_bit_to_int(next_bit);
        for index in indexes {
            let x = convert_char_bit_to_int(state.as_bytes()[index] as char);
            transition ^= x;
        }
        transition
    }

    fn get_second_control_bit(&self, next_bit: char, state: &String) -> u32 {
        let index_sequence = [0, 1, 2, 5];
        self.sum_control_bits(index_sequence, next_bit, state)
    }

    fn get_next_control_bits(&self, next_bit: char, state: &String) -> (u32, u32) {
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
    println!("{:?}", states_map);

    while !frame.is_empty() {
        let next_frame_bit = frame.remove(0);
        let current_state = bit_sm.get_current_state();
        let current_state_map = states_map.get(current_state).unwrap();
        bit_sm.shift_register(next_frame_bit);
        let next_state = bit_sm.get_current_state();
        let output_bits = current_state_map.get(next_state).unwrap();
        result_frame = format!("{}{}", output_bits, result_frame);
    }
    result_frame
}


fn decode(frame: String, states_map: &HashMap<String, HashMap<String, String>>) -> String {
    let trees = states_map.keys().map(
        |key| { Tree::new(Rc::new(RefCell::new(TreeNode::new(0, key.clone())))) }
    ).collect::<Vec<Tree>>();

    for index in (0..frame.len()).step_by(2) {
        let _current_transition_bits = &frame[index..index+2];
        for (key, next_states) in states_map {
            for tree in &trees {
                let extreme_node = tree.get_extreme_node().clone();

                if let Some(_transition_bits) = next_states.get(&extreme_node.borrow().state) {
                    extreme_node.borrow_mut().sum_hd = 0; // TODO: Need to add transition_bits ^ current_transition_bits
                    if extreme_node.borrow().left.is_none() {
                        extreme_node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(extreme_node.borrow().sum_hd, key.clone()))));
                    }
                    if extreme_node.borrow().right.is_none() {
                        extreme_node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(extreme_node.borrow().sum_hd, key.clone()))))
                    }
                }

                if let (
                    Some(left_node),
                    Some(right_node)
                ) = (extreme_node.borrow().left.clone(), extreme_node.borrow().right.clone()){
                    let left_borrowed = left_node.borrow();
                    let right_borrowed = right_node.borrow();
                    if left_borrowed.sum_hd < right_borrowed.sum_hd {
                        extreme_node.borrow_mut().right = None;
                    } else {
                        extreme_node.borrow_mut().left = None;
                    }
                };

            }

        }
    }
    // for node in all_nodes {
    //     for (key, value) in
    // }
    frame
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_encoded_1() {
        let frame = String::from("111");
        let mut bit_sm = BitSM { registers: String::from("000000") };
        let states_map = bit_sm.states_map();
        let encoded_frame = encode(frame, &mut bit_sm, &states_map);
        println!("{}", encoded_frame);
        assert_eq!(
            encoded_frame,
            "011011"
        );
    }

    #[test]
    fn frame_decoded_1() {
        let frame = String::from("011011");
        let bit_sm = BitSM { registers: String::from("000000") };
        let decoded_frame = decode(frame, &bit_sm.states_map());
        assert_eq!(
            decoded_frame,
            "111"
        );
    }

    #[test]
    fn empty_states_map_build() {
        let bit_sm = BitSM { registers: String::from("000000") };

        let states_map = bit_sm.states_map();
        println!("{:?}", states_map);
    }
}