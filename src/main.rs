use identity::crypto::KeyPair;
use identity::iota::{Client, Document, Network, Result, TangleRef};

fn main() {
    // Create a DID Document (an identity).
    let keypair: KeyPair = KeyPair::new_ed25519().unwrap();
    let mut document: Document = Document::from_keypair(&keypair).unwrap();

    // Sign the DID Document with the default authentication key.
    document.sign(keypair.secret()).unwrap();

    println!("{}", document.to_string());
}
