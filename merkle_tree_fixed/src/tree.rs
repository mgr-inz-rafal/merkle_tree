use std::fmt::Debug;

use crate::{
    node_index::NodeIndex,
    proof::{Direction, ProofStep},
};

#[derive(Debug)]
pub struct Nodes(Vec<Vec<u8>>);

impl Nodes {
    fn new(leaf_count: usize) -> Self {
        Self(vec![vec![]; leaf_count * 2])
    }

    fn at(&self, index: NodeIndex) -> &Vec<u8> {
        &self.0[index.inner()]
    }

    fn set_at(&mut self, index: NodeIndex, data: &[u8]) {
        self.0[index.inner()] = data.to_vec();
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Debug)]
pub struct MerkleTree<Hasher>
where
    Hasher: Fn(&[u8]) -> Vec<u8>,
{
    nodes: Nodes,
    hasher: Hasher,
}

impl<Hasher> MerkleTree<Hasher>
where
    Hasher: Fn(&[u8]) -> Vec<u8>,
{
    pub fn new(leaf_count: usize, hasher: Hasher) -> Self {
        assert!(
            Self::is_power_of_two(leaf_count),
            "leaf count should be a power of 2"
        );

        Self {
            nodes: Nodes::new(leaf_count),
            hasher,
        }
    }

    pub fn from_iter<'a>(i: impl Iterator<Item = &'a [u8]>, hasher: Hasher) -> Self
    where
        Hasher: Fn(&[u8]) -> Vec<u8>,
    {
        let all_items: Vec<_> = i.collect();
        let mut mt = MerkleTree::new(all_items.len(), hasher);
        all_items.into_iter().enumerate().for_each(|(index, item)| {
            mt.set_at(index, item);
        });
        mt
    }

    pub fn root(&self) -> &Vec<u8> {
        self.nodes.at(NodeIndex::new(1))
    }

    pub fn len(&self) -> usize {
        self.nodes.len() / 2
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn set_at(&mut self, item_index: usize, item: &[u8]) {
        let node_index = self.to_node_index(item_index);

        let my_hash = (self.hasher)(item);
        self.nodes.set_at(node_index, &my_hash);

        self.hash_recursive(node_index);
    }

    fn to_node_index(&self, index: usize) -> NodeIndex {
        NodeIndex::new(index + self.len())
    }

    fn concat(one: &[u8], two: &[u8]) -> Vec<u8> {
        one.iter().copied().chain(two.iter().copied()).collect()
    }

    fn hash_recursive(&mut self, node_index: NodeIndex) {
        let current_hash = self.nodes.at(node_index);
        let sibling = Self::sibling_index(node_index);
        let sibling_hash = &self.nodes.at(sibling);

        let concat = Self::concat(current_hash, sibling_hash);
        let parent_hash = (self.hasher)(&concat);

        let parent = Self::parent_index(node_index);
        self.nodes.set_at(parent, &parent_hash);

        if parent.is_root() {
            return;
        }
        self.hash_recursive(parent)
    }

    pub fn proof(&self, index: usize) -> Vec<ProofStep> {
        let mut proof = vec![];
        let node_index = self.to_node_index(index);
        self.proof_recursive(node_index, &mut proof);
        proof
    }

    fn proof_recursive(&self, node_index: NodeIndex, proof: &mut Vec<ProofStep>) {
        if node_index.is_root() {
            return;
        }
        proof.push(ProofStep::new(
            self.nodes.at(Self::sibling_index(node_index)).clone(),
            if Self::is_left(node_index) {
                Direction::Left
            } else {
                Direction::Right
            },
        ));
        self.proof_recursive(Self::parent_index(node_index), proof)
    }

    pub fn verify(proof: &[ProofStep], item: &[u8], hasher: Hasher) -> Vec<u8>
    where
        Hasher: Fn(&[u8]) -> Vec<u8>,
    {
        let mut my_hash = (hasher)(item);

        for step in proof {
            match step.direction() {
                Direction::Right => {
                    let concat = Self::concat(&my_hash, step.hash());
                    my_hash = (hasher)(&concat);
                }
                Direction::Left => {
                    let concat = Self::concat(step.hash(), &my_hash);
                    my_hash = (hasher)(&concat);
                }
            }
        }
        my_hash
    }

    fn is_power_of_two(n: usize) -> bool {
        if n == 0 {
            false
        } else {
            n & (n - 1) == 0
        }
    }

    fn parent_index(node_index: NodeIndex) -> NodeIndex {
        if Self::is_left(node_index) {
            NodeIndex::new(node_index.inner() / 2)
        } else {
            NodeIndex::new((node_index.inner() - 1) / 2)
        }
    }

    fn sibling_index(node_index: NodeIndex) -> NodeIndex {
        if Self::is_left(node_index) {
            NodeIndex::new(node_index.inner() + 1)
        } else {
            NodeIndex::new(node_index.inner() - 1)
        }
    }

    fn is_left(node_index: NodeIndex) -> bool {
        node_index.inner() % 2 == 0
    }
}
