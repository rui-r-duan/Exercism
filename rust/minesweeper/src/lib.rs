use std::slice::Iter;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let m = minefield.len(); // rows
    if m == 0 {
        return Vec::new();
    }

    let n = minefield[0].len(); // columns
    if n == 0 {
        return vec!["".to_string()];
    }

    let mut board = build_board(minefield);
    for i in 0..m {
        for j in 0..n {
            if board[i][j] == ' ' {
                let v = get_value(&board, i as i16, j as i16);
                if v > 0 {
                    // assume that the number of mines < 10
                    if let Some(x) = v.to_string().chars().next() {
                        board[i][j] = x;
                    }
                }
            }
        }
    }

    board.iter().map(|r| r.iter().collect()).collect()
}

fn build_board(minefield: &[&str]) -> Vec<Vec<char>> {
    let mut board: Vec<Vec<char>> = Vec::new();
    for &row in minefield {
        board.push(row.chars().collect());
    }
    board
}

fn get_value(board: &Vec<Vec<char>>, r: i16, c: i16) -> u8 {
    let rows = board.len() as i16;
    assert!(rows > 0);
    let cols = board[0].len() as i16;
    assert!(cols > 0);
    let scan = |pos: Iter<(i16, i16)>| {
        pos.fold(0u8, |mut accm: u8, &(r, c)| {
            if r >= 0 && r < rows && c >= 0 && c < cols {
                let i = r as usize;
                let j = c as usize;
                accm += if board[i][j] == '*' { 1u8 } else { 0u8 };
            }
            accm
        })
    };
    let positions = [
        (r - 1, c),
        (r - 1, c + 1),
        (r, c + 1),
        (r + 1, c + 1),
        (r + 1, c),
        (r + 1, c - 1),
        (r, c - 1),
        (r - 1, c - 1),
    ];
    scan(positions.iter())
}
