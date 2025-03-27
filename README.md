# Merkle-Tree

<div>

![Merkle Tree](https://img.shields.io/badge/Merkle%20Tree-Verification-brightgreen)
[![Crates.io](https://img.shields.io/crates/v/merkleproof.svg)](https://crates.io/crates/merkleproof)
[![Documentation](https://docs.rs/merkleproof/badge.svg)](https://docs.rs/merkleproof)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.65%2B-orange.svg)](https://www.rust-lang.org)
[![Build Status](https://img.shields.io/github/workflow/status/0xsouravm/merkle-tree-rs/CI)](https://github.com/0xsouravm/merkle-tree-rs/actions)

A robust, efficient, and well-documented implementation of Merkle trees in Rust.

[Documentation](https://docs.rs/merkleproof) | [Crates.io](https://crates.io/crates/merkleproof) | [GitHub](https://github.com/0xsouravm/merkle-tree-rs)

</div>

## Features

- **Efficient Verification**: O(log n) complexity for data verification operations
- **Cryptographically Secure**: Uses SHA-256 for secure hash computation
- **Complete API**: Tree construction, proof generation, and verification
- **Well-Tested**: Comprehensive test suite ensures reliability
- **Well-Documented**: Clear documentation with examples for all features
- **Zero Dependencies**: Minimal external dependencies (only `sha2` and `hex`)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
merkleproof = "0.1.0"
```

## Quick Start

```rust
use merkleproof::MerkleTree;

// Create a tree with your data
let data = vec![
    b"Transaction 1".to_vec(),
    b"Transaction 2".to_vec(),
    b"Transaction 3".to_vec(),
];

let tree = MerkleTree::new(data.clone());
println!("Merkle Root: {}", tree.root_hash_hex());

// Generate a proof for a specific item
if let Some(proof) = tree.generate_proof(&data[1]) {
    // Verify the proof
    let root_hash = tree.root_hash().unwrap();
    let is_valid = MerkleTree::verify_proof(&data[1], &proof, &root_hash);
    
    if is_valid {
        println!("Data verified successfully!");
    }
}
```

## What is a Merkle Tree?

A Merkle tree (hash tree) is a binary tree structure where:
- Leaf nodes contain hashes of individual data blocks
- Non-leaf nodes contain hashes of their children's hashes
- The root hash represents a cryptographic summary of all data

This structure enables efficient verification of data integrity in distributed systems:

<div align="center">

```
            Root Hash
           /        \
          /          \
     Hash(H_A+H_B)    Hash(H_C+H_D)
      /       \         /       \
     H_A      H_B      H_C      H_D
     |        |        |        |
  Data A   Data B   Data C   Data D
```

</div>

### Key Benefits

- **Efficiency**: Verify data with only log(n) hash operations
- **Data Integrity**: Detect any tampering in the dataset
- **Partial Verification**: Verify specific data without having the entire dataset

## Usage Examples

### Basic Merkle Tree

```rust
use merkleproof::MerkleTree;

// Create data items
let data_items = vec![
    b"Alice sends 5 BTC to Bob".to_vec(),
    b"Bob sends 3 BTC to Charlie".to_vec(),
    b"David sends 2 BTC to Eve".to_vec(),
];

// Create a new Merkle tree
let tree = MerkleTree::new(data_items.clone());

// Get the root hash as a hex string
println!("Merkle Root: {}", tree.root_hash_hex());
```

### Generating and Verifying Proofs

```rust
use merkleproof::MerkleTree;

let data_items = vec![
    b"Item 1".to_vec(),
    b"Item 2".to_vec(),
    b"Item 3".to_vec(),
];

let tree = MerkleTree::new(data_items.clone());
let root_hash = tree.root_hash().unwrap();

// Generate a proof for Item 2
let item_to_verify = &data_items[1];
if let Some(proof) = tree.generate_proof(item_to_verify) {
    // Verify the proof
    let is_valid = MerkleTree::verify_proof(item_to_verify, &proof, &root_hash);
    assert!(is_valid);
    
    // Tampered data will fail verification
    let mut tampered_item = item_to_verify.clone();
    tampered_item[0] ^= 1; // Flip a bit
    let is_valid = MerkleTree::verify_proof(&tampered_item, &proof, &root_hash);
    assert!(!is_valid);
}
```

## API Overview

### Core Structures

#### `MerkleNode`
Represents nodes in the Merkle tree:
- `Leaf`: Contains original data and its hash
- `Branch`: Contains left and right children and their combined hash

#### `MerkleTree`
The main structure with:
- `root`: Root node of the tree
- `leaves`: All leaf nodes for efficient proof generation

### Key Functions

#### Creating a Tree
```rust
// Creates a new Merkle tree from data items
let tree = MerkleTree::new(data_items);
```

#### Getting the Root Hash
```rust
// Get the binary root hash
let root_hash = tree.root_hash();

// Get the hexadecimal root hash string
let root_hash_hex = tree.root_hash_hex();
```

#### Generating Proofs
```rust
// Generate a proof for specific data
let proof = tree.generate_proof(&data);
```

#### Verifying Proofs
```rust
// Verify a proof against the root hash
let is_valid = MerkleTree::verify_proof(&data, &proof, &root_hash);
```

## Applications

Merkle trees are widely used in:

- **Blockchains**: Bitcoin and Ethereum use Merkle trees to efficiently verify transactions
- **Distributed File Systems**: IPFS, Filecoin, and similar systems use Merkle trees for content addressing
- **Git**: Version control systems use Merkle-like structures for efficient data storage
- **Certificate Transparency**: Merkle trees help efficiently audit digital certificates
- **P2P Networks**: Efficiently verify data integrity in distributed networks

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
Here are some specific areas where contributions would be particularly valuable:

- **Performance Optimizations**: Improving the speed of tree construction and proof generation
- **Serialization Support**: Adding serde support for serializing/deserializing trees and proofs
- **Alternative Hash Algorithms**: Supporting pluggable hash algorithms beyond SHA-256
- **Sparse Merkle Trees**: Implementing support for sparse Merkle trees
- **Incremental Tree Updates**: Supporting efficient updates to existing trees
- **Concurrent Processing**: Adding support for parallel tree construction with large datasets
- **Documentation**: Improving examples, especially for blockchain and distributed systems use cases
- **Benchmarking**: Creating more comprehensive benchmark suites
- **WebAssembly Support**: Ensuring compatibility with WASM for browser-based applications

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Inspired by blockchain implementations of Merkle trees
- Thanks to the Rust cryptography community for the excellent SHA-256 implementations