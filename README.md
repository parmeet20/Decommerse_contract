<!--
  ====== Welcome ======
  Paste this into your profile’s README.md for a modern, interactive showcase.
-->

<!-- Hero Section -->
<div align="center">
  <h1>🛒 Decommerse</h1>
  <p><em>Decentralized E‑Commerce Smart Contract on Solana</em></p>

  <!-- Badges -->
  <p>
    <a href="https://github.com/parmeet20/Decommerse_contract/stargazers">
      <img src="https://img.shields.io/github/stars/parmeet20/Decommerse_contract?style=social" alt="Stars" />
    </a>
    <a href="https://github.com/parmeet20/Decommerse_contract/network/members">
      <img src="https://img.shields.io/github/forks/parmeet20/Decommerse_contract?style=social" alt="Forks" />
    </a>
    <a href="https://github.com/parmeet20/Decommerse_contract/blob/main/LICENSE">
      <img src="https://img.shields.io/github/license/parmeet20/Decommerse_contract" alt="License" />
    </a>
    <a href="https://github.com/parmeet20/Decommerse_contract/issues">
      <img src="https://img.shields.io/github/issues/parmeet20/Decommerse_contract" alt="Issues" />
    </a>
    <a href="https://crates.io/crates/anchor-lang">
      <img src="https://img.shields.io/crates/v/anchor-lang" alt="Anchor Version" />
    </a>
  </p>

  <!-- Demo GIF / Screenshot -->
  <p>
    <img src="https://raw.githubusercontent.com/parmeet20/Decommerse_contract/main/docs/demo.gif" alt="Decommerse Demo" width="80%" />
  </p>
</div>

---

## 🚀 Why Decommerse?

<p align="center">
  Harness the power of Solana’s blazing-fast throughput and negligible fees to build a truly trustless marketplace—no middleman, no downtime, no surprises.
</p>

---

## ✨ Features

<details>
  <summary><strong>🗄️ On‑Chain Profiles</strong> <em>(Decentralized Identity)</em></summary>
  <br>
  Every user gets a unique Program-Derived Address (PDA) as their profile.  
  - **Immutable Ownership**: Only the profile owner (wallet) can update personal data.  
  - **Rich Metadata**: Store username, avatar URI, bio, reputation score.  
  - **Future‑Proof**: Easily extendable to include KYC badges, social links, etc.  
</details>

<details>
  <summary><strong>📦 Product Listings</strong> <em>(Dynamic Catalog)</em></summary>
  <br>
  Sellers can mint product accounts on‑chain with rich metadata.  
  - **Metadata URI**: Link to off‑chain JSON for images, descriptions, specs.  
  - **Configurable Pricing**: Set price per unit in SOL or SPL tokens.  
  - **Access Control**: Only the creator PDA can modify or delist items.  
</details>

<details>
  <summary><strong>💳 Purchase Flow</strong> <em>(Trustless Transactions)</em></summary>
  <br>
  Secure, escrow‑style purchase mechanism without a centralized intermediary.  
  - **Funds Escrow**: Buyer’s payment is held in a PDA until seller confirmation.  
  - **Atomic Settlement**: On purchase approval, funds transfer and stock update happen in one transaction.  
  - **Event Logs**: Emit structured logs for front‑end listeners and analytics.  
</details>

<details>
  <summary><strong>📊 Inventory Control</strong> <em>(Real‑Time Stock)</em></summary>
  <br>
  Keep your marketplace up‑to‑date with live stock adjustments.  
  - **Stock Validation**: Reject purchases when stock ≤ 0.  
  - **Batch Updates**: Sellers can restock multiple items in a single instruction.  
  - **Historical Tracking**: Maintain a log of all stock changes for audits.  
</details>

---

## 📂 Code Highlights

| 📄 File                                                                                           | 🔍 What It Does                                          |
| ------------------------------------------------------------------------------------------------ | -------------------------------------------------------- |
| [init.rs](programs/decommerse/src/instructions/init.rs)                                         | Bootstraps on‑chain state & config PDAs                  |
| [create_profile.rs](programs/decommerse/src/instructions/create_profile.rs)                       | PDA derivation & initialization of user profile accounts |
| [create_product.rs](programs/decommerse/src/instructions/create_product.rs)                       | Mints new product accounts with metadata                 |
| [purchase.rs](programs/decommerse/src/instructions/purchase.rs)                                   | Escrow payment, validate stock, finalize sale            |
| [update_stock.rs](programs/decommerse/src/instructions/update_stock.rs)                         | Increase/decrease product inventory                      |
| [mod.rs](programs/decommerse/src/instructions/mod.rs)                                            | Re‑exports all instruction handlers                      |

---

## 🛠️ Getting Started

```bash
# 1. Clone the repo
git clone https://github.com/parmeet20/Decommerse_contract.git

# 2. Change into project directory
cd Decommerse_contract

# 3. Build & Deploy (requires Solana CLI + Anchor)
anchor build
anchor deploy
