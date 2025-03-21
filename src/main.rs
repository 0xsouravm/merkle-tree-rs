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

impl MerkleNode {
    // Create a leaf node
    fn new_leaf(data: Vec<u8>) -> Self {
        let hash = Sha256::digest(&data).to_vec();
        MerkleNode::Leaf { data, hash }
    }
    
    // Create a branch node from two child nodes
    fn new_branch(left: MerkleNode, right: MerkleNode) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(&left.hash());
        hasher.update(&right.hash());
        let hash = hasher.finalize().to_vec();
        
        MerkleNode::Branch {
            left: Box::new(left),
            right: Box::new(right),
            hash,
        }
    }
    
    // Get the hash of this node
    fn hash(&self) -> Vec<u8> {
        match self {
            MerkleNode::Leaf { hash, .. } => hash.clone(),
            MerkleNode::Branch { hash, .. } => hash.clone(),
        }
    }
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

// The main MerkleTree structure
pub struct MerkleTree {
    root: Option<MerkleNode>,
    leaves: Vec<MerkleNode>,
}

impl MerkleTree {
    // Create a new Merkle tree from a list of data items
    pub fn new(data_items: Vec<Vec<u8>>) -> Self {
        if data_items.is_empty() {
            return MerkleTree { root: None, leaves: Vec::new() };
        }
        
        // Create leaf nodes
        let mut leaves: Vec<MerkleNode> = data_items.into_iter()
            .map(|data| MerkleNode::new_leaf(data))
            .collect();
        
        // If odd number of leaves, duplicate the last one
        if leaves.len() % 2 == 1 {
            leaves.push(leaves.last().unwrap().clone());
        }
        
        let leaves_copy = leaves.clone();
        let root = Some(MerkleTree::build_tree(leaves));
        
        MerkleTree { root, leaves: leaves_copy }
    }
    
    // Build the tree recursively
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
    
    // Get the root hash
    pub fn root_hash(&self) -> Option<Vec<u8>> {
        self.root.as_ref().map(|node| node.hash())
    }
    
    // Get the root hash as a hex string
    pub fn root_hash_hex(&self) -> String {
        match self.root_hash() {
            Some(hash) => hex::encode(&hash),
            None => String::from("Empty tree"),
        }
    }
    
    // Generate a proof for a specific leaf
    pub fn generate_proof(&self, data: &[u8]) -> Option<Vec<(Vec<u8>, bool)>> {
        // Find the leaf node
        let target_hash = Sha256::digest(data).to_vec();
        let leaf_index = self.leaves.iter().position(|node| match node {
            MerkleNode::Leaf { hash, .. } => hash == &target_hash,
            _ => false,
        })?;

        println!("LEAF_INDEX: {leaf_index}");
        
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
    
    // Verify a proof
    pub fn verify_proof(data: &[u8], proof: &[(Vec<u8>, bool)], root_hash: &[u8]) -> bool {
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
            println!("CURRENT_HASH: {:?}", hex::encode(&current_hash[0..4]));
        }
        
        current_hash == root_hash
    }
    
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

fn main() {
    println!("=== Merkle Tree Demonstration ===\n");
    
    // Example 1: Create a tree with transactions
    println!("Example 1: Blockchain Transaction Merkle Tree");
    let transactions = vec![
        b"Alice sends 5 BTC to Bob".to_vec(),
        b"Bob sends 3 BTC to Charlie".to_vec(),
        b"David sends 2 BTC to Eve".to_vec(),
        b"Charlie sends 1 BTC to Alice".to_vec(),
        b"Eve sends 0.5 BTC to David".to_vec(),
    ];
    
    // Print the transactions
    println!("Transactions:");
    for (i, tx) in transactions.iter().enumerate() {
        println!("  {}. {}", i+1, String::from_utf8_lossy(tx));
    }
    
    // Create the Merkle tree
    let tree = MerkleTree::new(transactions.clone());
    println!("\nMerkle Root: {}", tree.root_hash_hex());
    
    // Print the tree structure
    println!("\nTree Structure:");
    tree.print_tree();
    
    // Generate and verify a proof
    let tx_to_verify = b"Bob sends 3 BTC to Charlie".to_vec();
    println!("\nVerifying transaction: '{}'", String::from_utf8_lossy(&tx_to_verify));
    
    if let Some(proof) = tree.generate_proof(&tx_to_verify) {
        println!("Proof generated with {} elements", proof.len());
        
        // Show the proof details
        println!("Proof details:");
        for (i, (hash, is_left)) in proof.iter().enumerate() {
            println!("  Step {}: Combine with {} ({})", 
                     i+1, 
                     hex::encode(&hash[0..4]), 
                     if *is_left { "left" } else { "right" });
        }
        
        // Verify the proof
        let root_hash = tree.root_hash().unwrap();
        let is_valid = MerkleTree::verify_proof(&tx_to_verify, &proof, &root_hash);
        println!("\nProof verification result: {}", if is_valid { "VALID ✓" } else { "INVALID ✗" });
        
        // Try an invalid transaction
        let invalid_tx = b"Eve sends 100 BTC to Alice".to_vec();
        println!("\nAttempting to verify invalid transaction: '{}'", 
                 String::from_utf8_lossy(&invalid_tx));
        let is_valid = MerkleTree::verify_proof(&invalid_tx, &proof, &root_hash);
        println!("Invalid transaction verification result: {}", 
                 if is_valid { "VALID (PROBLEM!)" } else { "INVALID ✗ (Expected)" });
    } else {
        println!("Failed to generate proof - transaction not found in tree");
    }
}