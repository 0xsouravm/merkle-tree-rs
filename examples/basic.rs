use merkleproof::MerkleTree;

fn main() {
    println!("=== Merkle Tree Demonstration ===\n");

    // Create a tree with transactions
    println!("Blockchain Transaction Merkle Tree Example");
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
        println!("  {}. {}", i + 1, String::from_utf8_lossy(tx));
    }

    // Create the Merkle tree
    let tree = MerkleTree::new(transactions.clone());
    println!("\nMerkle Root: {}", tree.root_hash_hex());

    // Print the tree structure
    println!("\nTree Structure:");
    tree.print_tree();

    // Generate and verify a proof
    let tx_to_verify = b"Bob sends 3 BTC to Charlie".to_vec();
    println!(
        "\nVerifying transaction: '{}'",
        String::from_utf8_lossy(&tx_to_verify)
    );

    if let Some(proof) = tree.generate_proof(&tx_to_verify) {
        println!("Proof generated with {} elements", proof.len());

        // Show the proof details
        println!("Proof details:");
        for (i, (hash, is_left)) in proof.iter().enumerate() {
            println!(
                "  Step {}: Combine with {} ({})",
                i + 1,
                hex::encode(&hash[0..4]),
                if *is_left { "left" } else { "right" }
            );
        }

        // Verify the proof
        let root_hash = tree.root_hash().unwrap();
        let is_valid = MerkleTree::verify_proof(&tx_to_verify, &proof, &root_hash);
        println!(
            "\nProof verification result: {}",
            if is_valid { "VALID ✓" } else { "INVALID ✗" }
        );

        // Try an invalid transaction
        let invalid_tx = b"Eve sends 100 BTC to Alice".to_vec();
        println!(
            "\nAttempting to verify invalid transaction: '{}'",
            String::from_utf8_lossy(&invalid_tx)
        );
        let is_valid = MerkleTree::verify_proof(&invalid_tx, &proof, &root_hash);
        println!(
            "Invalid transaction verification result: {}",
            if is_valid {
                "VALID (PROBLEM!)"
            } else {
                "INVALID ✗ (Expected)"
            }
        );
    } else {
        println!("Failed to generate proof - transaction not found in tree");
    }
}
