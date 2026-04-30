# GigSync

**One-line description:** A Soroban-powered escrow dApp enabling trustless, low-fee micro-milestone payments for freelancers in SEA.

## Problem
A freelance web developer in Cebu, Philippines frequently loses out on $50–$100 initial milestone payments because Western clients refuse to pay $25 traditional wire fees for small increments, forcing the developer to take on unpaid risk.

## Solution
Clients lock the full project budget (USDC) into a Soroban smart contract escrow. The freelancer can draw from this pool in small increments as micro-milestones are approved. By leveraging Stellar’s sub-cent fees, continuous micro-dispersals become financially viable and instant.

## Timeline
* **Day 1:** Smart contract development and unit testing.
* **Day 2:** Frontend web app integration with Freighter wallet.
* **Day 3:** Testnet deployment, local anchor mapping, and final pitch prep.

## Stellar Features Used
* Soroban smart contracts (Escrow state management)
* USDC transfers (Cross-border stability)
* Trustlines (Wallet preparation for asset receipt)

## Vision and Purpose
To eliminate cross-border friction and client-ghosting risk for gig workers in emerging economies, enabling true meritocratic earning without banking penalties.

## Prerequisites
* Rust toolchain (`rustup target add wasm32-unknown-unknown`)
* Soroban CLI (`cargo install --locked soroban-cli`)

## Build Instructions
To build the optimized WebAssembly contract:
```bash
soroban contract build

## Deployed Contract Link
[1] https://stellar.expert/explorer/testnet/tx/c4d7c69955bd39647c30e41b15c10f45fd74ce8173d4965b439eb5071fbb23b7
[2] https://lab.stellar.org/r/testnet/contract/CDP66PTOYQ4QLKK3TJUJABJIZDDILHFJATLUDMAC6BUPEEQLHFZATRD2

## License

MIT License