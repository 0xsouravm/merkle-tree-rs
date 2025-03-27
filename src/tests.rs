#[cfg(test)]
mod tests {
    use crate::{MerkleProof, MerkleTree};
    use sha2::{Digest, Sha256};

    // Helper function to create test data
    fn create_test_data(count: usize) -> Vec<Vec<u8>> {
        (0..count)
            .map(|i| format!("Test data {}", i).into_bytes())
            .collect()
    }

    #[test]
    fn test_empty_tree() {
        let tree = MerkleTree::new(Vec::new());
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
        assert!(tree.root_hash().is_none());
        assert_eq!(tree.root_hash_hex(), "Empty tree");
    }

    #[test]
    fn test_single_node_tree() {
        let data = vec![b"Single node".to_vec()];
        let tree = MerkleTree::new(data.clone());

        assert!(!tree.is_empty());
        assert_eq!(tree.len(), 1);

        // The root hash should be the hash of the single data item
        let expected_hash = Sha256::digest(&data[0]).to_vec();
        assert_eq!(tree.root_hash().unwrap(), expected_hash);
    }

    #[test]
    fn test_multiple_nodes_tree() {
        let data = create_test_data(4);
        let tree = MerkleTree::new(data.clone());

        assert_eq!(tree.len(), 4);
        assert!(tree.root_hash().is_some());

        // Manually compute what the root hash should be
        let hash0 = Sha256::digest(&data[0]).to_vec();
        let hash1 = Sha256::digest(&data[1]).to_vec();
        let hash2 = Sha256::digest(&data[2]).to_vec();
        let hash3 = Sha256::digest(&data[3]).to_vec();

        let mut hasher = Sha256::new();
        hasher.update(&hash0);
        hasher.update(&hash1);
        let hash01 = hasher.finalize().to_vec();

        let mut hasher = Sha256::new();
        hasher.update(&hash2);
        hasher.update(&hash3);
        let hash23 = hasher.finalize().to_vec();

        let mut hasher = Sha256::new();
        hasher.update(&hash01);
        hasher.update(&hash23);
        let expected_root = hasher.finalize().to_vec();

        assert_eq!(tree.root_hash().unwrap(), expected_root);
    }

    #[test]
    fn test_odd_number_of_nodes() {
        let data = create_test_data(3);
        let tree = MerkleTree::new(data.clone());

        // Since we duplicate the last node, the tree should have 4 leaves
        assert_eq!(tree.len(), 4);

        // Manually compute what the root hash should be, with the last item duplicated
        let hash0 = Sha256::digest(&data[0]).to_vec();
        let hash1 = Sha256::digest(&data[1]).to_vec();
        let hash2 = Sha256::digest(&data[2]).to_vec();
        let hash3 = hash2.clone(); // Duplicated

        let mut hasher = Sha256::new();
        hasher.update(&hash0);
        hasher.update(&hash1);
        let hash01 = hasher.finalize().to_vec();

        let mut hasher = Sha256::new();
        hasher.update(&hash2);
        hasher.update(&hash3);
        let hash23 = hasher.finalize().to_vec();

        let mut hasher = Sha256::new();
        hasher.update(&hash01);
        hasher.update(&hash23);
        let expected_root = hasher.finalize().to_vec();

        assert_eq!(tree.root_hash().unwrap(), expected_root);
    }

    #[test]
    fn test_proof_generation_and_verification() {
        let data = create_test_data(8);
        let tree = MerkleTree::new(data.clone());

        // Generate proof for each item
        for (i, item) in data.iter().enumerate() {
            let proof = tree.generate_proof(item).unwrap();

            // Verify the proof
            let root_hash = tree.root_hash().unwrap();
            let is_valid = MerkleTree::verify_proof(item, &proof, &root_hash);

            assert!(is_valid, "Proof for item {} should be valid", i);
        }
    }

    #[test]
    fn test_proof_verification_fails_for_tampered_data() {
        let data = create_test_data(8);
        let tree = MerkleTree::new(data.clone());

        // Generate proof for an item
        let item = &data[3];
        let proof = tree.generate_proof(item).unwrap();
        let root_hash = tree.root_hash().unwrap();

        // Tamper with the data
        let mut tampered_item = item.clone();
        tampered_item[0] ^= 1; // Flip a bit

        // Verify should fail for tampered data
        let is_valid = MerkleTree::verify_proof(&tampered_item, &proof, &root_hash);
        assert!(!is_valid, "Proof should not be valid for tampered data");
    }

    #[test]
    fn test_proof_verification_fails_for_tampered_proof() {
        let data = create_test_data(8);
        let tree = MerkleTree::new(data.clone());

        // Generate proof for an item
        let item = &data[3];
        let mut proof = tree.generate_proof(item).unwrap();
        let root_hash = tree.root_hash().unwrap();

        // Tamper with the proof
        if !proof.is_empty() {
            let mut tampered_hash = proof[0].0.clone();
            tampered_hash[0] ^= 1; // Flip a bit
            proof[0] = (tampered_hash, proof[0].1);
        }

        // Verify should fail for tampered proof
        let is_valid = MerkleTree::verify_proof(item, &proof, &root_hash);
        assert!(!is_valid, "Proof should not be valid when tampered with");
    }

    #[test]
    fn test_large_tree() {
        // Create a larger tree to test performance and correctness
        let data = create_test_data(1000);
        let tree = MerkleTree::new(data.clone());

        // Verify the tree has the correct number of leaves
        assert_eq!(tree.len(), 1000);

        // Test proof generation and verification with a random item
        let item = &data[456]; // Arbitrary index
        let proof = tree.generate_proof(item).unwrap();
        let root_hash = tree.root_hash().unwrap();

        let is_valid = MerkleTree::verify_proof(item, &proof, &root_hash);
        assert!(is_valid, "Proof should be valid for large tree");

        // Calculate the maximum proof length (should be log2(n) rounded up)
        let max_proof_length = (1000 as f64).log2().ceil() as usize;
        assert!(
            proof.len() <= max_proof_length,
            "Proof length {} should not exceed log2(n) = {}",
            proof.len(),
            max_proof_length
        );
    }

    #[test]
    fn test_proof_for_nonexistent_data() {
        let data = create_test_data(8);
        let tree = MerkleTree::new(data.clone());

        // Try to generate proof for data that doesn't exist in the tree
        let nonexistent_data = b"This data doesn't exist".to_vec();
        let proof = tree.generate_proof(&nonexistent_data);

        assert!(proof.is_none(), "Proof should be None for nonexistent data");
    }

    #[test]
    fn test_tree_clone() {
        let data = create_test_data(8);
        let tree = MerkleTree::new(data.clone());
        let tree_clone = tree.clone();

        assert_eq!(tree.root_hash(), tree_clone.root_hash());
        assert_eq!(tree.len(), tree_clone.len());
    }

    #[test]
    fn test_different_trees_produce_different_roots() {
        let data1 = create_test_data(8);
        let tree1 = MerkleTree::new(data1.clone());

        let mut data2 = data1.clone();
        data2[0] = b"Different data".to_vec();
        let tree2 = MerkleTree::new(data2.clone());

        assert_ne!(tree1.root_hash(), tree2.root_hash());
    }

    #[test]
    fn test_proof_cross_verification_fails() {
        // Create two trees with different data
        let data1 = create_test_data(8);
        let tree1 = MerkleTree::new(data1.clone());

        let mut data2 = data1.clone();
        data2[3] = b"Different data".to_vec();
        let tree2 = MerkleTree::new(data2.clone());

        // Generate proof from first tree
        let item = &data1[2]; // An item that's the same in both trees
        let proof1 = tree1.generate_proof(item).unwrap();
        let root_hash2 = tree2.root_hash().unwrap();

        // Verify should fail when using proof from tree1 against root of tree2
        let is_valid = MerkleTree::verify_proof(item, &proof1, &root_hash2);
        assert!(!is_valid, "Cross-tree verification should fail");
    }

    #[test]
    fn test_merkle_proof_type() {
        // Test that the MerkleProof type alias works correctly
        let data = create_test_data(4);
        let tree = MerkleTree::new(data.clone());

        let proof: Option<MerkleProof> = tree.generate_proof(&data[0]);
        assert!(proof.is_some());

        let root_hash = tree.root_hash().unwrap();
        let is_valid = MerkleTree::verify_proof(&data[0], &proof.unwrap(), &root_hash);
        assert!(is_valid);
    }
}
