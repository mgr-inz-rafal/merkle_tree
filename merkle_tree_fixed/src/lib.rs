mod tree;
mod proof;

use std::{
    collections::hash_map::DefaultHasher,
    fmt::Debug,
    hash::{Hash, Hasher},
};

pub use tree::MerkleTree;