use core::fmt;
use std::fmt::Display;

struct Node<T> {
    is_fake: bool,
    data: T,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self {
            is_fake: false,
            data,
        }
    }

    fn new_fake(data: T) -> Self {
        Self {
            is_fake: true,
            data,
        }
    }
}

impl<T: Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.data, if self.is_fake { '-' } else { ' ' })?;
        Ok(())
    }
}

struct MerkleTree<T: Default> {
    nodes: Vec<Node<T>>,
}

impl<T: Default> MerkleTree<T> {
    pub fn new(data: T) -> Self {
        Self {
            nodes: vec![Node::new_fake(T::default()), Node::new(data)],
        }
    }

    pub fn add(&mut self, node: Node<T>) {
        self.nodes.push(node)
    }

    pub fn height(&self) -> u32 {
        self.nodes.len().ilog2()
    }

    pub fn level_of(&self, i: usize) -> u32 {
        self.height() - if i == 0 { 0 } else { i.ilog2() }
    }

    pub fn len(&mut self) -> usize {
        self.nodes.len()
    }

    pub fn node(&self, i: usize) -> Option<&Node<T>> {
        self.nodes.get(i)
    }
}

impl<T> fmt::Display for MerkleTree<T>
where
    T: Default + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "total nodes = {}, height = {}",
            self.nodes.len(),
            self.height()
        )?;
        let mut current_level: u32 = u32::MAX;
        for (i, n) in self.nodes.iter().enumerate().skip(1) {
            let level_of = self.level_of(i);
            if current_level != level_of {
                current_level = level_of;
                writeln!(f, "")?;
            }
            write!(f, "{}", n)?;
        }
        Ok(())
    }
}

fn main() {
    //                 A                  level 3
    //          B             C           level 2
    //      D      E      F      G        level 1
    //    H   I  J  .   .   .  .   .      level 0

    let mut mt: MerkleTree<char> = MerkleTree::new('A');
    mt.add(Node::new('B'));
    mt.add(Node::new('C'));
    mt.add(Node::new('D'));
    mt.add(Node::new('E'));
    mt.add(Node::new('F'));
    mt.add(Node::new('G'));
    mt.add(Node::new('H'));
    mt.add(Node::new('I'));
    mt.add(Node::new('J'));

    println!("{}", mt);

    for i in 1..mt.len() {
        let node = mt.node(i).unwrap();
        let parent = mt.node(i / 2).unwrap();
        println!("{} (level:{}) -> {}", node, mt.level_of(i), parent);
    }
}

#[cfg(test)]
mod tests {
    use crate::{MerkleTree, Node};

    #[test]
    fn should_calculate_height() {
        let mut mt: MerkleTree = MerkleTree::new('A');
        mt.add(Node::new('B'));
        mt.add(Node::new('C'));
        mt.add(Node::new('D'));
        mt.add(Node::new('E'));
        mt.add(Node::new('F'));
        mt.add(Node::new('G'));
        mt.add(Node::new('H'));
        mt.add(Node::new('I'));
        mt.add(Node::new('J'));

        assert_eq!(mt.height(), 3)
    }

    #[test]
    fn should_calculate_level_of_item() {
        let mut mt: MerkleTree = MerkleTree::new('A');
        mt.add(Node::new('B'));
        mt.add(Node::new('C'));
        mt.add(Node::new('D'));
        mt.add(Node::new('E'));
        mt.add(Node::new('F'));
        mt.add(Node::new('G'));
        mt.add(Node::new('H'));
        mt.add(Node::new('I'));
        mt.add(Node::new('J'));

        assert_eq!(mt.level_of(1), 3);
        assert_eq!(mt.level_of(2), 2);
        assert_eq!(mt.level_of(3), 2);
        assert_eq!(mt.level_of(4), 1);
        assert_eq!(mt.level_of(5), 1);
        assert_eq!(mt.level_of(6), 1);
        assert_eq!(mt.level_of(7), 1);
        assert_eq!(mt.level_of(8), 0);
        assert_eq!(mt.level_of(9), 0);
        assert_eq!(mt.level_of(10), 0);
    }
}
