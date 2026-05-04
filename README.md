# RentChain Pay

## One-line
A blockchain-based rental payment system that secures rent transactions using Stellar Soroban smart contracts.

## Problem
Tenants in the Philippines often pay rent in cash without proof, leading to disputes and unfair charges.

## Solution
RentChain Pay records rental payments on Stellar, locking funds in a smart contract until landlords confirm receipt.

## Timeline
Week 1: Smart contract development  
Week 2: Frontend integration  
Week 3: Wallet + testnet deployment  

## Stellar Features
- Soroban smart contracts  
- USDC / XLM transfers  
- Trustlines  

## Vision
To eliminate rental fraud and bring transparent housing payments to Southeast Asia.

## Prerequisites
- Rust
- Soroban CLI

## Build
soroban contract build

## Test
cargo test

## Deploy
soroban contract deploy --network testnet

## Sample CLI
register payment:
R1, tenant, landlord, 1000

confirm payment:
R1, landlord

## License
MIT