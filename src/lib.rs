// Main library file for the Merkle tree implementation
//
// This crate provides a complete implementation of a Merkle tree data structure,
// which is a fundamental component in many blockchain and distributed systems.

mod merkle_node;
mod merkle_tree;

// Re-export the main types and functions for external use
pub use merkle_node::MerkleNode;
pub use merkle_tree::MerkleTree;

#[cfg(test)]
mod tests;

/// A proof that a piece of data exists in a Merkle tree
///
/// Each element in the proof is:
/// - A hash value (sibling hash)
/// - A boolean flag indicating whether the sibling is on the left side
pub type MerkleProof = Vec<(Vec<u8>, bool)>;
