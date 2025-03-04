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
- **Programs ID:** veterinary:FaeFRU52JE9MKpQuq1EznrZBRBVAEb8vR5gu7XnDuHbg |animal: DZHj23G3RkSSZQ6pA9CWGYopCuFnUZ8WGJcnGnrVRY1i | animal-owner | Ctjucr5AAqnJkooGQS4WYbbpqyv9cg1srvi93ucSffKn
- - **Block Explorers:**
  - [Veterinary](https://explorer.solana.com/address/FaeFRU52JE9MKpQuq1EznrZBRBVAEb8vR5gu7XnDuHbg?cluster=devnet)
  - [Animal](https://explorer.solana.com/address/DZHj23G3RkSSZQ6pA9CWGYopCuFnUZ8WGJcnGnrVRY1i?cluster=devnet)
  - [Animal-Owner](https://explorer.solana.com/address/Ctjucr5AAqnJkooGQS4WYbbpqyv9cg1srvi93ucSffKn?cluster=devnet).
  - 
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
✅ Veterinary PDA: FY5jtWQDnESQU6utxZxR5tghy7k5gKZbgYKCrfsBzQxY
✅ Owner PDA: 85c5DowL76143M5F2FmjTJ1RtyFDYrUk6mJdCH3gDrtd
✅ Animal PDA: 3Vgk6ieHeQ49awnjjsGG7ZnfiWAn3pgFra7aBgoerKVH
✅ Vet Authority PDA: GvZ27cD6HkTFPWasXXjecffWG9gtAqfQqxe6w9h2Lcih
✅ Veterinary PDA already exists: FY5jtWQDnESQU6utxZxR5tghy7k5gKZbgYKCrfsBzQxY
    ✔ It initializes the veterinary account if it does not exist (403ms)
✅ Owner PDA already exists: 85c5DowL76143M5F2FmjTJ1RtyFDYrUk6mJdCH3gDrtd
    ✔ It initialize the animal owner ! (269ms)
✅ Animal PDA already exists: 3Vgk6ieHeQ49awnjjsGG7ZnfiWAn3pgFra7aBgoerKVH
    ✔ It initializes the animal program. (104ms)
🔍 Veterinary PDA: FY5jtWQDnESQU6utxZxR5tghy7k5gKZbgYKCrfsBzQxY
🔍 Owner PDA: 85c5DowL76143M5F2FmjTJ1RtyFDYrUk6mJdCH3gDrtd
🔍 Animal PDA: 3Vgk6ieHeQ49awnjjsGG7ZnfiWAn3pgFra7aBgoerKVH
🚀 Requesting authority from animal owner...
✅ Authority request transaction submitted: 22P7b4yCMHxBwsP8c1Vpc8zDbjisupHUUsokERgC8McTyDxmCszGg1x8aAMFK9R5yeRveQngx5ycXZnSnXYWKfMh
🔍 Request Status (Pending): 0
    ✔ It creats authorisation request. (3625ms)
🔍 Calling on-chain `check_pending_requests` function...
✅ Found 1 authority requests
🔍 Authority Request #1:
    🟢 Public Key: BnvegXduwV23p3NvKtKmQxigmFrZArc7sMJU4zTnLWXF
    🟢 Vet Pubkey: 3hoShi4sSwVb2iLUCPG8nJtrGHRApiCBT5pYTMePResZ
    🟢 Owner Pubkey: 85c5DowL76143M5F2FmjTJ1RtyFDYrUk6mJdCH3gDrtd
    🟢 Animal Pubkey: 3Vgk6ieHeQ49awnjjsGG7ZnfiWAn3pgFra7aBgoerKVH
    🔵 Status: Pending
--------------------------------------------------
✅ On-chain request scan executed successfully, transaction: RgG6FD7EVnAWMVsQXgCzDApwj1zbdEpVHpd5CyuBY5Pv3o9qhHWWkbC3KoFLTz5mes27JomBfjUxV6KER497D7j
    ✔ It checks for any outstanding authority requests via the on-chain function. (8037ms)
🔍 Vet Authority Request PDA: BnvegXduwV23p3NvKtKmQxigmFrZArc7sMJU4zTnLWXF
🔍 Vet Authority PDA: GvZ27cD6HkTFPWasXXjecffWG9gtAqfQqxe6w9h2Lcih
🚀 Approving request...
✅ Authority request approved, transaction: 3LdGBHbTd3Z2vkjUTcBVwuU29hYWDcBpNdqXYLeYH7UaowKMDxdMeaq28BGSvrftBTYcs5X7FfzUQDj1joP19w9R
🔍 Request Status (Approved): 1
    ✔ It approves the veterinary cabinet's authority request. (3239ms)
🔍 Fetching all VetAuthority accounts...
✅ Found 1 VetAuthority accounts
🔍 VetAuthority #1: GzRikKmr8XhkLXVQbh72AGzY5FFovJcKb6paLm7B2bCW
    🟢 Vet Pubkey: 3hoShi4sSwVb2iLUCPG8nJtrGHRApiCBT5pYTMePResZ
    🟢 Animal Pubkey: 3Vgk6ieHeQ49awnjjsGG7ZnfiWAn3pgFra7aBgoerKVH
    🔵 is_authorized: 1
--------------------------------------------------
    ✔ Lists all VetAuthority accounts and extracts their stored vet_pubkey (101ms)
🔍 Deriving Medical Record PDA...
🔍 Medical Record PDA: BfYQ4f1bTBniCjzLootysHicHMvgq97gtDYhSddkvV4g
🚀 Adding medical record...
✅ Medical record transaction: 488XFKPnfsrXFrhsPsKkXG8y4TNzFdP3C3KTpyLqw5oUQK9ihBBgpqxEZUytgD3urvpAk3x2GbuFkChrqbYPkpx1
✅ Medical record successfully added and verified!
    ✔ It adds a medical record for an animal (8010ms)
🔍 Deriving Behaviour Record PDA...
🔍 Behaviour Record PDA: CZtuGJXNiGvrxV5cSG4kySYsckyGkcqyG8rBagic2dai
🚀 Adding behaviour record...
✅ Behaviour record transaction: bCvoCmCSuC8iDDLN7yLbaZvZyv6rXJvw9Q2WwnaDdpQWcgXUoR7dA8cccJpXcLNou9EhwK8qEWgQjS8xwfGLoq3
✅ Behaviour record successfully added and verified!
    ✔ It adds a behaviour record for an animal (4310ms)

```

 ---

### Files included in this repo
- **Project Architecture**
- **Presentation**
- **Short video Presentation**
