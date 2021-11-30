/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let clean_code = code.chars().filter(|&c| c != ' ').rev().collect::<String>();
    if clean_code.chars().any(|x| !x.is_digit(10)) {
        return false;
    }
    let digit_bytes = clean_code.as_bytes();
    let n = digit_bytes.len();
    if n == 1 {
        return false;
    }
    let checksum = (0..n)
        .map(|i| {
            if i % 2 == 1 {
                let double_value = (digit_bytes[i] - '0' as u8) * 2;
                let double_value_str = double_value.to_string();
                let new_bytes = double_value_str.as_bytes();
                let new_value = new_bytes.iter().fold(0, |accm, &d| accm + (d - '0' as u8));
                new_value + '0' as u8
            } else {
                digit_bytes[i]
            }
        })
        .fold(0, |accum, v| accum + (v - '0' as u8));

    checksum % 10 == 0
}
