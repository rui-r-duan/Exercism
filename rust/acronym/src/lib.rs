pub fn abbreviate(phrase: &str) -> String {
    if phrase.len() == 0 {
        "".to_string()
    } else {
        let chars = phrase.chars().collect::<Vec<_>>();
        let mut word_beginning_positions: Vec<usize> = Vec::new();
        word_beginning_positions.push(0);
        for i in 1..chars.len() {
            let prev = chars[i - 1];
            let curr = chars[i];
            if !prev.is_alphabetic() && prev != '\'' && curr.is_alphabetic() {
                word_beginning_positions.push(i);
            } else {
                if !word_beginning_positions.contains(&(i - 1)) {
                    if prev.is_lowercase() && curr.is_uppercase() {
                        word_beginning_positions.push(i);
                    }
                }
            }
        }

        let mut words: Vec<String> = Vec::new();
        if word_beginning_positions[0] > 0 {
            words.push(chars[0..word_beginning_positions[0]].iter().collect());
        }
        for i in 0..word_beginning_positions.len() - 1 {
            let a = word_beginning_positions[i];
            let b = word_beginning_positions[i + 1];
            words.push(chars[a..b].iter().collect());
        }
        words.push(
            chars[word_beginning_positions[word_beginning_positions.len() - 1]..chars.len()]
                .iter()
                .collect(),
        );

        words
            .iter()
            .map(|w| w.chars().next().unwrap().to_uppercase().to_string())
            .collect()
    }
}
