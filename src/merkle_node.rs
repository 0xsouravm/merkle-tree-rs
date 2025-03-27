use sha2::{Digest, Sha256};
use std::fmt;

/// Represents a node in the Merkle tree
///
/// A node can be either:
/// - A leaf node containing data and its hash
/// - A branch node containing left and right children, and the hash of their combined hashes
#[derive(Clone)]
pub enum MerkleNode {
    /// A leaf node contains the original data and its hash
    Leaf {
        /// The original data
        data: Vec<u8>,
        /// The hash of the data
        hash: Vec<u8>,
    },
    /// A branch node contains left and right children and the hash of their combined hashes
    Branch {
        /// The left child node
        left: Box<MerkleNode>,
        /// The right child node
        right: Box<MerkleNode>,
        /// The hash of the combined hashes of the children
        hash: Vec<u8>,
    },
}

impl MerkleNode {
    /// Create a new leaf node from data
    ///
    /// The hash is computed using SHA-256.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to be stored in the leaf node
    ///
    /// # Returns
    ///
    /// A new leaf node containing the data and its hash
    pub fn new_leaf(data: Vec<u8>) -> Self {
        let hash = Sha256::digest(&data).to_vec();
        MerkleNode::Leaf { data, hash }
    }

    /// Create a new branch node from two child nodes
    ///
    /// The hash is computed by concatenating and hashing the hashes of the child nodes.
    ///
    /// # Arguments
    ///
    /// * `left` - The left child node
    /// * `right` - The right child node
    ///
    /// # Returns
    ///
    /// A new branch node containing the child nodes and the combined hash
    pub fn new_branch(left: MerkleNode, right: MerkleNode) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(left.hash());
        hasher.update(right.hash());
        let hash = hasher.finalize().to_vec();

        MerkleNode::Branch {
            left: Box::new(left),
            right: Box::new(right),
            hash,
        }
    }

    /// Get the hash of this node
    ///
    /// # Returns
    ///
    /// The hash of this node
    pub fn hash(&self) -> Vec<u8> {
        match self {
            MerkleNode::Leaf { hash, .. } => hash.clone(),
            MerkleNode::Branch { hash, .. } => hash.clone(),
        }
    }
}

/// Implementing the Debug trait for MerkleNode to allow printing
impl fmt::Debug for MerkleNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MerkleNode::Leaf { data, hash } => {
                write!(
                    f,
                    "Leaf {{ data: {:?}, hash: {} }}",
                    String::from_utf8_lossy(data),
                    hex::encode(hash)
                )
            }
            MerkleNode::Branch { left, right, hash } => {
                write!(
                    f,
                    "Branch {{ hash: {}, left: {:?}, right: {:?} }}",
                    hex::encode(hash),
                    left,
                    right
                )
            }
        }
    }
}
