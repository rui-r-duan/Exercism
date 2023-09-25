pub fn encode(source: &str) -> String {
    let v = source
        .chars()
        .fold(Vec::<(char, i32)>::new(), |mut accm, x| {
            let n = accm.len();
            if n > 0 && accm[n - 1].0 == x {
                accm[n - 1].1 += 1;
            } else {
                accm.push((x, 1));
            }
            accm
        })
        .iter()
        .map(|&(c, i)| {
            if i == 1 {
                format!("{}", c)
            } else {
                format!("{}{}", i, c)
            }
        })
        .collect::<Vec<String>>();

    v.join("")
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
