use std::str;

// Assume only ASCII non-digit characters are in the source string.
pub fn encode(source: &str) -> String {
    let n = source.len();
    if n == 0 || n == 1 {
        return String::from(source);
    }

    let mut ans: Vec<u8> = Vec::with_capacity(n);
    let src = source.as_bytes();
    let mut i = 1;
    let mut count = 1;
    while i < n {
        if src[i] == src[i - 1] {
            count += 1;
        } else {
            if count == 1 {
                ans.push(src[i - 1]);
            } else {
                vec_write_num(&mut ans, count);
                ans.push(src[i - 1]);
                count = 1;
            }
        }
        i += 1;
    }

    // Now i == n, process the last char src[i - 1].
    if count == 1 {
        ans.push(src[i - 1]);
    } else {
        vec_write_num(&mut ans, count);
        ans.push(src[i - 1]);
    }

    let s = ans.as_slice();
    unsafe { str::from_utf8_unchecked(s).to_string() }
}

fn vec_write_num(v: &mut Vec<u8>, n: usize) {
    let mut n = n;
    let old_len = v.len();
    let mut q = n / 10;
    let mut r = (n % 10) as u8;
    while q != 0 {
        v.push(b'0' + r);
        n = q;
        q = n / 10;
        r = (n % 10) as u8;
    }
    v.push(b'0' + r);
    let new_len = v.len();
    let digit_count = new_len - old_len;
    for i in 0..digit_count / 2 {
        let tmp = v[old_len + i];
        v[old_len + i] = v[new_len - i - 1];
        v[new_len - i - 1] = tmp;
    }
}

pub fn decode(source: &str) -> String {
    let chars = source.chars().collect::<Vec<char>>();
    let mut v: Vec<(char, i32)> = Vec::new();
    let mut begin: usize = 0;
    for i in 0..chars.len() {
        if !chars[i].is_ascii_digit() {
            if i == 0 || begin == i {
                v.push((chars[i], 1));
            } else {
                let n: i32 = chars[begin..i].iter().collect::<String>().parse().unwrap();
                v.push((chars[i], n));
            }
            begin = i + 1;
        }
    }

    let n = v
        .iter()
        .fold(0, |accm: usize, &(_c, i)| accm + (i as usize));
    let mut ans: String = String::with_capacity(n);
    let mut j: usize = 0; // index of chars in ans string
    for &(c, i) in v.iter() {
        for _k in 0..i {
            ans.insert(j, c);
            j += 1;
        }
    }

    ans
}
