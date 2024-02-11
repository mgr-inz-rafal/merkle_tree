#[derive(Debug)]
pub struct ProofStep {
    hash: Vec<u8>,
    direction: Direction,
}

impl ProofStep {
    pub fn new(hash: Vec<u8>, direction: Direction) -> Self {
        Self { hash, direction }
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn hash(&self) -> &Vec<u8> {
        &self.hash
    }
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}
