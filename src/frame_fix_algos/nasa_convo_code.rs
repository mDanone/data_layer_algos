use std::collections::HashMap;

use crate::utils::{binary_to_decimal};
use crate::frame_fix_algos::steps::{StateStep};
use crate::frame_fix_algos::state_machine::BitSM;


fn encode(mut frame: String, bit_sm: &mut BitSM, states_map: &HashMap<String, HashMap<String, String>>) -> String {
    let mut result_frame = String::new();
    if frame.len() < bit_sm.get_register_size() {
        frame.push_str("0".repeat(bit_sm.get_register_size() - frame.len()).as_str())
    };

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
        |key| { vec![StateStep::new(0, key.clone())] }
    ).collect::<Vec<Vec<StateStep>>>();
    let result_frame_len = frame.len() / 2;

    for index in (0..frame.len()).step_by(2) {
        let current_transition_bits = &frame[index..index+2];
        for state_steps in &mut all_state_steps {
            let state_steps_len = state_steps.len();
            let next_key = {
                let extreme_node = &mut state_steps[state_steps_len - 1];
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

            let extreme_node_sum_hd = state_steps[state_steps_len - 1].sum_hd;
            state_steps.push(StateStep::new(extreme_node_sum_hd, next_key));
        }
    }

    let mut result_frame = String::new();
    for state_steps in all_state_steps {
        if state_steps[state_steps.len() - 1].sum_hd == 0 {
            println!("{:?}", state_steps);
            for step in &state_steps[1..result_frame_len + 1] {
                result_frame.push(step.state.as_bytes()[0] as char)
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
        let mut bit_sm = BitSM::new();
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
        let mut bit_sm = BitSM::new();
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
        let mut bit_sm = BitSM::new();
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
        let mut bit_sm = BitSM::new();
        let states_map = bit_sm.states_map();
        let encoded_frame = encode(frame, &mut bit_sm, &states_map);
        assert_eq!(
            encoded_frame,
            "111001"
        );
    }

    #[test]
    fn frame_decoded_1() {
        let frame = String::from("111001011100010010110000");
        let bit_sm = BitSM::new();
        let decoded_frame = decode(frame, &bit_sm.states_map());
        assert_eq!(
            decoded_frame,
            "111010111010"
        );
    }

    #[test]
    fn frame_decoded_2() {
        let frame = String::from("00110100101110010000011111");
        let bit_sm = BitSM::new();
        let decoded_frame = decode(frame, &bit_sm.states_map());
        assert_eq!(
            decoded_frame,
            "0101001101111"
        );
    }

    #[test]
    fn frame_decoded_3() {
        let frame = String::from("11100101111111100110110001110101011001010000001110000101000011001100110011000010100111010111010111100000011111010011110010111001010000101011001010100100001100110011000010100111010111010111011001000011000010101001000000101001111000110011011001000111101101111100001010101001110000");
        let bit_sm = BitSM::new();
        let decoded_frame = decode(frame, &bit_sm.states_map());
        assert_eq!(
            decoded_frame,
            "1110110101000011110111001010101010101001001001001101111111101001101010010101010101001001001001010101001010100100110010111110101010010111010"
        );
    }


    #[test]
    fn frame_decoded_4() {
        let frame = String::from("111001010001");
        let bit_sm = BitSM::new();
        let decoded_frame = decode(frame, &bit_sm.states_map());
        assert_eq!(
            decoded_frame,
            "111000"
        );
    }

}