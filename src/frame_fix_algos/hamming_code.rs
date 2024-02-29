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


use crate::utils::number_odd;

fn get_redundant_bit_count(frame_len: u32) -> u32 {
    let mut bit_to_check = 0;
    while 2u32.pow(bit_to_check) < bit_to_check + frame_len + 1 {
        bit_to_check += 1;
    }
    bit_to_check
}
fn encode_frame(mut frame: String) -> String {
    let mut result_frame = String::new();
    let redundant_bits = get_redundant_bit_count(frame.len() as u32);
    let mut result_array = vec![' ';frame.len() + redundant_bits as usize];

    result_array = result_array.iter().enumerate().map(|(index, _)| {
        if number_odd(index + 1) { '0' } else {frame.remove(0)}
    }).collect();

    result_frame
}


#[cfg(test)]
mod tests {
    use crate::frame_fix_algos::hamming_code::encode_frame;

    #[test]
    fn frame_encoded() {
        let frame = String::from("1000001");
        let result_frame = encode_frame(frame);
        assert_eq!(
            String::from(""),
            result_frame
        )


    }
}
