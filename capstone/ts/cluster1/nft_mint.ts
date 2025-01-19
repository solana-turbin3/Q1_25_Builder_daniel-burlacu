import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "/home/daniel/.solana/.config/keypari.json";
import base58 from "bs58";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi);
const sellerFeeBasisPoints = percentAmount(0, 2);

const name = "SAF Supporter";
let uri = " https://devnet.irys.xyz/HAPEvLR5G53363X2Lu3XA8YsC661ejs8kC65VgVcAs1a";


uri = uri.replace("arweave.net", "devnet.irys.xyz"); 

const symbol = "SAF";
// const description = "Dogpound Charity NFT - A Sol for a Bone !";
// const image = "https://arweave.net/zpBs5PJR2eVwT2hGoMZg9aFbBu2MEYuKRwGnHkX3eAb";
(async () => {
    let tx = createNft(
        umi, 
        {
          mint,
          name,
          symbol,
          uri,
          sellerFeeBasisPoints,
        },
    )
    let result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);
    
    console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)
    // https://explorer.solana.com/address/${mint.publicKey}
    console.log("Mint Address: ", `https://explorer.solana.com/address/${mint.publicKey}?cluster=devnet`);
})();