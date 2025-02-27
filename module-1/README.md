# Module 1: Solana Fundamentals

## Solana Architecture Overview

### Proof of History (PoH) Consensus Mechanism
- Proof of History is a novel timekeeping method that enables high throughput
- Creates a historical record that proves events occurred at specific moments in time
- Provides a verifiable delay function (VDF) that requires sequential computation
- Allows validators to process transactions without waiting for network-wide consensus

### High Throughput and Low Transaction Costs
- Capable of processing 50,000+ transactions per second (TPS)
- Average transaction cost is ~$0.0005 (or less)
- Achieves speed through parallel transaction processing using GPU capabilities
- Optimized network communication protocols to minimize latency

### Scalability Solutions and Validator Infrastructure
- Horizontally scalable architecture through state sharding (in development)
- Validator requirements: high-bandwidth connections and powerful hardware
- State compression techniques to manage blockchain bloat
- Turbine block propagation protocol for efficient network communication

## Account Model Deep Dive

### Comparing Account Model with UTXO Model
- Solana uses an account-based model similar to Ethereum (not UTXO like Bitcoin)
- Accounts store state directly rather than through transaction references
- Allows for more efficient smart contract development and execution
- Enables simpler transaction logic and parallelization

### Account Ownership and Rent Economics
- Every account is owned by a program that controls data modification
- Rent is charged for storing data on-chain (2 years of rent exemption recommended)
- Rent-exempt accounts require a minimum balance based on their size
- Accounts can be closed to reclaim SOL used for rent exemption

### State Storage Patterns and Best Practices
- Program-Derived Addresses (PDAs) for deterministic account generation
- Cross-Program Invocation (CPI) for composability between programs
- Account data serialization using Borsh or custom serialization
- State management patterns: single accounts, collections, and relationships

## Transaction Lifecycle

### Instructions, Signatures, and Atomicity
- Transactions contain one or more instructions targeting specific programs
- Each instruction specifies: program ID, accounts to use, and instruction data
- Multiple signatures can authorize a single transaction
- Transactions are atomic: they either fully execute or completely fail

### Fee Structure and Prioritization
- Fees based on signature count and computational resources used
- Prioritization fee mechanism to increase transaction priority
- Fee markets develop during network congestion
- Validators prioritize transactions with higher fees

### Transaction Confirmation and Finality
- Transactions confirmed in ~400ms on average
- Finality achieved after 32 confirmations (~13 seconds)
- Confirmation states: processed, confirmed, and finalized
- Practical considerations for UX design around confirmation times

## Development Environment Setup

### Installing Solana CLI Tools
- Install Rust and Cargo using rustup
- Install Solana CLI tools using the installation script
- Configure your local environment and keypairs
- Learn basic CLI commands for account and network interaction

### Setting up Wallets (Phantom, Solflare)
- Install browser extensions for development testing
- Create new wallets or import existing ones
- Configure wallets for different networks (mainnet, testnet, devnet)
- Best practices for secure key management

### Connecting to Development Clusters
- Local development with `solana-test-validator`
- Working with devnet for early-stage testing
- Testnet for pre-production validation
- RPC endpoints and connection strategies

## Sample Projects

The [sample-project](./sample-project) folder contains a basic Solana program that demonstrates the fundamental concepts covered in this module. Follow the instructions in that directory to build and deploy your first Solana program.