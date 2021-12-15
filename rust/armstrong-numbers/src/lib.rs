pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 {
        return true;
    }
    let mut digits: Vec<u32> = vec![];
    let mut q = num;
    while q > 0 {
        digits.push(q % 10);
        q = q / 10;
    }
    let n = digits.len() as u32;
    let sum = digits.iter().map(|&d| d.pow(n)).sum::<u32>();

    sum == num
}
