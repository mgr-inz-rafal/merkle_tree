#[derive(Debug)]
pub struct ProofStep {
    hash: String,
    direction: Direction,
}

impl ProofStep {
    pub fn new(hash: String, direction: Direction) -> Self {
        Self { hash, direction }
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn hash(&self) -> &String {
        &self.hash
    }
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}
