use std::fmt::Debug;

use crate::{
    node_index::NodeIndex,
    proof::{Location, Proof, ProofStep},
};

#[derive(Debug)]
pub struct Nodes(Vec<Vec<u8>>);

impl Nodes {
    fn new(leaf_count: usize) -> Self {
        Self(vec![vec![0u8]; leaf_count * 2])
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

    pub fn leaf_count(&self) -> usize {
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
        NodeIndex::new(index + self.leaf_count())
    }

    fn concat(one: &[u8], two: &[u8]) -> Vec<u8> {
        one.iter().copied().chain(two.iter().copied()).collect()
    }

    fn hash_recursive(&mut self, node_index: NodeIndex) {
        let current_hash = self.nodes.at(node_index);
        let sibling = Self::sibling_index(node_index);
        let sibling_hash = &self.nodes.at(sibling);
        let concat = if Self::is_left(node_index) {
            Self::concat(current_hash, sibling_hash)
        } else {
            Self::concat(sibling_hash, current_hash)
        };
        let parent_hash = (self.hasher)(&concat);
        let parent = Self::parent_index(node_index);
        self.nodes.set_at(parent, &parent_hash);

        if parent.is_root() {
            return;
        }
        self.hash_recursive(parent)
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Vec<u8>> {
        self.nodes.0.iter().skip(1)
    }

    pub fn leaves(&self) -> impl Iterator<Item = &Vec<u8>> {
        self.nodes.0.iter().skip(self.leaf_count())
    }

    pub fn proof(&self, index: usize) -> Proof {
        let mut proof = Proof::new(self.leaf_count());
        let node_index = self.to_node_index(index);
        self.proof_recursive(node_index, &mut proof);
        proof
    }

    fn proof_recursive(&self, node_index: NodeIndex, proof: &mut Proof) {
        if node_index.is_root() {
            return;
        }

        proof.add_step(ProofStep::new(
            self.nodes.at(Self::sibling_index(node_index)).clone(),
            if Self::is_left(node_index) {
                Location::Right
            } else {
                Location::Left
            },
        ));

        self.proof_recursive(Self::parent_index(node_index), proof)
    }

    pub fn verify(proof: &Proof, item: &[u8], hasher: Hasher) -> Vec<u8>
    where
        Hasher: Fn(&[u8]) -> Vec<u8>,
    {
        let mut my_hash = (hasher)(item);

        for step in proof.iter() {
            let concat = match step.direction() {
                Location::Left => Self::concat(&my_hash, step.hash()),
                Location::Right => Self::concat(step.hash(), &my_hash),
            };
            my_hash = (hasher)(&concat);
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

#[cfg(test)]
mod tests {
    use crc::{Crc, CRC_8_DARC};

    use crate::{
        proof::{Location, Proof, ProofStep},
        MerkleTree,
    };

    fn hasher(data: &[u8]) -> Vec<u8> {
        let crc = Crc::<u8>::new(&CRC_8_DARC);
        let mut digest = crc.digest();
        digest.update(data);
        vec![digest.finalize()]
    }

    #[test]
    fn should_calculate_root() {
        let leaves = &[
            ("Alpha", 0x47),
            ("Bravo", 0x24),
            ("Charlie", 0x7E),
            ("Delta", 0x56),
            ("Echo", 0xEF),
            ("Foxtrot", 0x49),
            ("Golf", 0x12),
            ("Hotel", 0x04),
        ];

        const EXPECTED_ROOT: u8 = 0x0B;

        //               0B (EXPECTED_ROOT)
        //                |
        //        +-------+-------+
        //        |               |
        //       4C              DE
        //        |               |
        //    +---+---+       +---+---+
        //    |       |       |       |
        //   58      28      00      D5
        //    |       |       |       |
        //  +-+-+   +-+-+   +-+-+   +-+-+
        //  |   |   |   |   |   |   |   |
        // 47  24  7E  56  EF  49  12  04

        let mt = MerkleTree::from_iter(leaves.iter().map(|(data, _)| data.as_bytes()), hasher);

        let expected_root = vec![EXPECTED_ROOT];
        let actual_root = mt.root();
        assert_eq!(&expected_root, actual_root);
    }

    #[test]
    fn leaves_populated_in_different_order_should_yield_equal_root() {
        let mut mt = MerkleTree::new(4, hasher);
        mt.set_at(0, "A".as_bytes());
        mt.set_at(1, "B".as_bytes());
        mt.set_at(2, "C".as_bytes());
        mt.set_at(3, "D".as_bytes());
        let root_1 = mt.root();

        let mut mt = MerkleTree::new(4, hasher);
        mt.set_at(3, "D".as_bytes());
        mt.set_at(2, "C".as_bytes());
        mt.set_at(1, "B".as_bytes());
        mt.set_at(0, "A".as_bytes());
        let root_2 = mt.root();

        assert_eq!(root_1, root_2);

        let mut mt = MerkleTree::new(4, hasher);
        mt.set_at(2, "C".as_bytes());
        mt.set_at(3, "D".as_bytes());
        mt.set_at(1, "B".as_bytes());
        mt.set_at(0, "A".as_bytes());
        let root_3 = mt.root();

        assert_eq!(root_1, root_3);
    }

    #[test]
    fn should_return_nodes() {
        let leaves = &[
            ("Alpha", 0x47),
            ("Bravo", 0x24),
            ("Charlie", 0x7E),
            ("Delta", 0x56),
            ("Echo", 0xEF),
            ("Foxtrot", 0x49),
            ("Golf", 0x12),
            ("Hotel", 0x04),
        ];

        //               0B
        //                |
        //        +-------+-------+
        //        |               |
        //       4C              DE
        //        |               |
        //    +---+---+       +---+---+
        //    |       |       |       |
        //   58      28      00      D5
        //    |       |       |       |
        //  +-+-+   +-+-+   +-+-+   +-+-+
        //  |   |   |   |   |   |   |   |
        // 47  24  7E  56  EF  49  12  04

        let mt = MerkleTree::from_iter(leaves.iter().map(|(data, _)| data.as_bytes()), hasher);

        let expected_nodes = vec![
            0x0B, 0x4C, 0xDE, 0x58, 0x28, 0x00, 0xD5, 0x47, 0x24, 0x7E, 0x56, 0xEF, 0x49, 0x12,
            0x04,
        ];
        let actual_nodes: Vec<u8> = mt.nodes().map(|n| *n.first().unwrap()).collect();
        assert_eq!(expected_nodes, actual_nodes);
    }

    #[test]
    fn should_return_leaves() {
        let leaves = &[
            ("Alpha", 0x47),
            ("Bravo", 0x24),
            ("Charlie", 0x7E),
            ("Delta", 0x56),
            ("Echo", 0xEF),
            ("Foxtrot", 0x49),
            ("Golf", 0x12),
            ("Hotel", 0x04),
        ];

        //               0B
        //                |
        //        +-------+-------+
        //        |               |
        //       4C              DE
        //        |               |
        //    +---+---+       +---+---+
        //    |       |       |       |
        //   58      28      00      D5
        //    |       |       |       |
        //  +-+-+   +-+-+   +-+-+   +-+-+
        //  |   |   |   |   |   |   |   |
        // 47  24  7E  56  EF  49  12  04

        let mt = MerkleTree::from_iter(leaves.iter().map(|(data, _)| data.as_bytes()), hasher);

        let expected_nodes = vec![0x47, 0x24, 0x7E, 0x56, 0xEF, 0x49, 0x12, 0x04];
        let actual_nodes: Vec<u8> = mt.leaves().map(|n| *n.first().unwrap()).collect();
        assert_eq!(expected_nodes, actual_nodes);
    }

    #[test]
    fn calculates_node_index_large_tree() {
        let leaves = &[
            ("Alpha", 0x47),
            ("Bravo", 0x24),
            ("Charlie", 0x7E),
            ("Delta", 0x56),
            ("Echo", 0xEF),
            ("Foxtrot", 0x49),
            ("Golf", 0x12),
            ("Hotel", 0x04),
        ];

        //               0B
        //                |
        //        +-------+-------+
        //        |               |
        //       4C              DE
        //        |               |
        //    +---+---+       +---+---+
        //    |       |       |       |
        //   58       28      00      D5
        //    |       |       |       |
        //  +-+-+   +-+-+   +-+-+   +-+-+
        //  |   |   |   |   |   |   |   |
        // 47  24  7E  56  EF  49  12  04

        let mt = MerkleTree::from_iter(leaves.iter().map(|(data, _)| data.as_bytes()), hasher);

        assert_eq!(mt.to_node_index(0).inner(), 8);
        assert_eq!(mt.to_node_index(1).inner(), 9);
        assert_eq!(mt.to_node_index(2).inner(), 10);
        assert_eq!(mt.to_node_index(3).inner(), 11);
        assert_eq!(mt.to_node_index(4).inner(), 12);
        assert_eq!(mt.to_node_index(5).inner(), 13);
        assert_eq!(mt.to_node_index(6).inner(), 14);
        assert_eq!(mt.to_node_index(7).inner(), 15);
    }

    #[test]
    fn calculates_node_index_small_tree() {
        let leaves = &[
            ("Alpha", 0x47),
            ("Bravo", 0x24),
            ("Charlie", 0x7E),
            ("Delta", 0x56),
        ];

        //       4C
        //        |
        //    +---+---+
        //    |       |
        //   58       28
        //    |       |
        //  +-+-+   +-+-+
        //  |   |   |   |
        // 47  24  7E  56

        let mt = MerkleTree::from_iter(leaves.iter().map(|(data, _)| data.as_bytes()), hasher);

        assert_eq!(mt.to_node_index(0).inner(), 4);
        assert_eq!(mt.to_node_index(1).inner(), 5);
        assert_eq!(mt.to_node_index(2).inner(), 6);
        assert_eq!(mt.to_node_index(3).inner(), 7);
    }

    #[test]
    fn generates_proof_for_left_leaf() {
        let leaves = &[
            ("Alpha", 0x47),
            ("Bravo", 0x24),
            ("Charlie", 0x7E),
            ("Delta", 0x56),
            ("Echo", 0xEF),
            ("Foxtrot", 0x49),
            ("Golf", 0x12),
            ("Hotel", 0x04),
        ];

        //               0B
        //                |
        //        +-------+-------+
        //        |               |
        //       4C             [DE]
        //        |               |
        //    +---+---+       +---+---+
        //    |       |       |       |
        //  [58]      28      00      D5
        //    |       |       |       |
        //  +-+-+   +-+-+   +-+-+   +-+-+
        //  |   |   |   |   |   |   |   |
        // 47  24 [7E] 56  EF  49  12  04
        //              |
        //              + prooving this

        let mt = MerkleTree::from_iter(leaves.iter().map(|(data, _)| data.as_bytes()), hasher);

        let actual_proof = mt.proof(3);
        let mut expected_proof = Proof::new(mt.leaf_count());
        expected_proof.add_step(ProofStep::new(vec![0x7E], Location::Left));
        expected_proof.add_step(ProofStep::new(vec![0x58], Location::Left));
        expected_proof.add_step(ProofStep::new(vec![0xDE], Location::Right));
        assert_eq!(expected_proof, actual_proof);
    }

    #[test]
    fn generates_proof_for_right_leaf() {
        let leaves = &[
            ("Alpha", 0x47),
            ("Bravo", 0x24),
            ("Charlie", 0x7E),
            ("Delta", 0x56),
            ("Echo", 0xEF),
            ("Foxtrot", 0x49),
            ("Golf", 0x12),
            ("Hotel", 0x04),
        ];

        //               0B
        //                |
        //        +-------+-------+
        //        |               |
        //      [4C]             DE
        //        |               |
        //    +---+---+       +---+---+
        //    |       |       |       |
        //   58       28     [00]     D5
        //    |       |       |       |
        //  +-+-+   +-+-+   +-+-+   +-+-+
        //  |   |   |   |   |   |   |   |
        // 47  24  7E  56  EF  49 [12] 04
        //                             |
        //                             + prooving this

        let mt = MerkleTree::from_iter(leaves.iter().map(|(data, _)| data.as_bytes()), hasher);

        let actual_proof = mt.proof(7);
        let mut expected_proof = Proof::new(mt.leaf_count());
        expected_proof.add_step(ProofStep::new(vec![0x12], Location::Left));
        expected_proof.add_step(ProofStep::new(vec![0x00], Location::Left));
        expected_proof.add_step(ProofStep::new(vec![0x4C], Location::Left));
        assert_eq!(expected_proof, actual_proof);
    }
}
