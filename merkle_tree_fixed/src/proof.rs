#[derive(Debug, PartialEq)]
pub enum Location {
    Right,
    Left,
}

#[derive(Debug, PartialEq)]
pub struct ProofStep {
    hash: Vec<u8>,
    direction: Location,
}

impl ProofStep {
    pub fn new(hash: Vec<u8>, direction: Location) -> Self {
        Self { hash, direction }
    }

    pub fn direction(&self) -> &Location {
        &self.direction
    }

    pub fn hash(&self) -> &Vec<u8> {
        &self.hash
    }
}

#[derive(Debug, PartialEq)]
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
