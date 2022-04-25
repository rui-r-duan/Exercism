use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let n = input.len();
    if n < worker_count {
        let chunk = input[0..n]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        return letter_count(&chunk);
    } else {
        let lines_per_chunk = if n % worker_count == 0 {
            n / worker_count
        } else {
            n / worker_count + 1
        };
        let (tx, rx) = mpsc::channel();
        for i in 0..worker_count {
            let tx = tx.clone();
            let begin = std::cmp::min(i * lines_per_chunk, input.len());
            let end = std::cmp::min((i + 1) * lines_per_chunk, input.len());
            let chunk = input[begin..end]
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>();
            thread::spawn(move || {
                let m = letter_count(&chunk);
                tx.send(m).unwrap();
            });
        }
        let mut ans = HashMap::new();
        for _ in 0..worker_count {
            let received = rx.recv().unwrap();
            for (k, v) in received.into_iter() {
                *ans.entry(k).or_insert(0) += v;
            }
        }
        ans
    }
}

fn letter_count(lines: &[String]) -> HashMap<char, usize> {
    let mut ans = HashMap::new();
    for line in lines {
        for ch in line.chars().filter(|c| c.is_alphabetic()) {
            if let Some(c) = ch.to_lowercase().next() {
                *ans.entry(c).or_insert(0) += 1;
            }
        }
    }
    ans
}
