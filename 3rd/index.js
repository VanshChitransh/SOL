import { Connection, PublicKey } from '@solana/web3.js'
import { getMint } from '@solana/spl-token'

const connection = new Connection(
    "https://api.mainnet-beta.solana.com",
    "confirmed",
);
// token account (program account)

// const address = new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA')
// const accountInfo = await connection.getAccountInfo(address);
// console.log(accountInfo);



// mint account(data account)
const address = new PublicKey('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v')
const accountInfo = await connection.getAccountInfo(address);
console.log(accountInfo);
console.log("---------------------------------------------------------------------------------------------------------------------------------------------------")
const mintData = await getMint(connection,address, 'confirmed');
console.log(mintData);

// to see the data of the mint account we have to deserialize it. 
