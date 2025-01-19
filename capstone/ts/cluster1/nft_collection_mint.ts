import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi";
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";
import wallet from "/home/daniel/.solana/.config/keypari.json";
import base58 from "bs58";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata());

const sellerFeeBasisPoints = percentAmount(0, 2);
const name = "SAF Supporter Badge";
const symbol = "SAF";
let uriTemplate = "https://devnet.irys.xyz/HAPEvLR5G53363X2Lu3XA8YsC661ejs8kC65VgVcAs1a";

// Counter for NFT numbers
let nftCounter = 1; 

// Function to mint a single NFT
const mintNft = async () => {
    const mint = generateSigner(umi);
    const nftNumber = nftCounter++; // Increment NFTNumber for each mint

    const metadataUri = `${uriTemplate}?NFTNumber=${nftNumber}`; // Customize metadata URI
    const nftName = `${name} #${nftNumber}`;

    try {
        const tx = await createNft(
            umi,
            {
                mint,
                name: nftName,
                symbol,
                uri: metadataUri,
                sellerFeeBasisPoints,
            }
        ).sendAndConfirm(umi);

        const signature = base58.encode(tx.signature);
        console.log(`Succesfully Minted NFT #${nftNumber}!`);
        console.log(`Transaction: https://explorer.solana.com/tx/${signature}?cluster=devnet`);
        console.log(`Mint Address: https://explorer.solana.com/address/${mint.publicKey}?cluster=devnet`);
    } catch (error) {
        console.error(`Failed to mint NFT #${nftNumber}:`, error);
    }
};

// Function to mint multiple NFTs in batches
const mintNftCollection = async (totalNfts: number) => {
    console.log(`Starting to mint ${totalNfts} NFTs...`);
    for (let i = 0; i < totalNfts; i++) {
        await mintNft();
    }
    console.log(`Finished minting ${totalNfts} NFTs!`);
};

// Mint 1000 NFTs
(async () => {
    await mintNftCollection(1000);
})();
