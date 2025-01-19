import * as anchor from "@coral-xyz/anchor";
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { Program, AnchorProvider, Wallet } from "@coral-xyz/anchor";
import { assert } from "chai";
import  IDL  from "../target/idl/fgg.json";
import * as fs from "fs";
import * as path from "path";
// Adjust the import path based on your project structure
 // Adjust the import path based on your project structure

// Function to load the keypair from JSON file
function loadKeypairFromFile(filePath: string): Keypair {
  const secretKeyString = fs.readFileSync(filePath, { encoding: "utf-8" });
  const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
  return Keypair.fromSecretKey(secretKey);
}

describe("fgg", () => {
  // Load keypair from file
  const keypairPath = path.resolve(process.env.HOME || process.env.USERPROFILE, ".solana/.config/keypari.json");
  const userKeypair = loadKeypairFromFile(keypairPath);

  // Create a connection to Solana devnet
  const connection = new Connection("https://api.devnet.solana.com", "confirmed");

  // (async () => {
  //   // Request an airdrop of 2 SOL for the loaded wallet
  //   const airdropSignature = await connection.requestAirdrop(
  //     userKeypair.publicKey,
  //     5 * LAMPORTS_PER_SOL
  //   );
  
  //   // Confirm the transaction
  //   await connection.confirmTransaction(airdropSignature);
  
  //   console.log(`Airdropped 5 SOL to ${userKeypair.publicKey.toBase58()}`);
  // })();
  
  // Set up the provider using Anchor
  const provider = new AnchorProvider(
    connection,
    new Wallet(userKeypair),
    { commitment: "confirmed", preflightCommitment: "confirmed" }
  );
  
  anchor.setProvider(provider);

   // Load the program using the correct program ID and provider
   //const programId = new PublicKey("84A9QmWkUe8ZT1kWnQxBUiPNC5rL4KKW8XgQw8hPfCxe");

  // Load the program using the correct program ID
  const program = new Program(IDL as anchor.Idl,provider);

  let vaultState: PublicKey;
  let vault: PublicKey;
  let vsBump: number;
  let vBump: number;

  before(async () => {
    // Correct the PDA seed calculation for vaultState
    const [vaultStatePda, vaultStateBump] = await PublicKey.findProgramAddress(
      [Buffer.from("state"), userKeypair.publicKey.toBuffer()],
      program.programId
    );

    const [vaultPda, vaultBump] = await PublicKey.findProgramAddress(
      [Buffer.from("vault"), vaultStatePda.toBuffer()],
      program.programId
    );

    vaultState = vaultStatePda;
    vault = vaultPda;
    vsBump = vaultStateBump;
    vBump = vaultBump;

     // Check if the vault_state account already exists
  const vaultStateAccountInfo = await provider.connection.getAccountInfo(vaultState);
  if (vaultStateAccountInfo) {
    console.log("Vault state account already exists, skipping initialization.");
    return; // Skip initialization if the account exists
  }

    await program.methods
      .initialize()
      .accounts({
        user: userKeypair.publicKey, 
        vaultStateBump,      // User account
        vaultBump,
      })
      .signers([userKeypair]) // Sign with the loaded keypair
      .rpc();

    // Assert that the vault_state account has been created
    const vaultStateAccount = await provider.connection.getAccountInfo(vaultState);
    assert.ok(vaultStateAccount !== null, "vault_state account should exist");

    // Assert that the vault account has been created
    const vaultAccountInfo = await provider.connection.getAccountInfo(vault);
    assert.ok(vaultAccountInfo !== null, "vault account should exist");
  });

  it("should deposit funds into the vault", async () => {
    const depositAmount = 500_000_000; // 0.5 SOL

    // Call the deposit method
    await program.methods
      .deposit(new anchor.BN(depositAmount))
      .accounts({
        user: userKeypair.publicKey, 
        vsBump,
        vBump,
      })
      .signers([userKeypair])
      .rpc();

    // Fetch the vault account info after the deposit
    const vaultAccount = await provider.connection.getAccountInfo(vault);
    // Assert that the vault's balance has increased by the deposit amount
    assert.ok(vaultAccount.lamports >= depositAmount, "Vault should have received the deposit");
  });

  it("should withdraw funds from the vault", async () => {
    const withdrawAmount = 200_000_000; // 0.2 SOL

    // Call the withdraw method
    await program.methods
      .withdraw(new anchor.BN(withdrawAmount))
      .accounts({
        user: userKeypair.publicKey,
        vBump,
        vsBump
      })
      .signers([userKeypair])
      .rpc();

    // Assert that the vault balance decreased
    const vaultAccount = await provider.connection.getAccountInfo(vault);
    assert.ok(vaultAccount.lamports < 500_000_000, "vault should have decreased after withdrawal");
  });

  it("should close the vault and return remaining funds", async () => {
    const userBalanceBefore = await provider.connection.getBalance(userKeypair.publicKey);

    // Call the close method
    await program.methods
      .close()
      .accounts({
        user: userKeypair.publicKey,
        vsBump,
        vBump
      })
      .signers([userKeypair])
      .rpc();

    // Assert that the vault account has been closed
    const vaultAccountInfo = await provider.connection.getAccountInfo(vault);
    assert.ok(vaultAccountInfo === null, "vault should no longer exist");

    // Assert that the user's balance has increased by the remaining vault funds
    const userBalanceAfter = await provider.connection.getBalance(userKeypair.publicKey);
    assert.ok(userBalanceAfter > userBalanceBefore, "user should receive remaining vault funds");
  });
});
