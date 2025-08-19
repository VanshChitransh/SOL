use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use tokio;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let program_address = pubkey!("11111111111111111111111111111111");
    let seeds = [b"HelloWorld".as_ref()];

    // let seeds = ["HelloWorld".as_bytes()];
    // as_ref() converts the byte string literal to a slice of slices. 
    // as_bytes() directly convert the string literal to bytes.
    let a = Pubkey::find_program_address(&seeds, &program_address);
    // seeds is a slice of bytes
    // seeds = [seed1_bytes, seed2_bytes, seed3_bytes, ...] 
    // we need to pass a &[&[u8]] type input in it. 

    // it will return a tuple of two elements in it. 
    
    // println!("PDA -> {}", pda);
    // println!("bump -> {}", bump);
    println!("This -> {:?}", a);
}

