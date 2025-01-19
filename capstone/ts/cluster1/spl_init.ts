import { Keypair, Connection, Commitment } from "@solana/web3.js";
import { createMint } from '@solana/spl-token';
import wallet from "/home/daniel/.solana/.config/localwallet.json";

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

(async () => {
    const balance = await connection.getBalance(keypair.publicKey);
    console.log(`Balance: ${balance / 1e9} SOL`);
})();

(async () => {
    try {
        // Start here
        const mint = await createMint(
            connection, // connection: Connection
            keypair, // payer: Signer
            keypair.publicKey, // mintAuthority: PublicKey
            null, // freezeAuthority: PublicKey | null
            6 // decimals: number
        );
        console.log('mint :', mint.toBase58().toString());
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()
