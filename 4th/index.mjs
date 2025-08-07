import { Keypair, Connection, LAMPORTS_PER_SOL, SystemProgram, Transaction, sendAndConfirmTransaction } from '@solana/web3.js'

// Transfer SOL 

const main = async() => {
    const sender = Keypair.generate();
    const receiver = Keypair.generate();
    const connection = new Connection("http://127.0.0.1:8899", "confirmed");


    const senderBalance1 = await connection.getBalance(sender.publicKey);
    const receiverBalance1 = await connection.getBalance(receiver.publicKey);
    console.log(senderBalance1);
    console.log(receiverBalance1);

    
    const senderAirDrop = await connection.requestAirdrop(
        sender.publicKey,
        LAMPORTS_PER_SOL,
    );
    await connection.confirmTransaction(senderAirDrop, 'confirmed');

    const transferInstruction = SystemProgram.transfer({
        fromPubkey: sender.publicKey,
        toPubkey: receiver.publicKey,
        lamports: 0.5 * LAMPORTS_PER_SOL
    });

    const transaction = new Transaction().add(transferInstruction);

    const transactionSignature = await sendAndConfirmTransaction(
        connection, 
        transaction, 
        [sender],
    )

    console.log("Transaction Signature -> ", transactionSignature);

    const senderBalance = await connection.getBalance(sender.publicKey);
    const receiverBalance = await connection.getBalance(receiver.publicKey);
    console.log(senderBalance);
    console.log(receiverBalance);
}
main();