#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

const FRAME_CNT: usize = 10;
const BUFFER_SLOTS: usize = 21;
const TOTAL_PINS: u16 = 10;

#[derive(Default)]
pub struct BowlingGame {
    frame_begins: [usize; FRAME_CNT],
    frame_top: usize,
    rolls: [u16; BUFFER_SLOTS],
    rolls_top: usize,
    pins: u16,
}

impl BowlingGame {
    /// Data structure, for example
    /// rolls  [ 0, 1, 2, 3, ..., 16, 17, 18, 19, 20(bonus) ]
    ///          ^  ^     ^       ^       ^
    /// frames [ 0, 1,    2, ..., 8,      9,            ]
    pub fn new() -> Self {
        Self {
            frame_begins: [0; FRAME_CNT],
            frame_top: 0,
            rolls: [0; BUFFER_SLOTS],
            rolls_top: 0,
            pins: TOTAL_PINS,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.frame_top == FRAME_CNT {
            return Err(Error::GameComplete);
        }
        if pins > self.pins {
            return Err(Error::NotEnoughPinsLeft);
        }
        self.pins -= pins;
        self.rolls[self.rolls_top] = pins;
        self.rolls_top += 1;
        let offset = self.rolls_top - self.frame_begins[self.frame_top];
        if self.frame_top == FRAME_CNT - 1 {
            if (offset == 1 || offset == 2) && self.pins == 0 {
                self.pins = TOTAL_PINS;
            } else if offset == 2 && self.pins != 0 {
                let first_roll = self.rolls[self.frame_begins[self.frame_top]];
                if first_roll != TOTAL_PINS {
                    self.begin_new_frame();
                }
            } else if offset == 3 {
                self.begin_new_frame();
            }
        } else if self.pins == 0 || offset == 2 {
            // (strike or spare) or the second roll is complete
            self.begin_new_frame();
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frame_top < FRAME_CNT {
            None
        } else {
            let mut sum = 0;
            for i in 0..FRAME_CNT {
                let j = self.frame_begins[i];
                let a = self.rolls[j];
                let b = self.rolls[j + 1];
                sum += a + b;
                if a == TOTAL_PINS || a + b == TOTAL_PINS {
                    sum += self.rolls[j + 2];
                }
            }
            Some(sum)
        }
    }

    fn begin_new_frame(&mut self) {
        self.frame_top += 1;
        if self.frame_top < FRAME_CNT {
            self.frame_begins[self.frame_top] = self.rolls_top;
        }
        self.pins = TOTAL_PINS;
    }
}
