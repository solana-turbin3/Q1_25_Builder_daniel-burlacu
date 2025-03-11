<div align="center">
  <h1>Solana Ark Foundation</h1><h4>"The time to act is now ! ... before their silence becomes our legacy."</h4>
  <img src="https://bafkreibllcqfjk5ch26tdq7sqotkq3xxlymivip6ta7rdjhaf2qccnzc7u.ipfs.flk-ipfs.xyz" alt="Logo" width="200">
</div>

---

### Project Details Q4 2024
- **Project:** [First Part - NFT's](https://github.com/solana-turbin3/Q4_SOL_DanielB/tree/master/rs/src/programs/solana-ark-foundation)
- **Program ID:** H6eB3LauYEk4RxtjNH5dwteGAH8i5qy8ukdiSvtnYmhp
- **Block Explorer:** [Program-Address](https://explorer.solana.com/address/H6eB3LauYEk4RxtjNH5dwteGAH8i5qy8ukdiSvtnYmhp?cluster=devnet)

### Project Details Q1 2025
 **Project:**[website-presentation](https://www.solana-ark-foundation.xyz)
- **Programs ID:**
  - veterinary: FaeFRU52JE9MKpQuq1EznrZBRBVAEb8vR5gu7XnDuHbg
  - animal: DZHj23G3RkSSZQ6pA9CWGYopCuFnUZ8WGJcnGnrVRY1i
  - animal-owner: Ctjucr5AAqnJkooGQS4WYbbpqyv9cg1srvi93ucSffKn
- **Block Explorers:**
  - [Veterinary](https://explorer.solana.com/address/FaeFRU52JE9MKpQuq1EznrZBRBVAEb8vR5gu7XnDuHbg?cluster=devnet)
  - [Animal](https://explorer.solana.com/address/DZHj23G3RkSSZQ6pA9CWGYopCuFnUZ8WGJcnGnrVRY1i?cluster=devnet)
  - [Animal-Owner](https://explorer.solana.com/address/Ctjucr5AAqnJkooGQS4WYbbpqyv9cg1srvi93ucSffKn?cluster=devnet).

### Project Description

Solana ARK Foundation – A Blockchain-Powered Future for Animal Welfare and Ecological Impact

The Solana ARK Foundation leverages blockchain to create a secure, transparent, and decentralized ecosystem for animal welfare and environmental stewardship. Our platform empowers veterinarians, researchers, animal shelters, pet owners, and conservationists to make an impact through reliable data and eco-friendly practices that benefit animal and environmental health.

---

### Core Functionalities & Tests for Q5
There are 3 programs: Veterinary, Animal, Animal-Owner
Scenario 1: Al programs area being deployed
Scenario 2: Veterinary cabinet creates the Animal & the Animal Owner
Scenario 3: Veterinary cabinet request authorisation to add data from the Animal Owner
Scenario 4: Animal-Owner accepts/denies the request
Scenario 5: Veterinary cabinet adds medical data on the animal's program
Scenario 6: Veterinary cabinet adds Behaviour data on the animal's program

Some of the NFT's examples from Q4 can be found here:
- **Veterinary Cabinet:** [NFT-Address](https://explorer.solana.com/address/8w6nxoAjxA5yqzNZYf2odRfdQooJp8dAQ7XkfXap4M5D?cluster=devnet)
- **Animal Owner:** [NFT-Address](https://explorer.solana.com/address/J8thCVwinehmRoeTDRqAmbmYTVtuPNJ9kYdJuBAuYhBa?cluster=devnet)
- **Animal:** [NFT-Address](https://explorer.solana.com/address/4QndEkL3FmCkgpCFrZT14hwbSWHdDpRaxEhLkRSCNSdh?cluster=devnet)

An preview of the test checks:
```markdown
     solana-ark-foundation-pilot
✅ Veterinary PDA: HbVsrRNpcDQ9j4P4G7f1EtfPQvHJrQShtaYErdj1otML
✅ Owner PDA: 2Fg7ERHjLNnvcoha2r7WkXr3Hhfp9LDZsgQDgAW77LfW
✅ Animal PDA: YPD8wczXpG7S1T9KQbsdd1gc9QVNT6RmA2KtSDk6c4J
⚠️ Veterinary account does not exist, initializing...
✅ Veterinary account initialized, transaction signature: 52yyW13eB7LnDtKWBndR75UDSTnrE1oqnddykSnpRMTBLFN5YnGhq6SgXdUPGSSpKnNGwg9Gnsrq6nhbbxMKjJju
    ✔ It initializes the veterinary account if it does not exist (2195ms)
⚠️ Owner account does not exist, initializing...
✅ Owner account initialized, transaction signature: 5rMPjRtgh3ddwFMKfymZ5C73XkzdCGceyGgL2NN897ogpsjBTDtpsVV2yguD9Xtu3b7e1FCRC1QP5dKAyGyc59BP
    ✔ It initialize the animal owner ! (2060ms)
⚠️ Animal account does not exist, initializing...
🔍 Owner PDA: 2Fg7ERHjLNnvcoha2r7WkXr3Hhfp9LDZsgQDgAW77LfW
✅ Animal account initialized, transaction signature: 26dAUu83u5vKPKychina4oF34QfELgsiYqMoKrgBFSvbAMxpnw6N4yRDRaHNYX2zGdaLCNqdLmECBrWt4PRAUM2x
    ✔ It initializes the animal program. (3190ms)
🔍 Veterinary PDA: HbVsrRNpcDQ9j4P4G7f1EtfPQvHJrQShtaYErdj1otML
🔍 Owner PDA: 2Fg7ERHjLNnvcoha2r7WkXr3Hhfp9LDZsgQDgAW77LfW
🔍 Animal PDA: YPD8wczXpG7S1T9KQbsdd1gc9QVNT6RmA2KtSDk6c4J
🚀 Requesting authority from animal owner...
✅ Authority request transaction submitted: 28U2qTgLNx9Tc8nrWQQL5N4FSGFHawa4jVAJLL4ySpfw5mcZFan72BYvF59A64yAZ9rrCwPs5UK37VUgeJpoKyUw
🔍 Request Status (Pending): 0
    ✔ It creats authorisation request. (3052ms)
🔍 Calling on-chain `check_pending_requests` function...
✅ Found 1 authority requests
🔍 Authority Request #1:
    🟢 Public Key: 13YHKs5sReF9C5NfstAV5HqyFyCvH1ZgWbGr7Esj3zWY
    🟢 Vet Pubkey: 3hoShi4sSwVb2iLUCPG8nJtrGHRApiCBT5pYTMePResZ
    🟢 Owner Pubkey: 2Fg7ERHjLNnvcoha2r7WkXr3Hhfp9LDZsgQDgAW77LfW
    🟢 Animal Pubkey: YPD8wczXpG7S1T9KQbsdd1gc9QVNT6RmA2KtSDk6c4J
    🔵 Status: Pending
--------------------------------------------------
✅ On-chain request scan executed successfully, transaction: 2HA5vHNxQUVwzndgTckrvEtbJW6rfiD4igqfkdRSiM9TTUJA5v2XR8sBX9EHXxks94BqemvGtgdEHoVTMFSpPWBi
    ✔ It checks for any outstanding authority requests via the on-chain function. (2969ms)
🔍 Vet Authority Request PDA: 13YHKs5sReF9C5NfstAV5HqyFyCvH1ZgWbGr7Esj3zWY
🔍Passing the Vet Authority PDA: H77uQYzXxcNyRokRhzS6N38ZA3Jq9mBbKDCpurekfccH
🚀 Approving request...
✅ Authority request approved, transaction: 4724HbSGQM2Cjn1g139cUQPBig33eXRjkMqsqWHKDXmf3a7b4GKQCLZ6AMe1ScZFGeixsz1hdehJdBkcbBrmMHfm
🔍 Request Status (Approved): 0
    ✔ It approves the veterinary cabinet's authority request. (2258ms)
🔍 Searching VetAuthority accounts for VetPubkey: 3hoShi4sSwVb2iLUCPG8nJtrGHRApiCBT5pYTMePResZ...
✅ Found 1 total VetAuthority accounts
🔍 VetAuthority #1: H77uQYzXxcNyRokRhzS6N38ZA3Jq9mBbKDCpurekfccH
    🟢 Vet Pubkey: 3hoShi4sSwVb2iLUCPG8nJtrGHRApiCBT5pYTMePResZ
    🟢 Animal Pubkey: YPD8wczXpG7S1T9KQbsdd1gc9QVNT6RmA2KtSDk6c4J
    🔵 is_authorized: 1
--------------------------------------------------
    ✔ Lists all VetAuthority accounts and extracts their stored vet_pubkey (206ms)
🔍 Checking Vet Authority...
🔍 Vet Authority PDA: H77uQYzXxcNyRokRhzS6N38ZA3Jq9mBbKDCpurekfccH
✅ Vet authority check passed!
    ✔ It checks if veterinary cabinet has authority for an animal (103ms)
🔍 Deriving Medical Record PDA...
🔍 Medical Record PDA: HHDqdHaFtBZ8VfZsT1kf9tNykxN6PpBaFkta8pt6xYKP
⚠️ Medical record does not exist, creating a new one...
🚀 Adding medical record...
✅ Medical record transaction: JioMjD6BLgsehEFaiR8qgFwN6MZ9LJzvUzjWdQpXtXZjfwt2oq8tG5nqr7gY81sJDW8L7t6xpwwZNSfq1fjew2L
✅ Medical record successfully added and verified!
    ✔ It adds a medical record for an animal (3600ms)
🔍 Deriving Behaviour Record PDA...
🔍 Behaviour Record PDA: CAAwEFcSSZH9VRaAqUwGHiTs2WtSTApRboQBnwbVC6B2
⚠️ Behaviour record does not exist, creating a new one...
🚀 Adding behaviour record...
✅ Behaviour record transaction: 3qx7i6LE6fY7BjPuq5JXjZKJgxkBdxTQzchoJWYVbRrmDFQVQ1VzMqNcPs4HLi9QBDzszGuYMr4UbBtd7xrHTKh3
✅ Behaviour record successfully added and verified!
    ✔ It adds a Behaviour record for an animal (1966ms)


  10 passing (27s)

```

 ---

### Files included in this repo
- **Project Architecture**
- **Presentation**
- **Short video Presentation**
