#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

const BOARD_SIZE: i32 = 8;

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank >= 0 && rank < BOARD_SIZE && file >= 0 && file < BOARD_SIZE {
            Some(ChessPosition { rank, file })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    // In the game of chess, a queen can attack pieces which are on the
    // same row, column, or diagonal.
    pub fn can_attack(&self, other: &Queen) -> bool {
	let (r1, f1) = (self.position.rank, self.position.file);
	let (r2, f2) = (other.position.rank, other.position.file);
	if r1 == r2 {		// same row
	    true
	} else if f1 == f2 {	// same column
	    true
	} else {
	    r2.abs_diff(r1) == f2.abs_diff(f1)
	}
    }
}
