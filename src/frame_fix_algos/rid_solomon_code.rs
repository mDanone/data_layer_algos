fn encode(frame: String) -> String {
    frame
}

fn decode(frame: String) -> String {
    frame
}


#[cfg(test)]
mod tests {
    use super::{encode, decode};

    #[test]
    fn encode_1() {
        let frame = String::from("");
        assert_eq!(
            encode(frame),
            ""
        )
    }

    #[test]
    fn decode_1() {
        let frame = String::from("");
        assert_eq!(
            decode(frame),
            ""
        )
    }
}