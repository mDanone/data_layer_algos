use std::collections::{VecDeque, HashMap};


struct BitSM {
    registers: VecDeque<char>,
}

impl BitSM {

    fn build_empty_states_map(&self) -> HashMap<String, HashMap<String, String>> {
        let registers_len = self.registers.len();
        let mut states_map = HashMap::new();
        for val in 0..usize::pow(2, (registers_len - 1) as u32)  {
            let mut binary_number = format!("{:b}", val);
            binary_number =
                "0".repeat(registers_len - binary_number.len() - 1)
                    + &binary_number.clone();
            states_map.insert(binary_number, HashMap::new());
        }
        states_map
    }

    fn states_map(&self) -> HashMap<String, HashMap<String, String>> {
        let mut empty_states_map = self.build_empty_states_map();
        for (key, val) in &mut empty_states_map {
            let zero_transition = format!("{}{}", "0", key);
            let (first_control_bit, second_control_bit) = self.get_next_control_bits(&zero_transition);
            val.insert(zero_transition[..key.len()].to_string(), format!("{first_control_bit}{second_control_bit}"));

            let one_transition = format!("{}{}", "1", key);
            let (first_control_bit, second_control_bit) = self.get_next_control_bits(&one_transition);
            val.insert(one_transition[..key.len()].to_string(), format!("{first_control_bit}{second_control_bit}"));
        }
        empty_states_map
    }


    fn get_first_control_bit(&self, state: &String) -> u32 {
        let index_sequence = [1, 2, 4, 5];
        self.sum_control_bits(index_sequence, &state)
    }

    fn sum_control_bits(&self, indexes: [usize; 4], state: &String) -> u32{
        let mut transition= 0;
        for index in indexes {
            transition ^= state.as_bytes()[index] as u32;
        }
        transition
    }

    fn get_second_control_bit(&self, state: &String) -> u32 {
        let index_sequence = [0, 1, 3, 5];
        self.sum_control_bits(index_sequence, state)
    }

    fn get_next_control_bits(&self, state: &String) -> (u32, u32){
        let (first_bit, second_bit) = (
            self.get_first_control_bit(&state),
            self.get_second_control_bit(&state)
        );

        (first_bit, second_bit)
    }

    fn get_register_size(&self) -> usize {self.registers.len()}
}


// fn encode(mut frame: String, bit_sm: &mut BitSM) -> String {
//     let mut result_frame = String::new();
//
//     frame.push_str("0".repeat(bit_sm.get_register_size()).as_str());
//     while !frame.is_empty() {
//         // let (first_control_bit, second_control_bit) = bit_sm.get_next_control_bits(frame.remove(0));
//         result_frame.push_str(format!("{first_control_bit}{second_control_bit}").as_str());
//     }
//     result_frame
// }

fn decode(frame: String) -> String {
    // let output_frame_len = frame.len() / 2;
    frame
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn frame_encoded_1() {
    //     let frame = String::from("111");
    //     let registers = VecDeque::from(['0', '0', '0', '0', '0', '0']);
    //     let mut bit_sm = BitSM {registers};
    //     let encoded_frame = encode(frame, &mut bit_sm);
    //     assert_eq!(
    //         encoded_frame,
    //         "111001000001000111"
    //     );
    //     assert_eq!(
    //         bit_sm.registers,
    //         VecDeque::from(['0', '0', '0', '0', '0', '0'])
    //     );
    // }

    #[test]
    fn frame_decoded_1() {
        let frame = String::from("");
        let decoded_frame = decode(frame);
        assert_eq!(
            decoded_frame,
            ""
        );
    }

    #[test]
    fn empty_states_map_build() {
        let bit_sm = BitSM {registers: VecDeque::from(['0', '0', '0', '0', '0', '0'])};

        let states_map = bit_sm.states_map();
        println!("{:?}", states_map);
    }
}