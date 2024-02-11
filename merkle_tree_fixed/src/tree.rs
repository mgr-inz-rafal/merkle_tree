use std::{
    collections::hash_map::DefaultHasher,
    fmt::Debug,
    hash::{Hash, Hasher},
};

use crate::proof::{Direction, ProofStep};

#[derive(Debug)]
pub struct MerkleTree {
    nodes: Vec<String>,
}

impl MerkleTree {
    pub fn new(leaf_count: usize) -> Self {
        assert!(
            Self::is_power_of_two(leaf_count),
            "leaf count should be a power of 2"
        );

        Self {
            nodes: vec!["-".to_string(); leaf_count * 2],
        }
    }

    pub fn from_iter<T: Hash>(i: impl Iterator<Item = T>) -> Self {
        let all_items: Vec<_> = i.collect();
        let mut mt = MerkleTree::new(all_items.len());
        all_items.into_iter().enumerate().for_each(|(index, item)| {
            mt.set_at(index, item);
        });
        mt
    }

    pub fn root(&self) -> &String {
        &self.nodes[1]
    }

    pub fn len(&self) -> usize {
        self.nodes.len() / 2
    }

    pub fn set_at<T>(&mut self, item_index: usize, item: T)
    where
        T: Hash,
    {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        let my_hash = hasher.finish().to_string();
        let node_index = item_index + self.len();
        self.nodes[node_index] = my_hash.clone();

        self.hash_recursive(node_index);
    }

    // TODO: Add NodeIndex type
    fn hash_recursive(&mut self, node_index: usize) {
        let current_hash = self.nodes[node_index].clone();
        let sibling = Self::sibling_index(node_index);
        let sibling_hash = &self.nodes[sibling];

        let concat_hash = format!("{}{}", current_hash, sibling_hash);

        // TODO: Extract to simple "hash" function.
        let mut hasher = DefaultHasher::new();
        concat_hash.hash(&mut hasher);
        let parent_hash = hasher.finish().to_string();

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

    pub fn verify<T>(proof: &[ProofStep], item: T) -> String
    where
        T: Hash,
    {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        let mut my_hash = hasher.finish().to_string();

        for step in proof {
            match step.direction() {
                Direction::Right => {
                    let concat = format!("{}{}", my_hash, step.hash());

                    let mut hasher = DefaultHasher::new();
                    concat.hash(&mut hasher);
                    my_hash = hasher.finish().to_string();
                }
                Direction::Left => {
                    let concat = format!("{}{}", step.hash(), my_hash);

                    let mut hasher = DefaultHasher::new();
                    concat.hash(&mut hasher);
                    my_hash = hasher.finish().to_string();
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
