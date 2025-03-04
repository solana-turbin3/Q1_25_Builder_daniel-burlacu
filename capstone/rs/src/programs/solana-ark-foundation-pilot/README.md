<div align="center">
  <h1>Solana Ark Foundation</h1><h4>"The time to act is now ! ... before their silence becomes our legacy."</h4>
  <img src="https://bafkreibllcqfjk5ch26tdq7sqotkq3xxlymivip6ta7rdjhaf2qccnzc7u.ipfs.flk-ipfs.xyz" alt="Logo" width="200">
</div>

---

### Project Details
- **Project:** [Shortcut](https://github.com/solana-turbin3/Q4_SOL_DanielB/tree/master/rs/src/programs/solana-ark-foundation)
- **Program ID:** H6eB3LauYEk4RxtjNH5dwteGAH8i5qy8ukdiSvtnYmhp
- **Block Explorer:** [Program-Address](https://explorer.solana.com/address/H6eB3LauYEk4RxtjNH5dwteGAH8i5qy8ukdiSvtnYmhp?cluster=devnet)

---

### Project Description

Solana ARK Foundation – A Blockchain-Powered Future for Animal Welfare and Ecological Impact

The Solana ARK Foundation leverages blockchain to create a secure, transparent, and decentralized ecosystem for animal welfare and environmental stewardship. Our platform empowers veterinarians, researchers, animal shelters, pet owners, and conservationists to make an impact through reliable data and eco-friendly practices that benefit animal and environmental health.

---

### Core Functionalities & Tests

There is an Admin Authority that controls most of the program.
An entity - most likely an veterinary cabinet can join the eco-system.
It can add an animal owner, if the veterinary cabinet entity exists in the system.
It can add an animal for the owner, if the veterinary cabinet entity exists in the system.
The veterinary cabinet can mint an NFT as proof of license or to prove that it belongs to this eco-system.
The Animal owner can mint an NFT as proof that it belongs to this eco-system.
The Animal owner can mint an NFT for the animal that it own's as proof of ownership.

Some of the NFT's examples you can find here:
- **Veterinary Cabinet:** [NFT-Address](https://explorer.solana.com/address/8w6nxoAjxA5yqzNZYf2odRfdQooJp8dAQ7XkfXap4M5D?cluster=devnet)
- **Animal Owner:** [NFT-Address](https://explorer.solana.com/address/J8thCVwinehmRoeTDRqAmbmYTVtuPNJ9kYdJuBAuYhBa?cluster=devnet)
- **Animal:** [NFT-Address](https://explorer.solana.com/address/4QndEkL3FmCkgpCFrZT14hwbSWHdDpRaxEhLkRSCNSdh?cluster=devnet)

An preview of the test checks:
   
```markdown
  ✔ Initializes the Treasury PDA (47ms)
Funding Treasury PDA with 0.0008 SOL...
Treasury funded successfully.
Transaction Signature: 3naDteVZhV3azNZCCgsUpZc49W1ajjXkE9eNxY8iXdCWqZuYzVtRCcVWNZijBdC83H1ZpNRozvxHVLqmUZN8KeqM
    ✔ Adds a new veterinary cabinet (870ms)
Created Veterinary Cabinet NFT with mint: 8w6nxoAjxA5yqzNZYf2odRfdQooJp8dAQ7XkfXap4M5D
    ✔ Mints a Veterinary Cabinet NFT using Metaplex (439ms)
Transaction Signature: 3Sfxrc5Pjn1R2cmi7bDdQHpfApMcSXccJZH5HASSqMfRbWyKbzi7pzC1VfQDDX79AvBLkQ2EsotXD96V5SH9iMTt
Animal owner added successfully: EkwP9ubRM9vVhVJpWNJF9eYaR1yT1G83WshaAxDzNdbJ
    ✔ Adds a new animal owner (8204ms)
Created Animal Owner NFT with mint: J8thCVwinehmRoeTDRqAmbmYTVtuPNJ9kYdJuBAuYhBa
    ✔ Mints a Animal Owner NFT using Metaplex (272ms)
Transaction Signature: 4hm9bK8b3VPGAkJNSp9xyRKjBKQzBVaPnrWGDKdPjnuT8Y88y2C9yiymU96hnBsrKBLPVQKcUUiGVh2enLX2738f
Animal added successfully: 6dDj9Nu8qVB6MNc1uFf1avi74JxtD8iAzkiRQWboZ2Ek
    ✔ Adds a new animal (357ms)
Created Animal NFT with mint: 4QndEkL3FmCkgpCFrZT14hwbSWHdDpRaxEhLkRSCNSdh
    ✔ Mints an Animal NFT using Metaplex (445ms)
```

 ---

### Files included in this repo
- **Project Architecture**
- **Presentation**
- **Short video Presentation**
