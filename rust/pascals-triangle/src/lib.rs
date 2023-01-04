pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        if row_count == 0 {
            return PascalsTriangle { rows: Vec::new() };
        }

        let mut rows = vec![vec![1]];
        for i in 2..=row_count {
            rows.push(PascalsTriangle::new_row(&rows[i as usize - 2]));
        }
        PascalsTriangle { rows: rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }

    fn new_row(prev_row: &[u32]) -> Vec<u32> {
        let mut new_row = Vec::with_capacity(prev_row.len() + 1);
        let mut p = 0;
        for i in 0..prev_row.len() {
            new_row.push(p + prev_row[i]);
            p = prev_row[i];
        }
        new_row.push(p);

        new_row
    }
}
