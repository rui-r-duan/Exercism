use unicode_segmentation::UnicodeSegmentation;

// pub fn reverse(input: &str) -> String {
//     let grapheme_clusters = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
//     let mut output = String::new();

//     for c in grapheme_clusters.into_iter().rev() {
//         output.push_str(c);
//     }

//     output
// }

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
}
