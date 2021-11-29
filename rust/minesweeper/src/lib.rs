const NEIBOURHOOD_OFFSETS: [(i32, i32); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let rows = minefield.len() as i32;
    (0..rows)
        .map(|x| {
            let cols = minefield[x as usize].len() as i32;
            (0..cols)
                .map(|y| {
                    let cell = minefield[x as usize].as_bytes()[y as usize];
                    match cell {
                        b'*' => '*',
                        _ => {
                            let count = NEIBOURHOOD_OFFSETS
                                .iter()
                                .map(|&(dx, dy)| (x + dx, y + dy))
                                .filter(|&(r, c)| (0 <= r && r < rows) && (0 <= c && c < cols))
                                .filter(|&(x, y)| {
                                    minefield[x as usize].as_bytes()[y as usize] == b'*'
                                })
                                .count();
                            match count {
                                0 => ' ',
                                n => (n as u8 + '0' as u8) as char,
                            }
                        }
                    }
                })
                .collect()
        })
        .collect()
}
