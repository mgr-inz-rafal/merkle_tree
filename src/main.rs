use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use merkle_tree::MerkleTree;

fn main() {
    let chars = "ABCD";
    chars.chars().for_each(|c| {
        let mut hasher = DefaultHasher::new();
        c.hash(&mut hasher);
        println!("{} - {}", c, hasher.finish());
    });

    let hasher = |data: &[u8]| {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        hasher.finish().to_be_bytes().into()
    };

    let mut mt = MerkleTree::new(4, hasher);
    mt.set_at(0, "Ala".to_string().as_bytes());
    mt.set_at(1, "Beata".to_string().as_bytes());
    mt.set_at(2, "Czesław".to_string().as_bytes());
    mt.set_at(3, "Dariusz".to_string().as_bytes());

    let proof = mt.proof(2);
    let expected_root = mt.root();
    let actual_root = MerkleTree::verify(&proof, "Czesław".to_string().as_bytes(), hasher);
    assert_eq!(expected_root, &actual_root);

    let actual_root = MerkleTree::verify(&proof, "Ala".to_string().as_bytes(), hasher);
    assert_ne!(expected_root, &actual_root);

    let actual_root = MerkleTree::verify(&proof, "Beata".to_string().as_bytes(), hasher);
    assert_ne!(expected_root, &actual_root);

    let actual_root = MerkleTree::verify(&proof, "Dariusz".to_string().as_bytes(), hasher);
    assert_ne!(expected_root, &actual_root);

    let actual_root = MerkleTree::verify(&proof, "X".to_string().as_bytes(), hasher);
    assert_ne!(expected_root, &actual_root);

    /*
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
    */
}
