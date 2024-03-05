fn encode(frame: String) -> String {frame}
fn decode(frame: String) -> String {frame}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_encoded_1() {
        let frame = String::from("");
        let encoded_frame = encode(frame);
        assert_eq!(
            encoded_frame,
            ""
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