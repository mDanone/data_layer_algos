const FLAG: &str = "01111110";


fn setup_frame(mut frame: String) -> String {
    let all_five_ones: Vec<usize> = frame.match_indices("11111").map(|(i, _)| i).collect();

    let mut offset_of_five_numbers_count = 5;
    for five_one_index in all_five_ones {
        frame.insert(five_one_index + offset_of_five_numbers_count, '0');
        offset_of_five_numbers_count += 1;
    }

    format!("{FLAG}{frame}{FLAG}")
}

fn setup_frames<'a>(frames: Vec<String>) -> Vec<String>{
    let mut result_frame = Vec::new();
    for frame in frames.to_owned() {
        result_frame.push(setup_frame(frame));
    }
    result_frame
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_bit_stuffed() {
        let frame = String::from("10100100011111100011001010");

        let result_frame = setup_frame(frame);

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

        let result_frames = setup_frames(frames);

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
