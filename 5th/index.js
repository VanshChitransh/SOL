// Token creation

import { Connection, Keypair, SystemProgram, Transaction, sendAndConfirmTransaction, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { MINT_SIZE, TOKEN_2022_PROGRAM_ID, createInitializeMint2Instruction, getMinimumBalanceForRentExemptMint, getMint } from "@solana/spl-token";

const connection = new Connection('http://127.0.0.1:8899', 'confirmed');
const wallet = Keypair.generate();
const transaction = await connection.requestAirdrop(
    wallet.publicKey, 
    LAMPORTS_PER_SOL,
);
await connection.confirmTransaction(transaction, 'confirmed');
const mint = Keypair.generate();
const rentForMint = await getMinimumBalanceForRentExemptMint(connection);

const createAccountInstruction = SystemProgram.createAccount({
    fromPubkey: wallet.publicKey,
    newAccountPubkey: mint.publicKey, 
    space: MINT_SIZE, 
    lamports: rentForMint, 
    programId: TOKEN_2022_PROGRAM_ID,
});

const initializeMintInstruction = createInitializeMint2Instruction(
    mint.publicKey, 
    2, 
    wallet.publicKey, 
    wallet.publicKey, 
    TOKEN_2022_PROGRAM_ID,
);

const finalTransaction = new Transaction().add(
    createAccountInstruction, 
    initializeMintInstruction
);

const transactionSignature = await sendAndConfirmTransaction(
    connection, 
    finalTransaction, 
    [
        wallet, 
        mint
    ]
);
console.log("This is the transaction signature -> ", transactionSignature);

const mintData = await getMint(
    connection, 
    mint.publicKey, 
    "confirmed",
    TOKEN_2022_PROGRAM_ID,
)

console.log(mintData)