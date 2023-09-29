// Assume source only contains ASCII non-digit characters.
pub fn encode(source: &str) -> String {
    let n = source.len();
    if n == 0 || n == 1 {
        return String::from(source);
    }

    let mut ans: Vec<u8> = Vec::with_capacity(n);
    let src = source.as_bytes();
    let mut i = 1; // peek position
    let mut count = 1; // how many unencoded chars before peek position
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

    unsafe { String::from_utf8_unchecked(ans) }
}

fn vec_write_num(v: &mut Vec<u8>, n: usize) {
    let mut n = n;
    let old_len = v.len();
    while n != 0 {
        let r = (n % 10) as u8;
        v.push(b'0' + r);
        n = n / 10;
    }
    let new_len = v.len();
    let digit_count = new_len - old_len;
    for i in 0..digit_count / 2 {
        let tmp = v[old_len + i];
        v[old_len + i] = v[new_len - i - 1];
        v[new_len - i - 1] = tmp;
    }
}

// Assume source only contains ASCII characters.
pub fn decode(source: &str) -> String {
    let chars = source.as_bytes();
    let mut v: Vec<(u8, i32)> = Vec::new();
    let mut begin: usize = 0;
    for i in 0..chars.len() {
        if !chars[i].is_ascii_digit() {
            if i == 0 || begin == i {
                v.push((chars[i], 1));
            } else {
                let n: i32 = parse_positive_int(&chars[begin..i]);
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
            ans.insert(j, c as char);
            j += 1;
        }
    }

    ans
}

fn parse_positive_int(digits: &[u8]) -> i32 {
    let mut ans = 0;
    for i in 0..digits.len() {
        ans = ans * 10 + (digits[i] - b'0') as i32;
    }

    ans
}
