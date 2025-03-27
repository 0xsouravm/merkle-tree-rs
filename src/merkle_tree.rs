use crate::merkle_node::MerkleNode;
use crate::MerkleProof;
use sha2::{Digest, Sha256};

/// The main Merkle tree structure
///
/// A Merkle tree is a binary tree where:
/// - Each leaf node contains the hash of a data block
/// - Each non-leaf node contains the hash of its two children
/// - The root node represents a cryptographic summary of all data in the tree
#[derive(Clone)]
pub struct MerkleTree {
    /// The root node of the tree (None if the tree is empty)
    root: Option<MerkleNode>,
    /// A vector of all leaf nodes for easier proof generation
    leaves: Vec<MerkleNode>,
}

impl MerkleTree {
    /// Create a new Merkle tree from a list of data items
    ///
    /// # Arguments
    ///
    /// * `data_items` - A vector of data items to include in the tree
    ///
    /// # Returns
    ///
    /// A new Merkle tree containing the data items
    pub fn new(data_items: Vec<Vec<u8>>) -> Self {
        if data_items.is_empty() {
            return MerkleTree {
                root: None,
                leaves: Vec::new(),
            };
        }

        // Create leaf nodes
        let mut leaves: Vec<MerkleNode> = data_items
            .into_iter()
            .map(MerkleNode::new_leaf)
            .collect();

        // Special case for single node - don't duplicate it
        if leaves.len() == 1 {
            let leaf_copy = leaves[0].clone();
            return MerkleTree {
                root: Some(leaf_copy),
                leaves,
            };
        }

        // If odd number of leaves, duplicate the last one
        if leaves.len() % 2 == 1 {
            leaves.push(leaves.last().unwrap().clone());
        }

        let leaves_copy = leaves.clone();
        let root = Some(MerkleTree::build_tree(leaves));

        MerkleTree {
            root,
            leaves: leaves_copy,
        }
    }

    /// Build the tree recursively
    ///
    /// # Arguments
    ///
    /// * `nodes` - A vector of nodes to build the tree from
    ///
    /// # Returns
    ///
    /// The root node of the tree
    fn build_tree(nodes: Vec<MerkleNode>) -> MerkleNode {
        if nodes.len() == 1 {
            return nodes[0].clone();
        }

        let mut next_level = Vec::new();

        // Process pairs of nodes
        for chunk in nodes.chunks(2) {
            if chunk.len() == 2 {
                let branch = MerkleNode::new_branch(chunk[0].clone(), chunk[1].clone());
                next_level.push(branch);
            } else {
                // Should not happen if we handle odd number of leaves correctly
                next_level.push(chunk[0].clone());
            }
        }

        // Recurse to the next level
        MerkleTree::build_tree(next_level)
    }

    /// Get the root hash of the tree
    ///
    /// # Returns
    ///
    /// The root hash of the tree, or None if the tree is empty
    pub fn root_hash(&self) -> Option<Vec<u8>> {
        self.root.as_ref().map(|node| node.hash())
    }

    /// Get the root hash of the tree as a hexadecimal string
    ///
    /// # Returns
    ///
    /// The root hash as a hexadecimal string, or "Empty tree" if the tree is empty
    pub fn root_hash_hex(&self) -> String {
        match self.root_hash() {
            Some(hash) => hex::encode(&hash),
            None => String::from("Empty tree"),
        }
    }

    /// Generate a proof for a specific data item
    ///
    /// A proof consists of a list of sibling hashes and their positions
    /// (left or right) along the path from the leaf node to the root.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to generate a proof for
    ///
    /// # Returns
    ///
    /// A proof that the data exists in the tree, or None if the data is not found
    pub fn generate_proof(&self, data: &[u8]) -> Option<MerkleProof> {
        // Find the leaf node
        let target_hash = Sha256::digest(data).to_vec();
        let leaf_index = self.leaves.iter().position(|node| match node {
            MerkleNode::Leaf { hash, .. } => hash == &target_hash,
            _ => false,
        })?;

        let mut proof = Vec::new();
        let mut index = leaf_index;
        let mut level_size = self.leaves.len();
        let mut level_nodes = self.leaves.clone();

        while level_size > 1 {
            let is_left = index % 2 == 0;
            let sibling_idx = if is_left { index + 1 } else { index - 1 };

            // Handle edge case where we duplicated the last leaf
            if sibling_idx < level_nodes.len() {
                proof.push((level_nodes[sibling_idx].hash(), !is_left));
            }

            // Move to parent level
            index /= 2;
            level_size = (level_size + 1) / 2;

            // Build the next level
            let mut next_level = Vec::new();
            for chunk in level_nodes.chunks(2) {
                if chunk.len() == 2 {
                    let branch = MerkleNode::new_branch(chunk[0].clone(), chunk[1].clone());
                    next_level.push(branch);
                } else {
                    next_level.push(chunk[0].clone());
                }
            }
            level_nodes = next_level;
        }

        Some(proof)
    }

    /// Verify a proof against the root hash
    ///
    /// # Arguments
    ///
    /// * `data` - The data to verify
    /// * `proof` - The proof to verify
    /// * `root_hash` - The root hash to verify against
    ///
    /// # Returns
    ///
    /// True if the proof is valid, false otherwise
    pub fn verify_proof(data: &[u8], proof: &MerkleProof, root_hash: &[u8]) -> bool {
        let mut current_hash = Sha256::digest(data).to_vec();

        for (sibling_hash, is_left) in proof {
            let mut hasher = Sha256::new();

            if *is_left {
                hasher.update(sibling_hash);
                hasher.update(&current_hash);
            } else {
                hasher.update(&current_hash);
                hasher.update(sibling_hash);
            }

            current_hash = hasher.finalize().to_vec();
        }

        current_hash == root_hash
    }

    /// Get the number of leaves in the tree
    ///
    /// # Returns
    ///
    /// The number of leaf nodes in the tree
    pub fn len(&self) -> usize {
        self.leaves.len()
    }

    /// Check if the tree is empty
    ///
    /// # Returns
    ///
    /// True if the tree has no nodes, false otherwise
    pub fn is_empty(&self) -> bool {
        self.leaves.is_empty()
    }

    /// Print the tree structure for debugging
    ///
    /// This function prints the tree structure to standard output.
    pub fn print_tree(&self) {
        if let Some(root) = &self.root {
            println!("Merkle Tree Structure:");
            Self::print_node(root, 0);
        } else {
            println!("Empty tree");
        }
    }

    /// Helper function to print a node and its children recursively
    ///
    /// # Arguments
    ///
    /// * `node` - The node to print
    /// * `indent` - The indentation level (for pretty-printing)
    fn print_node(node: &MerkleNode, indent: usize) {
        let indent_str = " ".repeat(indent * 2);

        match node {
            MerkleNode::Leaf { data, hash } => {
                println!(
                    "{}Leaf: data={:?}, hash={}",
                    indent_str,
                    String::from_utf8_lossy(data),
                    hex::encode(&hash[0..4])
                ); // Print just the start of the hash
            }
            MerkleNode::Branch { left, right, hash } => {
                println!("{}Branch: hash={}", indent_str, hex::encode(&hash[0..4]));
                Self::print_node(left, indent + 1);
                Self::print_node(right, indent + 1);
            }
        }
    }
}
