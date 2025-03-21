# Merkle Tree Implementation in Rust

This repository contains a simple but complete implementation of a Merkle tree data structure in Rust. Merkle trees are fundamental data structures used in distributed systems and blockchains for efficiently verifying data integrity.

## What is a Merkle Tree?

A Merkle tree (or hash tree) is a binary tree where:
- Each leaf node contains the hash of a data block
- Each non-leaf node contains the hash of its two child nodes combined
- The root hash represents a cryptographic summary of all data in the tree

This structure allows for efficient verification of large datasets - to verify if data belongs in the tree, you only need log(n) hashes rather than checking the entire dataset.

## Features

- Create a Merkle tree from a list of data items
- Generate cryptographic proofs for data verification
- Verify proofs against the root hash
- Support for arbitrary binary data
- Automatic handling of trees with odd numbers of elements
- Debug visualization of the tree structure

## API Overview

### Core Structures

#### `MerkleNode`
An enum representing either a leaf or branch node in the tree:
- `Leaf`: Contains the original data and its hash
- `Branch`: Contains left and right child nodes and the hash of their combined hashes

#### `MerkleTree`
The main structure that holds:
- `root`: The root node of the tree
- `leaves`: A vector of all leaf nodes for easier proof generation

### Key Functions

#### `MerkleTree::new(data_items: Vec<Vec<u8>>) -> Self`
Creates a new Merkle tree from a list of data items.
- Converts each data item to a leaf node by hashing it
- Handles odd numbers of leaves by duplicating the last leaf
- Builds the tree recursively by combining pairs of nodes
- Returns a new `MerkleTree` instance

#### `MerkleTree::root_hash() -> Option<Vec<u8>>`
Returns the root hash of the tree, or `None` if the tree is empty.

#### `MerkleTree::root_hash_hex() -> String`
Returns the root hash as a hexadecimal string, or "Empty tree" if the tree is empty.

#### `MerkleTree::generate_proof(data: &[u8]) -> Option<Vec<(Vec<u8>, bool)>>`
Generates a proof for the given data:
- Finds the leaf node containing the data
- Collects sibling hashes along the path to the root
- For each sibling, records whether it's a left or right sibling
- Returns `None` if the data isn't found in the tree

#### `MerkleTree::verify_proof(data: &[u8], proof: &[(Vec<u8>, bool)], root_hash: &[u8]) -> bool`
Verifies a proof against the root hash:
- Hashes the data to get the leaf hash
- Combines this hash with each sibling hash in the correct order
- Returns true if the final hash matches the root hash

#### `MerkleTree::print_tree()`
Prints the tree structure for debugging, showing nodes and their relationships.

## Usage Example

```rust
use merkle_tree::MerkleTree;

fn main() {
    // Create sample data (could be transactions, files, etc.)
    let data_items = vec![
        b"Alice sends 5 BTC to Bob".to_vec(),
        b"Bob sends 3 BTC to Charlie".to_vec(),
        b"David sends 2 BTC to Eve".to_vec(),
        b"Charlie sends 1 BTC to Alice".to_vec(),
    ];
    
    // Create a new Merkle tree
    let tree = MerkleTree::new(data_items.clone());
    
    // Get the root hash
    println!("Merkle Root: {}", tree.root_hash_hex());
    
    // Generate a proof for a specific item
    let item_to_verify = b"Bob sends 3 BTC to Charlie".to_vec();
    if let Some(proof) = tree.generate_proof(&item_to_verify) {
        // Verify the proof
        let root_hash = tree.root_hash().unwrap();
        let is_valid = MerkleTree::verify_proof(&item_to_verify, &proof, &root_hash);
        
        if is_valid {
            println!("Data verified successfully!");
        } else {
            println!("Invalid proof!");
        }
    }
}
```

## How It Works

### Tree Construction

1. Each data item is hashed to create a leaf node
2. Pairs of nodes are combined by hashing their hashes together
3. This process continues until only one node remains (the root)
4. If there's an odd number of nodes at any level, the last node is duplicated

#### Pseudocode:
```
function BuildTree(data_items):
    if data_items is empty:
        return EmptyTree
    
    leaves = []
    for each item in data_items:
        leaf = CreateLeafNode(Hash(item), item)
        leaves.append(leaf)
    
    // Duplicate last leaf if odd number of leaves
    if length(leaves) is odd:
        leaves.append(copy of last leaf)
    
    return BuildTreeFromNodes(leaves)

function BuildTreeFromNodes(nodes):
    if nodes contains only 1 node:
        return that node
    
    next_level = []
    for each pair (i, i+1) of nodes:
        combined_hash = Hash(nodes[i].hash + nodes[i+1].hash)
        branch = CreateBranchNode(combined_hash, nodes[i], nodes[i+1])
        next_level.append(branch)
    
    return BuildTreeFromNodes(next_level)
```

### Proof Generation

To generate a proof that a piece of data exists in the tree:

1. Find the leaf node containing the data
2. Collect the sibling hashes along the path from leaf to root
3. For each sibling, record whether it's a left or right sibling
4. The proof consists of these sibling hashes and their positions

#### Pseudocode:
```
function GenerateProof(tree, data):
    target_hash = Hash(data)
    leaf_index = find index of leaf with hash == target_hash
    if leaf not found:
        return null
    
    proof = []
    index = leaf_index
    level_size = number of leaves
    current_level_nodes = copy of leaves
    
    while level_size > 1:
        is_left = (index % 2 == 0)  // Even indices are left children
        sibling_index = is_left ? index + 1 : index - 1
        
        if sibling_index is valid:
            // Record sibling hash and whether it's on the left
            proof.append(current_level_nodes[sibling_index].hash, !is_left)
        
        // Move up to parent
        index = floor(index / 2)
        level_size = ceil(level_size / 2)
        
        // Build next level by combining pairs
        next_level = []
        for each pair of nodes in current_level_nodes:
            branch = combine nodes into parent
            next_level.append(branch)
        
        current_level_nodes = next_level
    
    return proof
```

### Proof Verification

To verify a proof:

1. Hash the data to get the leaf hash
2. Combine this hash with each sibling hash in the proof, in the correct order
3. If the final hash matches the root hash, the proof is valid

#### Pseudocode:
```
function VerifyProof(data, proof, root_hash):
    current_hash = Hash(data)
    
    for each (sibling_hash, is_left) in proof:
        if is_left:  // Sibling is on the left
            next_hash = Hash(sibling_hash + current_hash)
        else:  // Sibling is on the right
            next_hash = Hash(current_hash + sibling_hash)
        
        current_hash = next_hash
    
    return current_hash == root_hash
```

## Dependencies

- `sha2`: For SHA-256 cryptographic hash functions
- `hex`: For encoding binary hashes as hexadecimal strings

## Applications

Merkle trees are used in:

- Blockchains (Bitcoin, Ethereum) for verifying transactions
- Distributed file systems for integrity verification
- Certificate transparency logs
- Peer-to-peer networks for data verification
- Git and other version control systems

## License

This project is open source and available under the MIT License.