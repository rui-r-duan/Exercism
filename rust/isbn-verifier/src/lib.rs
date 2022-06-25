/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut digits = [0u8; 11];
    let mut pos = 11;
    for c in isbn.chars() {
        if pos > 2 {
            if c.is_ascii_digit() {
                pos -= 1;
                digits[pos] = (c as u8) - b'0';
            } else if c != '-' {
                return false;
            }
        } else if pos == 2 {
            if c.is_ascii_digit() {
                pos -= 1;
                digits[pos] = (c as u8) - b'0';
            } else if c == 'X' {
                pos -= 1;
                digits[pos] = 10;
            } else if c == '-' {
                continue;
            } else {
                return false;
            }
        } else if pos == 1 {
            return false; // isbn string is too long
        }
    }
    if pos != 1 {
        // isbn string is too short
        return false;
    }
    let mut sum = 0u32;
    for (i, &v) in digits.iter().enumerate() {
        sum += (i as u32) * (v as u32);
    }
    sum % 11 == 0
}
