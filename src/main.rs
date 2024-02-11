use std::{
    collections::hash_map::DefaultHasher,
    fmt::Debug,
    hash::{Hash, Hasher},
};

use rand::Rng;

use merkle_tree::MerkleTree;

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

    let proof = mt.proof(2);
    let expected_root = mt.root();
    let actual_root = MerkleTree::verify(&proof, 'C');
    assert_eq!(expected_root, &actual_root);

    let mt = MerkleTree::from_iter("ABCDEFGH".chars());
    let proof = mt.proof(7);
    let expected_root = mt.root();
    let actual_root = MerkleTree::verify(&proof, 'H');
    assert_eq!(expected_root, &actual_root);

    let mt = MerkleTree::from_iter(["Stefan", "Zenek", "Mariusz", "Ewelina"].into_iter());
    let proof = mt.proof(3);
    let expected_root = mt.root();
    let actual_root = MerkleTree::verify(&proof, "Ewelina");
    assert_eq!(expected_root, &actual_root);

    let mut rng = rand::thread_rng();
    let count = 2_usize.pow(20);
    let integers: Vec<_> = std::iter::repeat_with(|| rng.gen::<u128>())
        .take(count)
        .collect();
    let mt = MerkleTree::from_iter(integers.iter());
    let proof = mt.proof(12345);
    let expected_root = mt.root();
    let actual_root = MerkleTree::verify(&proof, integers[12345]);
    assert_eq!(expected_root, &actual_root);
}
