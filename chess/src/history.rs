use crate::movement::Movement;

pub struct History {
    moves: Vec<Movement>
}

impl History {
    pub fn push (&mut self, movement: Movement) {
        self.moves.push(movement);
    }
    pub fn pop (&mut self) -> Option<Movement> {
        self.moves.pop()
    }
    pub fn is_empty (&self) -> bool {
        self.moves.is_empty()
    }
    pub fn clear (&mut self) {
        self.moves.clear();
    }
}