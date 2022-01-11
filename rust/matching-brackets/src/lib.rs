pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in string.chars() {
        if c == '{' || c == '[' || c == '(' {
            stack.push(c);
        } else if c == '}' || c == ']' || c == ')' {
            if stack.is_empty() {
                return false;
            } else {
                let &p = stack.last().unwrap();
                if p == '{' && c == '}' || p == '[' && c == ']' || p == '(' && c == ')' {
                    stack.pop();
                } else {
                    return false;
                }
            }
        } else {
            continue;
        }
    }

    stack.is_empty()
}
