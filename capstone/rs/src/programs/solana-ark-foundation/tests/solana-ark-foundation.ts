import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VeterinarySystem } from "../target/types/veterinary_system";
import { assert } from "chai";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import {  ASSOCIATED_TOKEN_PROGRAM_ID,TOKEN_PROGRAM_ID, createAssociatedTokenAccount, getAccount } from "@solana/spl-token";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { createNft, findMasterEditionPda, findMetadataPda, mplTokenMetadata, verifySizedCollectionItem } from '@metaplex-foundation/mpl-token-metadata'
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { KeypairSigner, PublicKey as MetaplexPublickey, createSignerFromKeypair, generateSigner, keypairIdentity, percentAmount } from '@metaplex-foundation/umi';
import bs58 from "bs58";

describe("solana-ark-foundation (Devnet)", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .VeterinarySystem as Program<VeterinarySystem>;

  let treasuryPda: PublicKey; // Global Treasury PDA
  let adminPda: PublicKey; // Global Admin PDA
  let cabinetPda: PublicKey; // Cabinet PDA
  let ownerPda: PublicKey; // Owner PDA

  const veterinaryCabinetMetadata = {
    cabinetMetadataUri: "https://bafkreifxw4bpwb56lczl4upcprjgximp3am7y7acsqkmgpyd6xdrjloawm.ipfs.flk-ipfs.xyz",
    name: "Veterinary Cabinet NFT",
    symbol: "VET",
  }

  const animalOwnerMetadata = {
    name: "John Doe",
    animalMetadataUri: "https://bafkreiazy6pflp66wtc37ogx4fpxd2lqq7pqrmoqyq24lm5l3ia2hypyhu.ipfs.flk-ipfs.xyz",
    symbol:"SAF", 
  }

  const animalMetadata = {
    name: "Thor DB",
    animalMetadataUri: "https://bafkreievdajadnlyqh4vw6kzjq4eg5ibiz3r4sv5dlgbxjoop3sc4jv5tm.ipfs.flk-ipfs.xyz",
    symbol:"SAF", 
  }


  // const cabinetMetadataUri = "https://bafkreifxw4bpwb56lczl4upcprjgximp3am7y7acsqkmgpyd6xdrjloawm.ipfs.flk-ipfs.xyz";
  // const name = "Veterinary Cabinet NFT";
  // const symbol = "VET";

  let mint: Keypair;
  let tokenAccount: PublicKey;
  let metadataAccount: PublicKey;
  let masterEditionAccount: PublicKey;

  const umi = createUmi(provider.connection);

  const payer = provider.wallet as NodeWallet;
   
  let nftMint: KeypairSigner = generateSigner(umi);
  // let collectionMint: KeypairSigner = generateSigner(umi);
     
  const creatorWallet = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(payer.payer.secretKey));
  const creator = createSignerFromKeypair(umi, creatorWallet);
  umi.use(keypairIdentity(creator));
  umi.use(mplTokenMetadata());

  // const collection: anchor.web3.PublicKey = new anchor.web3.PublicKey(collectionMint.publicKey.toString());

  const fundTreasury = async (amount: number) => {
    console.log(`Funding Treasury PDA with ${amount / anchor.web3.LAMPORTS_PER_SOL} SOL...`);
    const transferIx = anchor.web3.SystemProgram.transfer({
      fromPubkey: provider.wallet.publicKey,
      toPubkey: treasuryPda,
      lamports: amount,
    });

    const tx = new anchor.web3.Transaction().add(transferIx);
    await provider.sendAndConfirm(tx);
    console.log("Treasury funded successfully.");
  }

  before(async () => {
    // Derive and store the Treasury PDA and Admin PDA
    treasuryPda = PublicKey.findProgramAddressSync(
      [Buffer.from("treasury")],
      program.programId
    )[0];

    adminPda = PublicKey.findProgramAddressSync(
      [Buffer.from("admin")],
      program.programId
    )[0];

        // Derive Cabinet PDA
    cabinetPda = PublicKey.findProgramAddressSync(
          [Buffer.from("cabinet"), provider.wallet.publicKey.toBuffer()],
          program.programId
     )[0];
             // Derive Owner PDA
    ownerPda = PublicKey.findProgramAddressSync(
      [Buffer.from("owner"), provider.wallet.publicKey.toBuffer()],
      program.programId
    )[0];

     
    // Initialize mint and metadata accounts
    // mint = Keypair.generate();
    // tokenAccount = await createAssociatedTokenAccount(
    //   provider.connection,
    //   provider.wallet.payer,
    //   mint.publicKey,
    //   provider.wallet.publicKey
    // );

    // metadataAccount = PublicKey.findProgramAddressSync(
    //   [
    //     Buffer.from("metadata"),
    //     TOKEN_PROGRAM_ID.toBuffer(),
    //     mint.publicKey.toBuffer(),
    //   ],
    //   program.programId
    // )[0];

    // masterEditionAccount = PublicKey.findProgramAddressSync(
    //   [
    //     Buffer.from("metadata"),
    //     TOKEN_PROGRAM_ID.toBuffer(),
    //     mint.publicKey.toBuffer(),
    //     Buffer.from("edition"),
    //   ],
    //   program.programId
    // )[0];
  });

  it("Initializes the admin PDA on Devnet", async () => {
    try {
      const adminAccount = await program.account.adminAccount.fetch(adminPda);
      console.log("Admin PDA already exists:", adminAccount);
    } catch (err) {
      if (err.message.includes("Account does not exist")) {
        console.log("Admin PDA not found, initializing...");
        const tx = await program.methods
          .initializeAdmin()
          .accounts({
            payer: provider.wallet.publicKey,
          })
          .rpc();

        console.log("Transaction Signature:", tx);
      } else {
        throw err;
      }
    }
  });

  it("Initializes the Treasury PDA", async () => {
    try {
      const treasuryAccount = await program.account.treasuryAccount.fetch(
        treasuryPda
      );
      console.log("Treasury PDA already exists:", treasuryAccount);
  
      // If the Treasury PDA exists, check its balance
      if (treasuryAccount.balance.toNumber() !== 0) {
        console.log(
          "Treasury balance is already non-zero:",
          treasuryAccount.balance.toString()
        );
        return; // Skip initialization if balance is non-zero
      }
    } catch (err) {
      if (err.message.includes("Account does not exist")) {
        console.log("Treasury PDA not found, initializing...");
        const tx = await program.methods
          .initializeTreasury()
          .accounts({
            payer: provider.wallet.publicKey,
          })
          .rpc();
  
        console.log("Transaction Signature:", tx);
      } else {
        throw err;
      }
    }
  
    // Fetch the Treasury PDA again to verify its state
    const treasuryAccount = await program.account.treasuryAccount.fetch(
      treasuryPda
    );
    assert.strictEqual(
      treasuryAccount.balance.toString(),
      "0",
      "Treasury balance should be zero"
    );
    console.log(
      "Treasury successfully initialized with PDA:",
      treasuryPda.toBase58()
    );
  });

  it("Adds a new veterinary cabinet", async () => {
     [cabinetPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("cabinet"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );
  
    const [treasuryPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("treasury")],
      program.programId
    );
  
    const [adminPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("admin")],
      program.programId
    );
  
    const cabinetDetails = {
      info: Array.from(
        new Uint8Array(Buffer.from("Paw Perfect Veterinary Clinic".padEnd(32)))
      ),
    };
  
    const payer = provider.wallet.publicKey;
    const joinFeeLamports = 800_000; // 0.8 SOL in lamports
  
       // Fund the Treasury PDA
       await fundTreasury(joinFeeLamports);

       try {
         const tx = await program.methods
           .addVeterinaryCabinet(Array.from(new Uint8Array(Buffer.from("Paw Perfect Veterinary Clinic".padEnd(32)))))
           .accounts({
             cabinet: cabinetPda,
             treasury: treasuryPda,
             adminPda: adminPda,
             admin: provider.wallet.publicKey,
             payer: provider.wallet.publicKey,
             systemProgram: SystemProgram.programId,
           })
           .rpc();
   
         console.log("Transaction Signature:", tx);
       } catch (err) {
      console.error("Transaction failed:", err);
      if ("tx" in err) {
        const logs = await provider.connection.getConfirmedTransaction(
          err.tx,
          "confirmed"
        );
        console.error("Transaction Logs:", logs.meta?.logMessages || []);
      } else {
        console.error("Error Logs Unavailable. Raw Error:", err);
      }
      throw err;
    }
  });

it("Mints a Veterinary Cabinet NFT using Metaplex", async () => {
  // Use Metaplex to create the NFT
  const nftTransaction = await createNft(umi, {
    mint: nftMint,
    name: veterinaryCabinetMetadata.name,
    symbol: veterinaryCabinetMetadata.symbol,
    uri: veterinaryCabinetMetadata.cabinetMetadataUri,
    sellerFeeBasisPoints: percentAmount(5.5),
    creators: [
      {
        address: umi.identity.publicKey,
        verified: true,
        share: 100,
      },
    ],
  }).sendAndConfirm(umi);

  console.log(`Created Veterinary Cabinet NFT with mint: ${nftMint.publicKey.toString()}`);

  // Convert the signature to a base58 string
  const signatureBase58 = bs58.encode(nftTransaction.signature);

  // Confirm the transaction
  const confirmedTransaction = await provider.connection.confirmTransaction(signatureBase58, "confirmed");
  assert.ok(confirmedTransaction.value.err === null, "NFT minting failed.");
});

it("Adds a new animal owner", async () => {
  // Derive the necessary PDAs
  const ownerPda = PublicKey.findProgramAddressSync(
    [Buffer.from("owner"), provider.wallet.publicKey.toBuffer()],
    program.programId
  )[0];

  const [cabinetPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("cabinet"), provider.wallet.publicKey.toBuffer()],
    program.programId
  );

  const ownerDetails = {
    info: Array.from(
      new Uint8Array(Buffer.from("John Doe".padEnd(32))) // Animal owner info
    ),
  };

  try {
    // Call the add_animal_owner instruction
    const tx = await program.methods
      .addAnimalOwner(ownerDetails.info)
      .accounts({
        cabinet: cabinetPda,
        payer: provider.wallet.publicKey, // The wallet paying for the transactionsystemProgram: SystemProgram.programId, // System program

              })
      .rpc();

    console.log("Transaction Signature:", tx);

    // Fetch and validate the new owner account
    const ownerAccount = await program.account.owner.fetch(ownerPda);
    // const cabinetAccount = await program.account.veterinaryCabinet.fetch(cabinetPda);
    // assert.equal(
    //   ownerAccount.veterinaryCabinetId,
    //   cabinetAccount.id,
    //   "Owner's veterinary cabinet ID mismatch"
    // );

    assert.equal(
      ownerAccount.veterinaryCabinetId.toBase58(),
      cabinetPda.toBase58(),
      "Owner's veterinary cabinet ID mismatch"
    );

    assert.equal(
      Buffer.from(ownerAccount.info).toString().trim(),
      "John Doe",
      "Owner info mismatch"
    );
    assert.equal(
      ownerAccount.id.toBase58(),
      provider.wallet.publicKey.toBase58(),
      "Owner ID mismatch"
    );

    console.log("Animal owner added successfully:", ownerPda.toBase58());
  } catch (err) {
    console.error("Transaction failed:", err);
    if ("tx" in err) {
      const logs = await provider.connection.getConfirmedTransaction(
        err.tx,
        "confirmed"
      );
      console.error("Transaction Logs:", logs.meta?.logMessages || []);
    } else {
      console.error("Error Logs Unavailable. Raw Error:", err);
    }
    throw err;
  }
});

it("Mints a Animal Owner NFT using Metaplex", async () => {
  let nftMint: KeypairSigner = generateSigner(umi);
  // Use Metaplex to create the NFT
  const nftTransaction = await createNft(umi, {
    mint: nftMint,
    name: animalOwnerMetadata.name,
    symbol: animalOwnerMetadata.symbol,
    uri: animalOwnerMetadata.animalMetadataUri,
    sellerFeeBasisPoints: percentAmount(5.5),
    creators: [
      {
        address: umi.identity.publicKey,
        verified: true,
        share: 100,
      },
    ],
  }).sendAndConfirm(umi);

  console.log(`Created Animal Owner NFT with mint: ${nftMint.publicKey.toString()}`);

  // Convert the signature to a base58 string
  const signatureBase58 = bs58.encode(nftTransaction.signature);

  // Confirm the transaction
  const confirmedTransaction = await provider.connection.confirmTransaction(signatureBase58, "confirmed");
  assert.ok(confirmedTransaction.value.err === null, "NFT minting failed.");
});

it("Adds a new animal", async () => {
  const animalPda = PublicKey.findProgramAddressSync(
    [Buffer.from("animal"), ownerPda.toBuffer(), cabinetPda.toBuffer()],
    program.programId
  )[0];

  const animalDetails = {
    info: Array.from(
      new Uint8Array(Buffer.from("{name:Thor, breed:GSD}".padEnd(32))) // Animal metadata
    ),
  };

  try {
    const tx = await program.methods
      .addAnimal(animalDetails.info)
      .accounts({
        owner: ownerPda,
        cabinet: cabinetPda,
        animal: animalPda,
        payer: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Transaction Signature:", tx);

    const animalAccount = await program.account.animal.fetch(animalPda);

    assert.equal(
      animalAccount.veterinaryCabinetId.toBase58(),
      payer.publicKey.toBase58(),
      "Animal's veterinary ID mismatch"
    );

    assert.equal(
      animalAccount.ownerId.toBase58(),
      payer.publicKey.toBase58(),
      "Animal's owner ID mismatch"
    );

    assert.equal(
      animalAccount.id.toBase58(),
      payer.publicKey.toBase58(),
      "Animal's veterinary cabinet ID mismatch"
    );
    assert.equal(
      Buffer.from(animalAccount.info).toString().trim(),
      "{name:Thor, breed:GSD}",
      "Animal info mismatch"
    );

    console.log("Animal added successfully:", animalPda.toBase58());
  } catch (err) {
    console.error("Transaction failed:", err);
    throw err;
  }
});

it("Mints an Animal NFT using Metaplex", async () => {
  let nftMint: KeypairSigner = generateSigner(umi);
  // Use Metaplex to create the NFT
  const nftTransaction = await createNft(umi, {
    mint: nftMint,
    name: animalMetadata.name,
    symbol: animalMetadata.symbol,
    uri: animalMetadata.animalMetadataUri,
    sellerFeeBasisPoints: percentAmount(5.5),
    creators: [
      {
        address: umi.identity.publicKey,
        verified: true,
        share: 100,
      },
    ],
  }).sendAndConfirm(umi);

  console.log(`Created Animal NFT with mint: ${nftMint.publicKey.toString()}`);

  // Convert the signature to a base58 string
  const signatureBase58 = bs58.encode(nftTransaction.signature);

  // Confirm the transaction
  const confirmedTransaction = await provider.connection.confirmTransaction(signatureBase58, "confirmed");
  assert.ok(confirmedTransaction.value.err === null, "NFT minting failed.");
});

});