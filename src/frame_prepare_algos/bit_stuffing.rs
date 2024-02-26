const FLAG: &str = "01111110";


fn setup_frame(mut frame: String) -> String {
    let all_five_ones: Vec<usize> = frame.match_indices("11111").map(|(i, _)|i).collect();
    for five_one_index in all_five_ones {
        frame.insert(five_one_index + 5, '0');
    }

    format!("{FLAG}{frame}{FLAG}")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_bit_stuffed() {
        let frame = String::from("10100100011111100011001010");

        let result_frames = setup_frame(frame);

        assert_eq!(
            result_frames,
            "0111111010100100011111010001100101001111110"
        )
    }
}
