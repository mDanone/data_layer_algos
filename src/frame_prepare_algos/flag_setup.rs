use std::cell::RefCell;
use std::rc::Rc;
use crate::frame_prepare_algos::converters::{
    get_string_from_frames,
    get_frames_from_vectors
};

const FLAG: &str = "00000000";
const ESC: &str = "00000100";

fn get_frame_with_flags<'a>(frame: &'a Vec<&'a str>) -> Vec<&'a str> {
    let mut frame_with_flags = Vec::new();
    frame_with_flags.push(FLAG);
    for &byte_str in frame.iter() {
        if byte_str == FLAG || byte_str == ESC {
            frame_with_flags.push(ESC);
        }
        frame_with_flags.push(byte_str)
    };
    frame_with_flags.push(FLAG);
    frame_with_flags
}

fn get_frames_with_flags<'a>(frames: &'a [Vec<&'a str>]) -> Vec<Vec<&'a str>> {
    let mut frames_with_flags: Vec<Vec<&str>> = Vec::new();

    for frame in frames {
        let frame_with_flag = get_frame_with_flags(frame);
        frames_with_flags.push(frame_with_flag);
    };
    frames_with_flags
}

fn get_raw_byte_sequence_from_frame<'a>(frame: &'a mut Vec<&'a str>) -> String
{
    frame.join("").as_str().to_string()
}


fn get_first_cleaned_frame(bytes_vec: Rc<RefCell<Vec<&str>>>) -> Vec<&str>{
    let mut number_of_flags = 0;
    let mut final_frame: Vec<&str> = Vec::new();
    let mut esc_set = false;
    while !bytes_vec.borrow().is_empty() {
        let byte_str = bytes_vec.borrow_mut().remove(0);
        if (byte_str == ESC) & (!esc_set) {
            esc_set = true;
            continue;
        };
        if (byte_str == FLAG) & (number_of_flags < 2) & (!esc_set) {
            number_of_flags += 1;
        }
        else if number_of_flags < 2 {
            final_frame.push(byte_str)
        }
        else {
            break;
        }
        esc_set = false;
    }
    final_frame
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn from_bytes_to_frames() {
        let vec_of_bytes = Rc::new(
            RefCell::from(
                Vec::from(
                    [
                        "00000000", "00000101", "00001101", "00111111", "00000000",
                        "00000000", "00001111", "01001101", "00000011", "00100001", "00000000"
                    ]
                )
            )
        );

        let ready_frames = get_frames_from_vectors(vec_of_bytes.clone(), get_first_cleaned_frame);
        assert_eq!(
            ready_frames,
            Vec::from([
                Vec::from(["00000101", "00001101", "00111111"]),
                Vec::from(["00001111", "01001101", "00000011", "00100001"])
            ])
        );
    }

    #[test]
    fn from_bytes_to_frames_with_esc() {
        let vec_of_bytes = Rc::new(
            RefCell::from(
                Vec::from(
                    [
                        "00000000", "00000101", "00000100", "00000000", "00001101", "00111111", "00000000",
                        "00000000", "00001111", "01001101", "00000100", "00000100", "00000011", "00100001", "00000000"
                    ]
                )
            )
        );

        let ready_frames = get_frames_from_vectors(vec_of_bytes.clone(), get_first_cleaned_frame);
        assert_eq!(
            ready_frames,
            Vec::from([
                Vec::from(["00000101", "00000000", "00001101", "00111111"]),
                Vec::from(["00001111", "01001101", "00000100", "00000011", "00100001"])
            ])
        );
    }

        #[test]
    fn first_frame_cleaned_and_retrieved() {
        let mock_bytes_vec = Rc::new(
            RefCell::from(
                Vec::from([
                    FLAG, "01010101", "10000000", FLAG
                ])
            )
        );

        let final_frame = get_first_cleaned_frame(mock_bytes_vec.clone());

        assert_eq!(final_frame, Vec::from(["01010101", "10000000"]));

        assert_eq!(mock_bytes_vec.clone(), Rc::new(RefCell::from(Vec::new())));
    }

    #[test]
    fn flag_set_to_frame() {
        let frame = Vec::from(
            ["10001000", "10010010"]
        );

        let frame_with_flags = get_frame_with_flags(&frame);

        assert_eq!(frame_with_flags, Vec::from([FLAG, "10001000", "10010010", FLAG]))
    }

    #[test]
    fn flag_set_to_frame_with_esc() {
        let frame = Vec::from(
            ["10001000", "00000000", "00000000", "00000100", "10010010"]
        );

        let frame_with_flags = get_frame_with_flags(&frame);

        assert_eq!(
            frame_with_flags,
            Vec::from(
                [
                    FLAG,
                    "10001000",
                    ESC,
                    "00000000",
                    ESC,
                    "00000000",
                    ESC,
                    "00000100",
                    "10010010",
                    FLAG
            ]
            )
        )
    }

    #[test]
    fn frames_converted_to_raw_byte_string() {
        let frames = [
            Vec::from(["10001000", "10010010"]),
            Vec::from(["10101010", "00000010"]),
            Vec::from(["00010001", "01101010"])
        ];

        let mut frames_with_flags = get_frames_with_flags(&frames);
        let raw_byte_string = get_string_from_frames(&mut frames_with_flags, get_raw_byte_sequence_from_frame);

        assert_eq!(
            raw_byte_string,
            "000000001000100010010010000000000000000010101010000000100000000000000000000100010110101000000000"
        )
    }

    #[test]
    fn setup_frames_with_flags() {
        let frames = [
            Vec::from(["10001000", "10000010"]),
            Vec::from(["10101010", "00000010"]),
            Vec::from(["00010001", "01101010"])
        ];

        let frames_with_flags = get_frames_with_flags(&frames);

        assert_eq!(
            frames_with_flags,
            [
                Vec::from([FLAG, "10001000", "10010010", FLAG]),
                Vec::from([FLAG, "10101010", "00000010", FLAG]),
                Vec::from([FLAG, "00010001", "01101010", FLAG])
            ]
        )
    }
}