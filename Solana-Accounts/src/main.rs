use solana_sdk::signer::{keypair::Keypair, Signer};
use tokio;

#[tokio::main]
async fn main(){
    let keypair = Keypair::new();
    println!("{:?}", keypair);
    println!();
    println!("Public key-> {}", keypair.pubkey());
    println!("Private key -> {:?}", keypair.to_bytes());
}

// [122, 84, 235, 220, 252, 212, 161, 67, 219, 75, 68, 177, 153, 225, 47, 71, 63, 145, 201, 20, 23, 203, 117, 250, 183, 168, 113, 120, 13, 10, 42, 66]

// [103, 112, 50, 26, 36, 169, 159, 218, 36, 251, 170, 224, 235, 117, 191, 5, 56, 101, 33, 174, 187, 64, 85, 163, 91, 145, 96, 188, 75, 107, 11, 114, 
// 122, 84, 235, 220, 252, 212, 161, 67, 219, 75, 68, 177, 153, 225, 47, 71, 63, 145, 201, 20, 23, 203, 117, 250, 183, 168, 113, 120, 13, 10, 42, 66]




// Keypair::new() generates a 32-byte private key seed.
// The Ed25519 math uses that seed to compute a single public key point on the Edwards curve (not multiple points). So for each private key, there’s exactly one corresponding public key.
// keypair.pubkey() just returns that public key, usually Base58-encoded.
// keypair.to_bytes() returns a 64-byte array:
// First 32 bytes: the private key seed
// Next 32 bytes: the public key derived from that seed
// So it’s private key first, then public key, not the other way around.