pub fn build_proverb(list: &[&str]) -> String {
    let mut result: String = String::new();
    let length: usize = list.len();

    if length == 0 {
        return result;
    }

    let mut i: usize = 0;
    while i < length - 1 {
        let a = list[i];
        let b = list[i + 1];
        result.push_str(&format!("For want of a {} the {} was lost.\n", a, b));
        i += 1;
    }
    result.push_str(&format!("And all for the want of a {}.", list[0]));

    result
}
