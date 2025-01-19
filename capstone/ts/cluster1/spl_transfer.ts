import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "/home/daniel/.solana/.config/keypari.json";
import toWallet from "./wallet/toWallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const toKeyPair = Keypair.fromSecretKey(new Uint8Array(toWallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("HYNwWEDVv5izH7adxgyRTJ9uSig7nG3gqH6YuRjGd4sT");

// Recipient address
const to = toKeyPair.publicKey;

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const ataFrom = await getOrCreateAssociatedTokenAccount(
            connection, 
            keypair, 
            mint, 
            keypair.publicKey
        );
        // Get the token account of the toWallet address, and if it does not exist, create it
        const ataTo = await getOrCreateAssociatedTokenAccount(
            connection, 
            keypair, 
            mint, 
            to
        );
        // Transfer the new token to the "toTokenAccount" we just created
        const txid = await transfer(
            connection,
            keypair,
            ataFrom.address,
            ataTo.address,
            keypair.publicKey,
            1e9,
        );

        console.log(`Succesfully Minted! , Transaction Here: https://explorer.solana.com/tx/ ${txid}?cluster=devnet`);

    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();