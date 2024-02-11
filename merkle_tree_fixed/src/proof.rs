#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

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
pub struct Proof(Vec<ProofStep>);

impl Proof {
    pub(crate) fn new(leaf_count: usize) -> Self {
        Self(Vec::with_capacity(leaf_count.ilog2() as usize))
    }

    pub(crate) fn add_step(&mut self, step: ProofStep) {
        self.0.push(step)
    }

    pub fn iter(&self) -> impl Iterator<Item = &ProofStep> {
        self.0.iter()
    }
}
