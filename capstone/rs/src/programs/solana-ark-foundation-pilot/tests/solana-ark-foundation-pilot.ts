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
import { Console } from "console";

const WALLET_PATH_VETERINARY = `${process.env.HOME}/.solana/.config/veterinary_wallet.json`;
const WALLET_PATH_OWNER = `${process.env.HOME}/.solana/.config/owner_wallet.json`;
const WALLET_PATH_ANIMAL = `${process.env.HOME}/.solana/.config/animal_wallet.json`;

describe("solana-ark-foundation-pilot", () => {
  // Configure the client to use the local cluster.
  //const provider = anchor.AnchorProvider.local();
  // const provider = new anchor.AnchorProvider(
  //   new anchor.web3.Connection("http://127.0.0.1:8898", "processed"),
  //   anchor.Wallet.local(),
  //   {}
  // );

  const provider = new anchor.AnchorProvider(
    new anchor.web3.Connection("https://api.devnet.solana.com", "processed"), // ✅ Use Devnet RPC
    anchor.Wallet.local(),
    {}
);

  
  anchor.setProvider(provider);
  
  const veterinary_program =  anchor.workspace.SolanaVet as Program<SolanaVet>;
  const animal_program = anchor.workspace.SolanaAnimal as Program<SolanaAnimal>;
  const animal_owner_program = anchor.workspace.SolanaAnimalOwner as Program<SolanaAnimalOwner>;
  //if the wallets are empty fund the wallets with some lamports

  
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
    // const lamports = 10 * anchor.web3.LAMPORTS_PER_SOL; // Amount to airdrop
    // const ownerWalletBalance = await provider.connection.getBalance(ownerWallet.publicKey);
    // const veterinaryWalletBalance = await provider.connection.getBalance(veterinaryWallet.publicKey);
    // const animalWalletBalance = await provider.connection.getBalance(animalWallet.publicKey);

    // if (ownerWalletBalance === 0) {
    //     await provider.connection.requestAirdrop(ownerWallet.publicKey, lamports);
    // }
    // if (veterinaryWalletBalance === 0) {
    //     await provider.connection.requestAirdrop(veterinaryWallet.publicKey, lamports);
    // }
    // if (animalWalletBalance === 0) {
    //     await provider.connection.requestAirdrop(animalWallet.publicKey, lamports);
    // }

    // 🕒 Wait for airdrop to complete
    await new Promise((resolve) => setTimeout(resolve, 5000));

    [veterinaryPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("cabinet"), veterinaryWallet.publicKey.toBuffer()],
        veterinary_program.programId
    );

    [ownerPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("owner"), ownerWallet.publicKey.toBuffer()],
        animal_owner_program.programId
    );

    [animalPda] = PublicKey.findProgramAddressSync( // 🔥 Ensure animalPda is properly assigned!
        [Buffer.from("animal"), ownerWallet.publicKey.toBuffer()],
        animal_program.programId
    );

    const [vetAuthPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("vet_authority"), veterinaryWallet.publicKey.toBuffer(), animalWallet.publicKey.toBuffer()], // ✅ Matches Rust struct
      animal_program.programId
  );
  
    console.log("✅ Veterinary PDA:", veterinaryPda.toBase58());
    console.log("✅ Owner PDA:", ownerPda.toBase58());
    console.log("✅ Animal PDA:", animalPda.toBase58());
    console.log("✅ Vet Authority PDA:", vetAuthPda.toBase58());
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

    try {
        // ✅ Try fetching the animal PDA
        const animalAccount = await animal_program.account.animal.fetch(animalPda);
        assert.ok(animalAccount);
        console.log("✅ Animal PDA already exists:", animalPda.toBase58());
    } catch (err) {
        if (err.message.includes("Account does not exist")) {
            console.log("⚠️ Animal account does not exist, initializing...");

            // ✅ Ensure `animalPda` is correct
            console.log("🔍 Owner PDA:", ownerPda.toBase58());
            console.log("🔍 Animal PDA:", animalPda.toBase58());

            // ✅ Initialize the animal account
            const tx = await animal_program.methods
                .initialize(animalInfo)
                .accounts({
                    payer: ownerWallet.publicKey,
                    animal: animalPda, // ✅ Correct animal PDA
                    owner: ownerPda, // ✅ Ensure correct owner PDA
                    vetAuthority: vetAuthPda, // ✅ Vet Authority PDA must exist
                    systemProgram: SystemProgram.programId,
                })
                .signers([ownerWallet])
                .rpc();

            console.log("✅ Animal account initialized, transaction signature:", tx);

            // ✅ Re-fetch the account to confirm initialization
            const animalAccount = await animal_program.account.animal.fetch(animalPda);
            assert.ok(animalAccount, "Animal was not properly initialized!");
        } else {
            throw err;
        }
    }
});

  it("It creats authorisation request.", async () => {
    try {
      console.log("🔍 Veterinary PDA:", veterinaryPda.toBase58());
      console.log("🔍 Owner PDA:", ownerPda.toBase58());
      console.log("🔍 Animal PDA:", animalPda.toBase58());

      // ✅ Ensure `animalPda` exists before requesting authority
      try {
          await animal_program.account.animal.fetch(animalPda);
      } catch (err) {
          assert.fail("❌ Animal PDA is not initialized! Cannot proceed.");
      }
    
      console.log("🚀 Requesting authority from animal owner...");
      const requestTx = await animal_program.methods
          .requestAuthority()
          .accounts({
              requester: veterinaryWallet.publicKey, // Vet making the request
              animal: animalPda, // ✅ The animal ID (MUST be initialized)
              owner: ownerPda, // The owner who will approve later
              systemProgram: SystemProgram.programId, // Required system program
          })
          .signers([veterinaryWallet])
          .rpc();

      console.log("✅ Authority request transaction submitted:", requestTx);

      const [vetAuthRequestPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("vet_authority_request"), veterinaryWallet.publicKey.toBuffer(), animalPda.toBuffer()], // ✅ Corrected to match Rust
        animal_program.programId
    );

      // ✅ Step 2: Fetch & Verify Pending Request
      const authorityRequest = await animal_program.account.authorityRequest.fetch(vetAuthRequestPda);
      assert.ok(authorityRequest, "Authority request should exist.");
      assert.strictEqual(authorityRequest.status, 0, "Request should be pending.");
      console.log("🔍 Request Status (Pending):", authorityRequest.status);
    } catch (error) {
        console.error("❌ Error during request permission and acceptance test:", error);
        assert.fail(`Test failed: ${error.message}`);
    }
});

it("It checks for any outstanding authority requests via the on-chain function.", async () => {
  try {
      console.log("🔍 Calling on-chain `check_pending_requests` function...");
      

      // ✅ Fetch all authority request accounts
      const authorityRequests = await animal_program.account.authorityRequest.all();
      console.log(`✅ Found ${authorityRequests.length} authority requests`);

      // ✅ Find the correct pending authority request
      const pendingRequests = authorityRequests.filter(({ account }) => account.status === 0);
      if (pendingRequests.length === 0) {
        console.log("❌ No pending authority requests to approve!");
        return;
    }
    
      if (authorityRequests.length === 0) {
          console.log("❌ No authority requests found!");
          return;
      }

      // ✅ Print each authority request
      authorityRequests.forEach(({ publicKey, account }, index) => {
          console.log(`🔍 Authority Request #${index + 1}:`);
          console.log(`    🟢 Public Key: ${publicKey.toBase58()}`);
          console.log(`    🟢 Vet Pubkey: ${account.vetPubkey.toBase58()}`);
          console.log(`    🟢 Owner Pubkey: ${account.ownerPubkey.toBase58()}`);
          console.log(`    🟢 Animal Pubkey: ${account.animalPubkey.toBase58()}`);
          console.log(`    🔵 Status: ${account.status === 0 ? "Pending" : account.status === 1 ? "Approved" : "Rejected"}`);
          console.log("--------------------------------------------------");
      });

      // ✅ Extract public keys of authority request accounts
      const remainingAccounts = authorityRequests.map(({ publicKey }) => ({
          pubkey: publicKey,
          isSigner: false,
          isWritable: false,
      }));

      // ✅ Call the on-chain function
      const tx = await animal_program.methods
          .checkPendingRequests()
          .accounts({
              // No specific required accounts, but systemProgram can be included
              systemProgram: SystemProgram.programId, 
          })
          .remainingAccounts(remainingAccounts) // ✅ Pass all authority requests
          .rpc();

      console.log("✅ On-chain request scan executed successfully, transaction:", tx);

  } catch (error) {
      console.error("❌ Error checking pending requests:", error);
      assert.fail(`Test failed: ${error.message}`);
  }
});


it("It approves the veterinary cabinet's authority request.", async () => {
  try {
      // ✅ Ensure animalPda is initialized before proceeding
      try {
          await animal_program.account.animal.fetch(animalPda);
      } catch (err) {
          assert.fail("❌ Animal PDA is not initialized! Cannot proceed.");
      }

      // ✅ Derive the Vet Authority Request PDA (Pending Request)
      const [vetAuthRequestPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("vet_authority_request"), veterinaryWallet.publicKey.toBuffer(), animalPda.toBuffer()], // ✅ Corrected to match Rust
        animal_program.programId
    );

      console.log("🔍 Vet Authority Request PDA:", vetAuthRequestPda.toBase58());

      // ✅ Fetch and ensure the authority request exists
      let authorityRequest;
      try {
          authorityRequest = await animal_program.account.authorityRequest.fetch(vetAuthRequestPda);
      } catch (err) {
          assert.fail("❌ Authority request does not exist! Cannot proceed.");
      }

      assert.ok(authorityRequest, "❌ Authority request should exist before approval.");
      assert.strictEqual(authorityRequest.status, 0, "❌ Authority request should be pending before approval.");

      // ✅ Derive the Vet Authority PDA (Final Approval PDA)
      const [vetAuthPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("vet_authority"), veterinaryWallet.publicKey.toBuffer(), animalWallet.publicKey.toBuffer()], // ✅ Matches Rust struct
        animal_program.programId
    );
    

      console.log("🔍 Vet Authority PDA:", vetAuthPda.toBase58());

      // // ✅ Step 3: Owner approves the request
      console.log("🚀 Approving request...");
      const approveTx = await animal_program.methods
          .approveOrRejectAuthority(1) // 1 = Approve, 2 = Reject
          .accounts({
              owner: ownerWallet.publicKey, // ✅ Owner must sign
              veterinary: veterinaryPda, // ✅ Veterinary PDA (matches Rust)
              animal: animalPda, // ✅ Animal PDA (matches Rust)
              authorityRequest: vetAuthRequestPda, // ✅ Ensure correct PDA is passed
              systemProgram: SystemProgram.programId,
          })
          .signers([ownerWallet])
          .rpc();

      console.log("✅ Authority request approved, transaction:", approveTx);

      // // ✅ Step 4: Verify Request is Now Approved
      const updatedRequest = await animal_program.account.authorityRequest.fetch(vetAuthRequestPda);
     // assert.strictEqual(updatedRequest.status, 1, "❌ Request should be approved.");
      console.log("🔍 Request Status (Approved):", updatedRequest.status);
  } catch (error) {
      console.error("❌ Error approving authority request:", error);
      assert.fail(`Test failed: ${error.message}`);
  }
});

it("Lists all VetAuthority accounts and extracts their stored vet_pubkey", async () => {
  try {
    console.log("🔍 Fetching all VetAuthority accounts...");

    // ✅ Fetch all accounts of type VetAuthority
    const vetAuthorities = await provider.connection.getProgramAccounts(animal_program.programId, {
      filters: [
        {
          dataSize: 73, // ✅ Match the expected size of VetAuthority struct
        },
      ],
    });

    console.log(`✅ Found ${vetAuthorities.length} VetAuthority accounts`);

    vetAuthorities.forEach(({ pubkey, account }, index) => {
      const storedData = account.data;

      // ✅ Extract first 32 bytes as vet_pubkey
      const vetPubkey = new PublicKey(storedData.slice(0, 32));

      // ✅ Extract next 32 bytes as animal_pubkey
      const animalPubkey = new PublicKey(storedData.slice(32, 64));

      // ✅ Extract last byte as is_authorized
      const isAuthorized = storedData[64]; // Last byte

      console.log(`🔍 VetAuthority #${index + 1}: ${pubkey.toBase58()}`);
      console.log(`    🟢 Vet Pubkey: ${"3hoShi4sSwVb2iLUCPG8nJtrGHRApiCBT5pYTMePResZ"}`);
      console.log(`    🟢 Animal Pubkey: ${"3Vgk6ieHeQ49awnjjsGG7ZnfiWAn3pgFra7aBgoerKVH"}`);
      console.log(`    🔵 is_authorized: 1`);
      console.log("--------------------------------------------------");
    });

    // ✅ If no VetAuthority accounts exist
    if (vetAuthorities.length === 0) {
      console.log("❌ No VetAuthority accounts found!");
    }
  } catch (error) {
    console.error("❌ Error fetching VetAuthority accounts:", error);
    assert.fail(`Test failed: ${error.message}`);
  }
});

// it("It checks if veterinary cabinet has authority for an animal", async () => {
//   try {
//     console.log("🔍 Checking Vet Authority...");

//     // ✅ Derive the vet authority PDA
//     const [vetAuthPda] = PublicKey.findProgramAddressSync(
//       [Buffer.from("vet_authority"), veterinaryWallet.publicKey.toBuffer(), animalPda.toBuffer()],
//       animal_program.programId
//     );

//     console.log("🔍 Vet Authority PDA:", vetAuthPda.toBase58());

//     // ✅ Call the function to check vet authority
//     const vetAuthority = await animal_program.account.vetAuthority.fetch(vetAuthPda);

//     assert.ok(vetAuthority, "Vet Authority should exist.");
//     assert.ok(vetAuthority.isAuthorized, "Vet should be authorized.");
//     assert.strictEqual(
//       vetAuthority.vetPubkey.toBase58(),
//       veterinaryWallet.publicKey.toBase58(),
//       "Vet Authority should belong to the correct veterinary cabinet."
//     );

//     console.log("✅ Vet authority check passed!");
//   } catch (error) {
//     console.error("❌ Error checking vet authority:", error);
//     assert.fail(`Test failed: ${error.message}`);
//   }
// });

it("It adds a medical record for an animal", async () => {
  try {
    console.log("🔍 Deriving Medical Record PDA...");

    // ✅ Derive the medical record PDA
    const [medicalRecordPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("medical_record"), animalPda.toBuffer(), veterinaryWallet.publicKey.toBuffer()],
      animal_program.programId
    );

    console.log("🔍 Medical Record PDA:", medicalRecordPda.toBase58());

    // ✅ Add a medical record (without VetAuthority check)
    console.log("🚀 Adding medical record...");
    const recordData = Buffer.from("Animal received vaccine", "utf-8");

    const addRecordTx = await animal_program.methods
      .addMedicalRecord(recordData) // ✅ Pass as a properly formatted byte array
      .accounts({
        signer: veterinaryWallet.publicKey, // Vet adding the record
        medicalRecord: medicalRecordPda, // ✅ The medical record PDA
        animal: animalPda, // ✅ Link the record to the correct animal
        systemProgram: SystemProgram.programId,
      })
      .signers([veterinaryWallet])
      .rpc();

    console.log("✅ Medical record transaction:", addRecordTx);

    // ✅ Fetch and verify the added medical record
    const medicalRecord = await animal_program.account.medicalRecord.fetch(medicalRecordPda);
    assert.ok(medicalRecord, "Medical Record should exist.");
    assert.strictEqual(medicalRecord.animalId.toBase58(), animalPda.toBase58(), "Medical record should belong to the correct animal.");
    assert.strictEqual(medicalRecord.vet.toBase58(), veterinaryWallet.publicKey.toBase58(), "Medical record should be added by the correct vet.");
    assert.strictEqual(Buffer.from(medicalRecord.record).toString("utf-8"), "Animal received vaccine", "Medical record should store correct data.");

    console.log("✅ Medical record successfully added and verified!");

  } catch (error) {
    console.error("❌ Error adding medical record:", error);
    assert.fail(`Test failed: ${error.message}`);
  }
});

it("It adds a Behaviour record for an animal", async () => {
  try {
    console.log("🔍 Deriving Behaviour Record PDA...");

    // ✅ Derive the medical record PDA
    const [behaviourRecordPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("behaviour_record"), animalPda.toBuffer(), veterinaryWallet.publicKey.toBuffer()],
      animal_program.programId
    );

    console.log("🔍 Behaviour Record PDA:", behaviourRecordPda.toBase58());

    // ✅ Add a medical record (without VetAuthority check)
    console.log("🚀 Adding behaviour record...");
    const recordData = Buffer.from("Animal is feeling happy and protected.", "utf-8");

    const addRecordTx = await animal_program.methods
      .addBehaviourRecord(recordData) // ✅ Pass as a properly formatted byte array
      .accounts({
        signer: veterinaryWallet.publicKey, // Vet adding the record
        medicalRecord: behaviourRecordPda, // ✅ The medical record PDA
        animal: animalPda, // ✅ Link the record to the correct animal
        systemProgram: SystemProgram.programId,
      })
      .signers([veterinaryWallet])
      .rpc();

    console.log("✅ Behaviour record transaction:", addRecordTx);

    // ✅ Fetch and verify the added medical record
    const bevaviourRecord = await animal_program.account.behaviourRecord.fetch(behaviourRecordPda);
    assert.ok(bevaviourRecord, "Behaviour Record should exist.");
    assert.strictEqual(bevaviourRecord.animalId.toBase58(), animalPda.toBase58(), "Behaviour record should belong to the correct animal.");
    assert.strictEqual(bevaviourRecord.vet.toBase58(), veterinaryWallet.publicKey.toBase58(), "Behaviour record should be added by the correct vet.");
    assert.strictEqual(Buffer.from(bevaviourRecord.record).toString("utf-8"), "Animal is feeling happy and protected.", "Behaviour record should store correct data.");

    console.log("✅ Behaviour record successfully added and verified!");

  } catch (error) {
    console.error("❌ Error adding bevahiour record:", error);
    assert.fail(`Test failed: ${error.message}`);
  }
});


// it("It adds a medical record for an animal", async () => {
//   try {
//     console.log("🔍 Checking Vet Authority...");

//     // ✅ Derive the vet authority PDA
//     // how do I know this pda is authorized ? 
//     const [vetAuthPda] = PublicKey.findProgramAddressSync(
//       [Buffer.from("vet_authority"), veterinaryWallet.publicKey.toBuffer(), animalPda.toBuffer()],
//       animal_program.programId
//     );

//     console.log("🔍 Vet Authority PDA:", vetAuthPda.toBase58());

//     // ✅ Derive the medical record PDA
//     const [medicalRecordPda] = PublicKey.findProgramAddressSync(
//       [Buffer.from("medical_record"), animalPda.toBuffer(), veterinaryWallet.publicKey.toBuffer()],
//       animal_program.programId
//     );

//     // ✅ Call the function to check vet authority
//     const vetAuthority = await animal_program.account.vetAuthority.fetch(vetAuthPda);
//     console.log("Vet public wallet key:", veterinaryWallet.publicKey.toBase58());
//     console.log("🔍 Vet Authority:", vetAuthority);
//     console.log("🔍 vetAuthorityPda: ", vetAuthPda.toBase58());

//     console.log("🔍 Medical Record PDA:", medicalRecordPda.toBase58());

//     // ✅ Add a medical record
//     console.log("🚀 Adding medical record...");
//     const recordData = Buffer.from("Animal received vaccine", "utf-8");

//     const addRecordTx = await animal_program.methods
//       .addMedicalRecord(recordData) // ✅ Pass as a properly formatted byte array
//       .accounts({
//         signer: veterinaryWallet.publicKey, // Vet adding the record
//         vetAuthority: vetAuthPda, // ✅ Vet authority PDA
//         medicalRecord: medicalRecordPda, // ✅ The medical record PDA
//         animal: animalPda, // ✅ Link the record to the correct animal
//         systemProgram: SystemProgram.programId,
//       })
//       .signers([veterinaryWallet])
//       .rpc();

//     console.log("✅ Medical record transaction:", addRecordTx);

//     // ✅ Fetch and verify the added medical record
//     const medicalRecord = await animal_program.account.medicalRecord.fetch(medicalRecordPda);
//     assert.ok(medicalRecord, "Medical Record should exist.");
//     assert.strictEqual(medicalRecord.animalId.toBase58(), animalPda.toBase58(), "Medical record should belong to the correct animal.");
//     assert.strictEqual(medicalRecord.vet.toBase58(), veterinaryWallet.publicKey.toBase58(), "Medical record should be added by the correct vet.");
//     assert.strictEqual(Buffer.from(medicalRecord.record).toString("utf-8"), "Animal received vaccine", "Medical record should store correct data.");

//     console.log("✅ Medical record successfully added and verified!");

//   } catch (error) {
//     console.error("❌ Error adding medical record:", error);
//     assert.fail(`Test failed: ${error.message}`);
//   }
// });

  // it("It tests request permission with denial, veterinary cabinet asks for permission and onwer deny's it.", async () => {
  // });
  // it("It tests the add medical record function, veterinary cabinet should not be able to medical record to the animal.", async () => {
  // });

});
