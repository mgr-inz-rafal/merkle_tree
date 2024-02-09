use std::{
    collections::hash_map::DefaultHasher,
    fmt::Debug,
    hash::{Hash, Hasher},
};

#[derive(Debug)]
struct MerkleTree {
    hashes: Vec<String>,
}

impl MerkleTree {
    fn new(leaf_count: usize) -> Self {
        assert!(
            is_power_of_two(leaf_count),
            "leaf count should be a power of 2"
        );

        Self {
            hashes: vec!["-".to_string(); leaf_count * 2],
        }
    }

    fn len(&self) -> usize {
        self.hashes.len() / 2
    }

    fn set_at<T>(&mut self, index: usize, item: T)
    where
        T: Hash,
    {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        let my_hash = hasher.finish().to_string();

        let i = index + self.len();
        self.hashes[i] = my_hash.clone();

        let sibling = sibling_index(i);
        let sibling_hash = &self.hashes[sibling];

        let concat_hash = format!("{}{}", my_hash, sibling_hash);
        let mut hasher = DefaultHasher::new();
        concat_hash.hash(&mut hasher);
        let parent_hash = hasher.finish().to_string();

        let parent = parent_index(i);
        self.hashes[parent] = parent_hash;
    }
}

fn is_power_of_two(n: usize) -> bool {
    if n == 0 {
        false
    } else {
        n & (n - 1) == 0
    }
}

fn parent_index(index: usize) -> usize {
    if index % 2 == 0 {
        index / 2
    } else {
        (index - 1) / 2
    }
}

fn sibling_index(index: usize) -> usize {
    if index % 2 == 0 {
        index + 1
    } else {
        index - 1
    }
}

fn main() {
    let chars = "ABCD";
    chars.chars().for_each(|c| {
        let mut hasher = DefaultHasher::new();
        c.hash(&mut hasher);
        println!("{} - {}", c, hasher.finish());
    });

    let mut mt = MerkleTree::new(4);

    mt.set_at(1, 'A');

    dbg!(&mt);
}
