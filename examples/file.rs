use merkleproof::MerkleTree;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;

/// This example demonstrates how to use Merkle trees for file integrity verification
fn main() -> io::Result<()> {
    println!("=== Merkle Tree File Verification Example ===\n");

    // You can use this example with any directory containing files
    let dir_path = "./examples";
    println!("Reading files from directory: {}", dir_path);

    // Read files from the directory
    let mut file_contents = Vec::new();
    let mut file_names = Vec::new();

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();
            let content = read_file(&path)?;

            println!("Added file: {} ({} bytes)", file_name, content.len());
            file_names.push(file_name);
            file_contents.push(content);
        }
    }

    if file_contents.is_empty() {
        println!("No files found in directory!");
        return Ok(());
    }

    // Create a Merkle tree from the file contents
    let tree = MerkleTree::new(file_contents.clone());
    println!("\nMerkle root hash: {}", tree.root_hash_hex());

    // Simulate verifying a specific file
    let file_index = 0; // Verify the first file
    let file_to_verify = &file_contents[file_index];
    let file_name = &file_names[file_index];

    println!("\nVerifying file: {}", file_name);

    if let Some(proof) = tree.generate_proof(file_to_verify) {
        println!("Proof generated with {} elements", proof.len());

        // Verify the proof
        let root_hash = tree.root_hash().unwrap();
        let is_valid = MerkleTree::verify_proof(file_to_verify, &proof, &root_hash);
        println!(
            "Verification result: {}",
            if is_valid { "VALID ✓" } else { "INVALID ✗" }
        );

        // Simulate tampering with the file
        println!("\nSimulating file tampering...");
        let mut tampered_content = file_to_verify.clone();
        if !tampered_content.is_empty() {
            // Modify a byte in the file content
            tampered_content[0] ^= 1;

            let is_valid = MerkleTree::verify_proof(&tampered_content, &proof, &root_hash);
            println!(
                "Verification of tampered file: {}",
                if is_valid {
                    "VALID (PROBLEM!)"
                } else {
                    "INVALID ✗ (Expected)"
                }
            );
        }
    } else {
        println!("Failed to generate proof for file");
    }

    Ok(())
}

/// Helper function to read a file into a byte vector
fn read_file(path: &Path) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
