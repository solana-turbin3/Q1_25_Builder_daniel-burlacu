import wallet from "/home/daniel/.solana/.config/localwallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { 
    createMetadataAccountV3, 
    CreateMetadataAccountV3InstructionAccounts, 
    CreateMetadataAccountV3InstructionArgs,
    DataV2Args,
    getSignMetadataInstructionDataSerializer
} from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, signerIdentity, publicKey } from "@metaplex-foundation/umi";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";

// Define our Mint address
const mint = publicKey("7EovjfTZMuixX3Goeu9ZP9CbZx7yrdjqopN9t3LzWb1k")

// Create a UMI connection
const umi = createUmi('https://api.devnet.solana.com');
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

(async () => {
    try {
        // Start here
        let accounts: CreateMetadataAccountV3InstructionAccounts = {
            mint:mint,
            mintAuthority: signer,
        };

        let data: DataV2Args = {
            name: "SAF Token",
            symbol: "SAFT",
            uri: "https://bafkreibfnlozk2yqb6im73kl4ww6nmhowdoaa7ma6y5aan3i27fekd7exe.ipfs.flk-ipfs.xyz",
            sellerFeeBasisPoints: 500,
            creators: [
                {
                    address: signer.publicKey,
                    verified: true,
                    share: 100,  // If this is the only creator, give it 100% share
                }
            ],
            collection: null,
            uses: null,
        }
        
        
        let args: CreateMetadataAccountV3InstructionArgs = {
            data,
            isMutable: true,
            collectionDetails: null,
        }

        let tx = createMetadataAccountV3(
            umi,
            {
                ...accounts,
                ...args
            }
        )
        let result = await tx.sendAndConfirm(umi);
        console.log(bs58.encode(result.signature));
        
        const encode = bs58.encode([76,52,55,154,46,78,56,105,88,166,215,144,29,146,121,202,59,129,216,16,248,130,207,122,29,225,103,209,14,151,200,153,153,254,93,52,214,208,130,119,26,64,36,167,16,13,242,237,178,21,200,28,64,130,176,130,171,162,39,66,171,236,171,187])
        console.log('encode: ',encode);

    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();
