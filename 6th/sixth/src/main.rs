// use solana_sdk::signer::{keypair::Keypair, Signer};
// #[tokio::main]



// async fn main(){
//     let keypair = Keypair::new();
//     println!("This is keypair -> {:?}", keypair);
//     // explore the underline structure of the keypair object. 
//     // check if it does have something as edward's curve 
//     println!("Public Key: {}", keypair.pubkey());
//     println!("Private Key: {:?}", keypair.to_bytes());
// }



// use solana_sdk::pubkey;
// use solana_sdk::pubkey::Pubkey;
// #[tokio::main]
// async fn main(){
//     let program_address = pubkey!("11111111111111111111111111111111");
//     let seeds = [b"Hello 1nsh".as_ref()];
//     let (pda, bump) = Pubkey::find_program_address(&seeds, &program_address);
//     println!("This is pda -> {}", pda);
//     println!("This is bump -> {}", bump);
//     // what is bump? Explore it a bit.
// }




// use anyhow::Result;
// use solana_client::nonblocking::rpc_client::RpcClient;
// use solana_sdk::{
//     commitment_config::CommitmentConfig,
//     native_token::LAMPORTS_PER_SOL,
//     signer::{keypair::Keypair, Signer},
// };

// #[tokio::main]
// async fn main() -> Result<()>{
//     let keypair = Keypair::new();
//     println!("Public key: {}", keypair.pubkey());


//     let connection = RpcClient::new_with_commitment(
//         "http://localhost:8899".to_string(),
//         CommitmentConfig::confirmed(),
//     );

//     let signature = connection
//         .request_airdrop(&keypair.pubkey(), LAMPORTS_PER_SOL)
//         .await?;

//     connection.confirm_transaction(&signature).await?;

//     let account_info = connection.get_account(&keypair.pubkey()).await?;
//     println!("{:?}", account_info);
//     Ok(())
// }

// TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA

// use anyhow::Result;
// use solana_client::nonblocking::rpc_client::RpcClient;
// use solana_sdk::{
//     commitment_config::CommitmentConfig, 
//     pubkey,
// };

// #[tokio::main]
// async fn main() -> Result<()>{
//     let connection = RpcClient::new_with_commitment(
//         "http://localhost:8899".to_string(),
//         CommitmentConfig::confirmed(),
//     );

//     let program_id = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
//     let account_info = connection.get_account(&program_id).await?;
//     println!("{:?}", account_info);
//     Ok(())
// }




use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    program_pack: Pack, 
    signature::{Keypair, Signer},
    transaction::Transaction,
    system_instruction::create_account,
    native_token::LAMPORTS_PER_SOL,
};
use spl_token_2022::{id as token_2022_program_id, instruction::initialize_mint, state::Mint};

#[tokio::main]
async fn main() -> Result<()>{
    let client = RpcClient::new_with_commitment(
        "http://localhost:8899".to_string(),
        CommitmentConfig::confirmed(),
    );

    let recent_blockhash = client.get_latest_blockhash().await?;
    let fee_payer = Keypair::new();
    let airdrop_signature = client
        .request_airdrop(&fee_payer.pubkey(), LAMPORTS_PER_SOL)
        .await?;
    client.confirm_transaction(&airdrop_signature);


    loop{
        let confirmed = client.confirm_transaction(&airdrop_signature).await?;
        if confirmed{
            break;
        }
    }

    let mint = Keypair::new();
    let space = Mint::LEN;
    let rent = client.get_minimum_balance_for_rent_exemption(space).await?;

    let create_account_instruction = create_account(
        &fee_payer.pubkey(),
        &mint.payer(),
        rent,
        space as u64, 
        &token_2022_program_id(),
    );

    let initialize_mint_instructions = initialize_mint(
        &token_2022_program_id, 
        &mint.pubkey(),
        &fee_payer.pubkey(),
        Some(&fee_payer.pubkey()),
        9,
    )?;

    let transaction = Transaction::new_signed_with_payer(
        &[create_account_instruction, initialize_mint_instructions], 
        Some(&fee_payer.pubkey()),
        &[&fee_payer, &mint],
        recent_blockhash, 
    );

    let transaction_signature = client.send_and_confirm_transaction(&transaction).await?;
    println!("Mint address: {}", mint.pubkey());
    println!("Transaction signature: {}", transaction_signature);

    let account_info = client.get_account(&mint.pubkey()).await?;
    println!("{:#?}", account_info);

    let mint_account = Mint::unpack(&account_info.data)?;
    println!("{:#?}", mint_account);

    Ok(())
}