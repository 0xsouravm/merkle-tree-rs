use sha2::{Sha256, Digest};
use std::fmt;
use hex;

// MerkleNode represents a node in the Merkle tree
#[derive(Clone)]
enum MerkleNode {
    Leaf {
        data: Vec<u8>,
        hash: Vec<u8>,
    },
    Branch {
        left: Box<MerkleNode>,
        right: Box<MerkleNode>,
        hash: Vec<u8>,
    },
}

// The main MerkleTree structure
pub struct MerkleTree {
    root: Option<MerkleNode>,
    leaves: Vec<MerkleNode>,
}

impl fmt::Debug for MerkleNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MerkleNode::Leaf { data, hash } => {
                write!(f, "Leaf {{ data: {:?}, hash: {} }}", 
                       String::from_utf8_lossy(data), hex::encode(hash))
            }
            MerkleNode::Branch { left, right, hash } => {
                write!(f, "Branch {{ hash: {}, left: {:?}, right: {:?} }}", 
                       hex::encode(hash), left, right)
            }
        }
    }
}

impl MerkleTree {   
    // Print the tree structure for debugging
    pub fn print_tree(&self) {
        if let Some(root) = &self.root {
            println!("Merkle Tree Structure:");
            MerkleTree::print_node(root, 0);
        } else {
            println!("Empty tree");
        }
    }
    
    fn print_node(node: &MerkleNode, indent: usize) {
        let indent_str = " ".repeat(indent * 2);
        
        match node {
            MerkleNode::Leaf { data, hash } => {
                println!("{}Leaf: data={:?}, hash={}", 
                         indent_str, 
                         String::from_utf8_lossy(data), 
                         hex::encode(&hash[0..4])); // Print just the start of the hash
            }
            MerkleNode::Branch { left, right, hash } => {
                println!("{}Branch: hash={}", indent_str, hex::encode(&hash[0..4]));
                MerkleTree::print_node(left, indent + 1);
                MerkleTree::print_node(right, indent + 1);
            }
        }
    }
}