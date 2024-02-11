#[derive(Copy, Clone)]
pub struct NodeIndex(usize);

impl NodeIndex {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn is_root(&self) -> bool {
        self.0 == 1
    }

    pub fn inner(&self) -> usize {
        self.0
    }
}
