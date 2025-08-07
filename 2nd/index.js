import { Connection, Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js'
const main = async() => {
    const keypair = Keypair.generate();
    console.log(keypair);

    const connection = new Connection("http://127.0.0.1:8899", "confirmed");
    const transaction = await connection.requestAirdrop(
        keypair.publicKey,
        LAMPORTS_PER_SOL,
    )
    await connection.confirmTransaction(transaction, 'confirmed');

    const accountInfo = await connection.getAccountInfo(
        keypair.publicKey
    )
    console.log(JSON.stringify(accountInfo, null, 2));
}
main();


// {
//   "data": {
//     "type": "Buffer",
//     "data": []
//   },
//   "executable": false,
//   "lamports": 1000000000,
//   "owner": "11111111111111111111111111111111",
//   "rentEpoch": 18446744073709552000,
//   "space": 0
// }