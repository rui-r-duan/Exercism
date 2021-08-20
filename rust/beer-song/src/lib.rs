pub fn verse(n: u32) -> String {
    match n {
        0 => String::from(
            "No more bottles of beer on the wall, no more bottles of beer.\n\
             Go to the store and buy some more, 99 bottles of beer on the wall.\n",
        ),
        1 => String::from(
            "1 bottle of beer on the wall, 1 bottle of beer.\n\
             Take it down and pass it around, no more bottles of beer on the wall.\n",
        ),
        2 => String::from(
            "2 bottles of beer on the wall, 2 bottles of beer.\n\
             Take one down and pass it around, 1 bottle of beer on the wall.\n",
        ),
        _ => format!(
            "{0} bottles of beer on the wall, {0} bottles of beer.\n\
             Take one down and pass it around, {1} bottles of beer on the wall.\n",
            n,
            n - 1
        ),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let verses: Vec<String>;
    if start >= end {
        let r = (end..start + 1).rev();
        verses = r.map(verse).collect();
    } else {
        let r = start..end + 1;
        verses = r.map(verse).collect();
    }

    verses.join("\n")
}
