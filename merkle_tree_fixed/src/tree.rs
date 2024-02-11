use std::{
    collections::hash_map::DefaultHasher,
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::proof::{Direction, ProofStep};

#[derive(Debug)]
pub struct MerkleTree<Hasher>
where
    Hasher: Fn(&[u8]) -> Vec<u8>,
{
    nodes: Vec<Vec<u8>>,
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
            nodes: vec![vec![]; leaf_count * 2],
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
        &self.nodes[1]
    }

    pub fn len(&self) -> usize {
        self.nodes.len() / 2
    }

    pub fn set_at(&mut self, item_index: usize, item: &[u8]) {
        let my_hash = (self.hasher)(item);
        let node_index = item_index + self.len();
        self.nodes[node_index] = my_hash.clone();

        self.hash_recursive(node_index);
    }

    // TODO: Add NodeIndex type
    fn hash_recursive(&mut self, node_index: usize) {
        let current_hash = self.nodes[node_index].clone();
        let sibling = Self::sibling_index(node_index);
        let sibling_hash = &self.nodes[sibling];

        let concat = format!("{}{}", hex::encode(current_hash), hex::encode(sibling_hash));
        let parent_hash = (self.hasher)(concat.as_bytes());

        let parent = Self::parent_index(node_index);
        self.nodes[parent] = parent_hash;

        if parent == 1 {
            return;
        }
        self.hash_recursive(parent)
    }

    pub fn proof(&self, item_index: usize) -> Vec<ProofStep> {
        let mut proof = vec![];
        let node_index = item_index + self.len();
        self.proof_recursive(node_index, &mut proof);
        proof
    }

    fn proof_recursive(&self, node_index: usize, proof: &mut Vec<ProofStep>) {
        if node_index == 1 {
            return;
        }
        proof.push(ProofStep::new(
            self.nodes[Self::sibling_index(node_index)].clone(),
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
                    let concat = format!("{}{}", hex::encode(my_hash), hex::encode(step.hash()));
                    my_hash = (hasher)(concat.as_bytes());
                }
                Direction::Left => {
                    let concat = format!("{}{}", hex::encode(step.hash()), hex::encode(my_hash));
                    my_hash = (hasher)(concat.as_bytes());
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

    fn parent_index(node_index: usize) -> usize {
        if Self::is_left(node_index) {
            node_index / 2
        } else {
            (node_index - 1) / 2
        }
    }

    fn sibling_index(node_index: usize) -> usize {
        if Self::is_left(node_index) {
            node_index + 1
        } else {
            node_index - 1
        }
    }

    fn is_left(node_index: usize) -> bool {
        node_index % 2 == 0
    }
}
