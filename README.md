# CredChain 🎓⛓️

> Built by SolveForge — We Solve. We Build. We Impact.

CredChain is a decentralized student reputation and credential verification system built on Stellar Soroban. African university students can store their verified skills, credentials, and endorsements on-chain — making their talent visible and trustless to employers and companies worldwide.

## Why CredChain Exists

African students have real skills but no verifiable proof. Employers cannot trust a PDF certificate. CredChain fixes that by putting student credentials on the Stellar blockchain — permanent, tamper-proof, and globally accessible.

## Part of the SolveForge Ecosystem

CredChain is a product of SolveForge (https://github.com/SolveForgeHQ) — a technology company building problem-solving products for African students, creators, and entrepreneurs.

It powers the on-chain reputation layer of CampusGig (https://campusgig.com.ng) — a live verified student platform expanding across Africa.

## Tech Stack

Smart Contract: Soroban Rust
Frontend: React TypeScript
Backend: Node.js TypeScript
Blockchain: Stellar
Integration: CampusGig API

## Project Structure

credchain/
├── contracts/
│   └── reputation/
│       ├── src/
│       │   └── lib.rs
│       └── Cargo.toml
├── frontend/
│   ├── src/
│   │   ├── components/
│   │   └── pages/
│   └── package.json
├── backend/
│   └── src/
├── docs/
│   └── CONTRIBUTING.md
└── README.md

## Getting Started

### Prerequisites
- Rust and Cargo installed
- Stellar CLI installed
- Node.js 18+
- A Stellar testnet account

### Clone the repo

git clone https://github.com/SolveForgeHQ/credchain.git
cd credchain

### Build the contract

cd contracts/reputation
cargo build --target wasm32-unknown-unknown --release

### Run tests

cargo test

### Run frontend

cd frontend
npm install
npm run dev

## Features

- Store verified student credentials on-chain
- Skill endorsement system
- Reputation score calculator
- Trustless credential verification
- CampusGig integration
- Multi-university support coming soon
- Employer verification portal coming soon

## Contributing

We welcome contributions from developers worldwide especially African developers breaking into Web3.

Read our Contributing Guide in docs/CONTRIBUTING.md to get started.

Issues labeled bounty have USDC rewards via Drips Wave on Stellar. Fix the issue get your PR merged and earn USDC.

## Open Issues

Check our Issues page for tasks you can work on. Issues labeled good first issue are perfect for beginners.

## Contact

Email: solveforgehq@gmail.com
X: @SolveForgeHQ
LinkedIn: SolveForge
GitHub: github.com/SolveForgeHQ

## License

MIT License — free to use and contribute.

Built with love by SolveForge
We Solve. We Build. We Impact.
