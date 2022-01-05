#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        let n = self.scores.len();
        if self.scores.len() > 0 {
            return Some(self.scores[n - 1]);
        } else {
            return None;
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        let n = self.scores.len();
        if n == 0 {
            return None;
        } else {
            let mut max = 0;
            for i in 0..n {
                if self.scores[i] > max {
                    max = self.scores[i];
                }
            }
            return Some(max);
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        if self.scores.len() <= 3 {
            let mut result = self.scores.to_vec();
            result.sort_unstable_by(|a, b| b.cmp(a));
            return result;
        } else {
            let mut result = vec![0; 3];
            for &score in self.scores {
                if score > result[0] {
                    result[2] = result[1];
                    result[1] = result[0];
                    result[0] = score;
                } else if score > result[1] {
                    result[2] = result[1];
                    result[1] = score;
                } else if score > result[2] {
                    result[2] = score;
                }
            }
            return result;
        }
    }
}
