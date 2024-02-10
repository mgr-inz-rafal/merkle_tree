use std::{
    collections::hash_map::DefaultHasher,
    fmt::Debug,
    hash::{Hash, Hasher},
};

#[derive(Debug)]
struct MerkleTree {
    nodes: Vec<String>,
}

impl MerkleTree {
    fn new(leaf_count: usize) -> Self {
        assert!(
            is_power_of_two(leaf_count),
            "leaf count should be a power of 2"
        );

        Self {
            nodes: vec!["-".to_string(); leaf_count * 2],
        }
    }

    fn root(&self) -> &String {
        &self.nodes[1]
    }

    fn len(&self) -> usize {
        self.nodes.len() / 2
    }

    fn set_at<T>(&mut self, item_index: usize, item: T)
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
        let sibling = sibling_index(node_index);
        let sibling_hash = &self.nodes[sibling];

        let concat_hash = format!("{}{}", current_hash, sibling_hash);

        // TODO: Extract to simple "hash" function.
        let mut hasher = DefaultHasher::new();
        concat_hash.hash(&mut hasher);
        let parent_hash = hasher.finish().to_string();

        let parent = parent_index(node_index);
        self.nodes[parent] = parent_hash;

        if parent == 1 {
            return;
        }
        self.hash_recursive(parent)
    }

    fn proof(&self, item_index: usize) -> Vec<ProofStep> {
        let mut proof = vec![];
        let node_index = item_index + self.len();
        self.proof_recursive(node_index, &mut proof);
        proof
    }

    fn proof_recursive(&self, node_index: usize, proof: &mut Vec<ProofStep>) {
        if node_index == 1 {
            return;
        }
        proof.push(ProofStep {
            hash: self.nodes[sibling_index(node_index)].clone(),
            direction: if is_left(node_index) {
                Direction::Left
            } else {
                Direction::Right
            },
        });
        self.proof_recursive(parent_index(node_index), proof)
    }
}

fn is_power_of_two(n: usize) -> bool {
    if n == 0 {
        false
    } else {
        n & (n - 1) == 0
    }
}

fn parent_index(node_index: usize) -> usize {
    if is_left(node_index) {
        node_index / 2
    } else {
        (node_index - 1) / 2
    }
}

fn sibling_index(node_index: usize) -> usize {
    if is_left(node_index) {
        node_index + 1
    } else {
        node_index - 1
    }
}

fn is_left(node_index: usize) -> bool {
    node_index % 2 == 0
}

#[derive(Debug)]
struct ProofStep {
    hash: String,
    direction: Direction,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn verify<T>(proof: &[ProofStep], item: T) -> String
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    item.hash(&mut hasher);
    let mut my_hash = hasher.finish().to_string();

    for ProofStep { hash, direction } in proof {
        match direction {
            Direction::Right => {
                let concat = format!("{}{}", my_hash, hash);

                let mut hasher = DefaultHasher::new();
                concat.hash(&mut hasher);
                my_hash = hasher.finish().to_string();
            }
            Direction::Left => {
                let concat = format!("{}{}", hash, my_hash);

                let mut hasher = DefaultHasher::new();
                concat.hash(&mut hasher);
                my_hash = hasher.finish().to_string();
            }
        }
    }
    my_hash
}

fn main() {
    let chars = "ABCD";
    chars.chars().for_each(|c| {
        let mut hasher = DefaultHasher::new();
        c.hash(&mut hasher);
        println!("{} - {}", c, hasher.finish());
    });

    let mut mt = MerkleTree::new(4);

    mt.set_at(0, 'A');
    mt.set_at(1, 'B');
    mt.set_at(2, 'C');
    mt.set_at(3, 'D');
    dbg!(&mt);

    let proof = mt.proof(2);
    dbg!(&proof);

    let expected_root = mt.root();
    let actual_root = verify(&proof, 'C');

    assert_eq!(expected_root, &actual_root)
}
