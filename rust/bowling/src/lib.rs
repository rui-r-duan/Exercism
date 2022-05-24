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
    frame_beginings: [usize; FRAME_CNT],
    curr_frame: usize,
    rolls: [u16; BUFFER_SLOTS],
    curr_roll: usize,
    pins: u16,
}

impl BowlingGame {
    /// Data structure, for example
    /// rolls  [ 0, 1, 2, 3, ..., 16, 17, 18, 19, 20(bonus) ]
    ///          ^  ^     ^       ^       ^
    /// frames [ 0, 1,    2, ..., 8,      9,            ]
    pub fn new() -> Self {
        Self {
            pins: TOTAL_PINS,
            ..Default::default()
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.curr_frame == FRAME_CNT {
            return Err(Error::GameComplete);
        }
        if pins > self.pins {
            return Err(Error::NotEnoughPinsLeft);
        }
        self.pins -= pins;
        self.rolls[self.curr_roll] = pins;
        self.curr_roll += 1;
        let offset = self.curr_roll - self.frame_beginings[self.curr_frame];
        if self.curr_frame == FRAME_CNT - 1 {
            if (offset == 1 || offset == 2) && self.pins == 0 {
                self.pins = TOTAL_PINS;
            } else if offset == 2 && self.pins != 0 {
                let first_roll = self.rolls[self.frame_beginings[self.curr_frame]];
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
        if self.curr_frame < FRAME_CNT {
            None
        } else {
            let mut sum = 0;
            for i in 0..FRAME_CNT {
                let j = self.frame_beginings[i];
                let (a, b) = (self.rolls[j], self.rolls[j + 1]);
                sum += a + b;
                if a == TOTAL_PINS || a + b == TOTAL_PINS {
                    sum += self.rolls[j + 2];
                }
            }
            Some(sum)
        }
    }

    fn begin_new_frame(&mut self) {
        self.curr_frame += 1;
        if self.curr_frame < FRAME_CNT {
            self.frame_beginings[self.curr_frame] = self.curr_roll;
        }
        self.pins = TOTAL_PINS;
    }
}
