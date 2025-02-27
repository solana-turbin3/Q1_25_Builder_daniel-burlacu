import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
//importingt the types from the target folder 
import { SolanaVet } from "../target/types/solana_vet";
import { SolanaAnimal } from "../target/types/solana_animal";
import { SolanaAnimalOwner } from "../target/types/solana_animal_owner";

import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { assert } from "chai";
import * as fs from "fs";
import { SystemProgram } from "@solana/web3.js";

const WALLET_PATH_VETERINARY = `${process.env.HOME}/.solana/.config/veterinary_wallet.json`;
const WALLET_PATH_OWNER = `${process.env.HOME}/.solana/.config/owner_wallet.json`;
const WALLET_PATH_ANIMAL = `${process.env.HOME}/.solana/.config/animal_wallet.json`;

describe("solana-ark-foundation-pilot", () => {
  // Configure the client to use the local cluster.
  //const provider = anchor.AnchorProvider.local();
  const provider = new anchor.AnchorProvider(
    new anchor.web3.Connection("http://127.0.0.1:8898", "processed"),
    anchor.Wallet.local(),
    {}
  );
  
  anchor.setProvider(provider);
  
  const veterinary_program =  anchor.workspace.SolanaVet as Program<SolanaVet>;
  const animal_program = anchor.workspace.SolanaAnimal as Program<SolanaAnimal>;
  const animal_owner_program = anchor.workspace.SolanaAnimalOwner as Program<SolanaAnimalOwner>;
  
  let veterinaryWallet = Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(fs.readFileSync(WALLET_PATH_VETERINARY, "utf-8")))
  );

  let ownerWallet = Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(fs.readFileSync(WALLET_PATH_OWNER, "utf-8")))
  );

  let animalWallet = Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(fs.readFileSync(WALLET_PATH_ANIMAL, "utf-8")))
  );
  
  let vetAuthPda: PublicKey; // Global Treasury PDA
  let veterinaryPda: PublicKey; // Global Admin PDA
  let animalPda: PublicKey; // Cabinet PDA
  let ownerPda: PublicKey; // Owner PDA

  before(async () => {

    [veterinaryPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("cabinet"), veterinaryWallet.publicKey.toBuffer()], // ✅ Correct seeds
      veterinary_program.programId
    );

  [ownerPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("owner"), ownerWallet.publicKey.toBuffer()],
    animal_owner_program.programId
  );

  [animalPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("animal"), ownerWallet.publicKey.toBuffer()],
    animal_program.programId
  );

 [vetAuthPda] = PublicKey.findProgramAddressSync(
  [Buffer.from("vet_authority"), ownerWallet.publicKey.toBuffer()],
  animal_program.programId
 );
 
  console.log("✅ Veterinary PDA:", veterinaryPda.toBase58());
  console.log("✅ Owner PDA:", ownerPda.toBase58());
  console.log("✅ Veterinary PDA:", animalPda.toBase58());

  });

  it("It initializes the veterinary account if it does not exist", async () => {
    const veterinaryInfo = new Array(32).fill(0); // Creates a 32-byte zero-filled array
  
    try {
      // ✅ Try fetching the veterinary cabinet PDA
      const vetAccount = await veterinary_program.account.veterinaryCabinet.fetch(veterinaryPda);
      assert.ok(vetAccount); // ✅ Ensure the account exists
      console.log("✅ Veterinary PDA already exists:", veterinaryPda.toBase58());
    } catch (err) {
      if (err.message.includes("Account does not exist")) {
        console.log("⚠️ Veterinary account does not exist, initializing...");
  
        // ✅ Now initialize the account
        const tx = await veterinary_program.methods
          .initialize(veterinaryInfo)
          .accounts({
            payer: veterinaryWallet.publicKey, // ✅ Ensure correct account name
            cabinet: veterinaryPda, // ✅ Pass the correctly derived PDA
            systemProgram: SystemProgram.programId, // ✅ Ensure this is imported correctly
          })
          .signers([veterinaryWallet])
          .rpc();
  
        console.log("✅ Veterinary account initialized, transaction signature:", tx);
  
        // ✅ Re-fetch the account to confirm it was created successfully
        const vetAccount = await veterinary_program.account.veterinaryCabinet.fetch(veterinaryPda);
        assert.ok(vetAccount, "Veterinary cabinet was not properly initialized!"); // ✅ Add missing assert
      } else {
        throw err; // ✅ Throw unexpected errors
      }
    }
  });

  it("It initialize the animal owner !", async () => {
    const ownerInfo = new Array(32).fill(1); // Creates a 32-byte zero-filled array
    try{
      //✅ Try fetching the veterinary cabinet PDA
      const ownerAccount = await animal_owner_program.account.owner.fetch(ownerPda);
      assert.ok(ownerAccount); // ✅ Ensure the account exists
      console.log("✅ Owner PDA already exists:", ownerPda.toBase58());
    }catch(err){
      if (err.message.includes("Account does not exist")) {
        console.log("⚠️ Owner account does not exist, initializing...");

        // ✅ Fix PDA derivation (must match Rust program)
        // [ownerPda] = PublicKey.findProgramAddressSync(
        //   [Buffer.from("owner"), ownerWallet.publicKey.toBuffer()],
        //   animal_owner_program.programId
        // );
    
        const tx = await animal_owner_program.methods
        .initialize(ownerInfo)
        .accounts({
          payer: ownerWallet.publicKey,
          owner: ownerPda,
          systemProgram: SystemProgram.programId,
        })
        .signers([ownerWallet])
        .rpc();

        console.log("✅ Owner account initialized, transaction signature:", tx);
  
        // ✅ Re-fetch the account to confirm it was created successfully
        const ownerAccount = await animal_owner_program.account.owner.fetch(ownerPda);
        assert.ok(ownerAccount, "Owner was properly initialized !"); // ✅ Add missing assert
    }else{
      throw err;
    }
   }
  });
    it("It initializes the animal program.", async () => {
    const animalInfo = new Array(32).fill(2); // Creates a 32-byte array
    const ownerInfo = new Array(32).fill(2); // Creates a 32-byte array
  
    try {
      // ✅ Try fetching the animal PDA
      const animalAccount = await animal_program.account.animal.fetch(animalPda);
      assert.ok(animalAccount);
      console.log("✅ Animal PDA already exists:", animalPda.toBase58());
    } catch (err) {
      if (err.message.includes("Account does not exist")) {
        console.log("⚠️ Animal account does not exist, initializing...");
  
        // ✅ Print to verify
        console.log("🔍 Owner PDA:", ownerPda.toBase58());
        console.log("🔍 Vet Authority PDA:", vetAuthPda.toBase58());
        console.log("🔍 Animal PDA:", animalPda.toBase58());
  
        // ✅ Now initialize the animal
        const tx = await animal_program.methods
          .initialize(animalInfo)
          .accounts({
            payer: ownerWallet.publicKey,
            animal: animalPda,
            owner: ownerPda,
            vetAuthority: vetAuthPda, // ✅ Must be included
            systemProgram: SystemProgram.programId,
          })
          .signers([ownerWallet])
          .rpc();
  
        console.log("✅ Animal account initialized, transaction signature:", tx);
  
        // ✅ Re-fetch the account to confirm it was created successfully
        const animalAccount = await animal_program.account.animal.fetch(animalPda);
        assert.ok(animalAccount, "Animal was not properly initialized!");
      } else {
        throw err;
      }
    }  
  });

});
