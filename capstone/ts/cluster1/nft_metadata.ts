import wallet from "/home/daniel/.solana/.config/keypari.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createGenericFile,
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";

// Create a devnet connection
const umi = createUmi("https://api.devnet.solana.com");

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
  try {
    // Follow this JSON structure
    // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

    const image = "https://devnet.irys.xyz/GCvp8e3UvU9oS51jcdAsCgJm8pbAVsFV26NWtHeSBxJn";
    
    const metadata = {
      name: "Solana Ark Foundation Supporter Badge",
      symbol: "SAF",
      description: "The time to act is now ! ... before their silence becomes our legacy.",
      image,
      attributes: [
        { trait_type: "Proof of Ownership", value: "Yes" },
        { trait_type: "Access Level", value: "Supporter" },
        { trait_type: "Expiration Date", value: "None" },
      ],
      properties: {
        files: [
          {
            type: "image/png",
            uri: "https://devnet.irys.xyz/GCvp8e3UvU9oS51jcdAsCgJm8pbAVsFV26NWtHeSBxJn",
          },
        ],
      },
      creators: [],
    };
    const myUri = await umi.uploader.uploadJson(metadata);
    
    console.log("Your metadata URI: ", myUri.replace("arweave.net", "devnet.irys.xyz"));
  } catch (error) {
    console.log("Oops.. Something went wrong", error);
  }
})();

