const FLAG: &str = "01111110";


fn serialize_frame(mut frame: String) -> String {
    let all_five_ones: Vec<usize> = frame.match_indices("11111").map(|(i, _)| i).collect();

    let mut offset_of_five_numbers_count = 5;
    for five_one_index in all_five_ones {
        frame.insert(five_one_index + offset_of_five_numbers_count, '0');
        offset_of_five_numbers_count += 1;
    }

    format!("{FLAG}{frame}{FLAG}")
}

fn serialize_frames<'a>(frames: Vec<String>) -> Vec<String>{
    let mut result_frame = Vec::new();
    for frame in frames.to_owned() {
        result_frame.push(serialize_frame(frame));
    }
    result_frame
}

fn deserialize_frame(frame: String) -> String {
    let mut new_frame = frame.replace(FLAG, "");
    let all_five_ones: Vec<usize> = new_frame.match_indices("11111").map(|(i, _)| i).collect();
    let mut five_ones_offset_counter = 0;
    for five_one_index in all_five_ones {
        new_frame.remove(five_one_index + 5 - five_ones_offset_counter);
        five_ones_offset_counter += 1;
    }
    new_frame
}


fn deserialize_frames(frames: Vec<&str>) -> Vec<String> {
    let mut result_frame = Vec::new();
    for frame in frames.to_owned() {
        result_frame.push(deserialize_frame(frame.to_string()));
    }
    result_frame
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_deserialized() {
        let frame = String::from("0111111010100100011111010001100101001111110");

        let result_frame = deserialize_frame(frame);

        assert_eq!(
            result_frame,
            "10100100011111100011001010"
        )
    }

    #[test]
    fn frames_deserialize() {
        let frames = Vec::from([
            "0111111010100100011111010001100101001111110",
            "01111110111110111110111110111110111110101111110",
            "011111101010101010101010101010101111110"
        ]);

        let result_frames = deserialize_frames(frames);

        assert_eq!(
            result_frames,
            Vec::from(
                [
                    String::from("10100100011111100011001010"),
                    String::from("11111111111111111111111111"),
                    String::from("10101010101010101010101")
                ]
            )
        );
    }

    #[test]
    fn frame_bit_stuffed() {
        let frame = String::from("10100100011111100011001010");

        let result_frame = serialize_frame(frame);

        assert_eq!(
            result_frame,
            "0111111010100100011111010001100101001111110"
        )
    }

    #[test]
    fn frames_bit_stuffed() {
        let frames = Vec::from(
            [
                String::from("10100100011111100011001010"),
                String::from("11111111111111111111111111"),
                String::from("10101010101010101010101")
            ]
        );

        let result_frames = serialize_frames(frames);

        assert_eq!(
            result_frames,
            Vec::from([
                "0111111010100100011111010001100101001111110",
                "01111110111110111110111110111110111110101111110",
                "011111101010101010101010101010101111110"
            ])
        );
    }
}
