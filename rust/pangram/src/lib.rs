/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut flag: u32 = 0;
    const PANGRAM_BITMAP: u32 = !0_u32 >> 6; // 0b11111111111111111111111111, 67108863
    for c in sentence.chars() {
        if c.is_ascii() {
            let c = c.to_ascii_lowercase();
            let u = c as i8 - 'a' as i8;
            if u >= 0 && u < 26 {
                flag |= 1 << u;
            }
            if flag == PANGRAM_BITMAP {
                return true;
            }
        }
    }
    false
}
