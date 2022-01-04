fn is_yelling(clean_msg: &str) -> bool {
    let contains_letter = clean_msg.contains(char::is_alphabetic);
    let capitalized = clean_msg.to_uppercase() == clean_msg;

    contains_letter && capitalized
}

pub fn reply(message: &str) -> &str {
    let clean_msg = message.trim();
    let ends_with_question_mark = clean_msg.ends_with('?');
    let yelling = is_yelling(clean_msg);
    let empty_msg = clean_msg.len() == 0;
    if empty_msg {
        return "Fine. Be that way!";
    } else if ends_with_question_mark {
        if yelling {
            // yell a question
            return "Calm down, I know what I'm doing!";
        } else {
            // normal question
            return "Sure.";
        }
    } else if yelling {
        return "Whoa, chill out!";
    } else {
        return "Whatever.";
    }
}
