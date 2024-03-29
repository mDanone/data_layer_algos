// Количество бит, которыми отличаются два кодовых слова, называется кодовым
// расстоянием или расстоянием между кодовыми комбинациями в смысле Хэмминга
// 10001001 и 10110001 имеют кодовое расстояние 3
// так как 10001001 ^ 10110001 = 00111000 где находится 3 единицы

// Смысл этого числа состоит в том, что если два кодовых слова находятся
// на кодовом расстоянии d, то для преобразования одного кодового слова в другое
// понадобится d ошибок в одиночных битах.

// Зная алгоритм формирования контрольных разрядов, можно построить полный
// список всех допустимых кодовых слов и в этом списке найти такую пару кодовых
// слов, кодовое расстояние между которыми будет минимальным. Это расстояние
// называется минимальным кодовым расстоянием кода, или расстоянием всего кода
// в смысле Хэмминга.


use crate::utils::power_of_two;

fn get_redundant_bit_count(frame_len: u32) -> usize {
    let mut bit_to_check = 0;
    while 2u32.pow(bit_to_check) < bit_to_check + frame_len + 1 {
        bit_to_check += 1;
    }
    bit_to_check as usize
}

fn encode(mut frame: String) -> String {
    let mut control_bit_sum: usize = 0;
    // Предварительное заполнение выходного вектора битов
    let redundant_bits = get_redundant_bit_count(frame.len() as u32);
    let mut result_vec = vec!['0';frame.len() + redundant_bits];

    // Заполнение выходного вектора битами данных
    for index in 0..result_vec.len() {
        if !power_of_two(index + 1) {
            let next_data_bit = frame.remove(0);
            if next_data_bit == '1' {
                control_bit_sum ^= index + 1;
                result_vec[index] = next_data_bit;
            };
       };
    };

    // Заполнение выходного вектора контрольными битами
    let mut x = 1;
    while x <= result_vec.len() {
        result_vec[x - 1] = if (control_bit_sum & x) > 0 { '1' } else { '0' };
        x <<= 1;
    };
    result_vec.iter().collect()
}


fn decode(frame: String) -> (String, usize) {
    let mut control_bit_sum: usize = 0;
    let mut result_frame = String::new();
    for index in 0..frame.len() {
        let next_frame_bit = frame.as_bytes()[index] as char;
        if next_frame_bit == '1' {
            control_bit_sum ^= index + 1;
        }
        if !power_of_two(index + 1){
            result_frame.push(next_frame_bit);
        }
    };

    let encoded_frame = encode(result_frame.clone());
    if power_of_two(control_bit_sum) {
        for index in 0..frame.len() {
            if power_of_two(index + 1) {
                if encoded_frame.as_bytes()[index] != frame.as_bytes()[index] {
                    return (result_frame, 0);
                }
            }
        }
    }

    if control_bit_sum > 0 {
        let mut x = 1;
        let mut control_bit_counter = 0;
        while x < control_bit_sum {
            x <<= 1;
            control_bit_counter += 1;
        }
        control_bit_sum -= control_bit_counter;
    }

    (result_frame, control_bit_sum)
}


fn decode_and_fix(frame: String) -> String {
    let (mut decoded_frame, control_bit_sum) = decode(frame);
    if control_bit_sum > 0 {
        if decoded_frame.as_bytes()[control_bit_sum - 1] == '1' as u8 {
            decoded_frame.replace_range(control_bit_sum-1..control_bit_sum, "0");
        } else {
            decoded_frame.replace_range(control_bit_sum-1..control_bit_sum, "1")
        }
    };
    decoded_frame
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn frame_encoded_1() {
        let frame = String::from("101");
        let result_frame = encode(frame);
        assert_eq!(
            String::from("101101"),
            result_frame
        );
    }

    #[test]
    fn frame_encoded_2() {
        let frame = String::from("1000001");
        let result_frame = encode(frame);
        assert_eq!(
            String::from("00100001001"),
            result_frame
        );
    }

    #[test]
    fn frame_encoded_3() {
        let frame = String::from("0100010000111101");
        let result_frame = encode(frame);
        assert_eq!(
            String::from("100110000100001011101"),
            result_frame
        );
    }

    #[test]
    fn frame_encoded_4(){
        let frame = String::from("0011111001011000");
        let result_frame_2 = encode(frame);
        assert_eq!(
            String::from("000101101110010011000"),
            result_frame_2
        )
    }

    #[test]
    fn frame_encoded_5(){
        let frame = String::from("100100101110001");
        let result_frame_2 = encode(frame);
        assert_eq!(
            String::from("11110010001011110001"),
            result_frame_2
        )
    }


    #[test]
    fn frame_decoded_1() {
        let frame = String::from("11110010001011110001");
        let (result_frame_2, control_bit_sum) = decode(frame);
        assert_eq!(control_bit_sum, 0);
        assert_eq!(
            String::from("100100101110001"),
            result_frame_2
        )
    }

    #[test]
    fn frame_decoded_2(){
        let frame = String::from("000101101110010011000");
        let (result_frame_2, control_bit_sum) = decode(frame);
        assert_eq!(control_bit_sum, 0);
        assert_eq!(
            String::from("0011111001011000"),
            result_frame_2
        )
    }

    #[test]
    fn frame_decoded_and_fixed(){
        let frame = String::from("001101101110010011000");
        let result_frame_2 = decode_and_fix(frame);
        assert_eq!(
            String::from("0011111001011000"),
            result_frame_2
        )
    }

    #[test]
    fn frame_decoded_and_fixed_2(){
        let frame = String::from("000101101110000011000");
        let result_frame_2 = decode_and_fix(frame);
        assert_eq!(
            String::from("0011111001011000"),
            result_frame_2
        )
    }

    #[test]
    fn frame_decoded_and_fixed_5() {
        let frame = String::from("000110000100001011101");
        let result_frame = decode_and_fix(frame);
        assert_eq!(
            String::from("0100010000111101"),
            result_frame
        );
    }
}