/// Determine whether a sentence is a pangram.
// pub fn is_pangram(sentence: &str) -> bool {
//     let mut flag: u32 = 0;
//     const PANGRAM_BITMAP: u32 = !0_u32 >> 6; // 0b11111111111111111111111111, 67108863
//     for c in sentence.chars() {
//         if c.is_ascii() {
//             let c = c.to_ascii_lowercase();
//             let u = c as i16 - 'a' as i16;
//             if u >= 0 && u < 26 {
//                 flag |= 1 << u;
//             }
//             if flag == PANGRAM_BITMAP {
//                 return true;
//             }
//         }
//     }
//     false
// }
pub fn is_pangram(sentence: &str) -> bool {
    let mut flag: u32 = 0;
    const PANGRAM_BITMAP: u32 = !0_u32 >> 6; // 0b11111111111111111111111111, 67108863, (1<<26)-1
    for c in sentence.bytes() {
        let c = c.to_ascii_lowercase();
        if c < b'a' || c > b'z' {
            continue;
        }
        let u = c as i16 - b'a' as i16;
        flag |= 1 << u;
        if flag == PANGRAM_BITMAP {
            return true;
        }
    }

    false
}
