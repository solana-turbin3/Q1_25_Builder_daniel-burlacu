import { Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import walletData from "/home/daniel/.solana/.config/keypari.json";

const connection = new Connection("https://api.devnet.solana.com");

(async () => {
    try {
    // We're going to claim 2 devnet SOL tokens
    const keypair = Keypair.generate();

    //generate keypair from wallet
    const wallet = Keypair.fromSecretKey(Uint8Array.from(walletData));

    const txhash = await connection.requestAirdrop(wallet.publicKey, 2 * LAMPORTS_PER_SOL);
    console.log(`Success! Check out your TX here:
    https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch(e) {
    console.error(`Oops, something went wrong: ${e}`)
    }
    })();
