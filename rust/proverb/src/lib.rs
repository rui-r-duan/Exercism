use std::iter;

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    list.windows(2)
        .map(|pair| format!("For want of a {} the {} was lost.\n", pair[0], pair[1]))
        .chain(iter::once(format!(
            "And all for the want of a {}.",
            list[0]
        )))
        .collect()
}
