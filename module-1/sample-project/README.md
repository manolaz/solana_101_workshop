# Sample Project: Hello Solana

This sample project demonstrates Solana fundamentals by creating a simple program that:
1. Stores a message on the Solana blockchain
2. Updates the message
3. Reads the message

## Project Structure

```
sample-project/
├── program/                # Solana program (smart contract)
│   ├── src/
│   │   └── lib.rs          # Program logic
│   └── Cargo.toml          # Rust dependencies
└── client/                 # JavaScript client
    ├── src/
    │   └── index.ts        # Client code to interact with the program
    ├── package.json        # Node.js dependencies
    └── tsconfig.json       # TypeScript configuration
```

## Prerequisites

- Rust and Cargo
- Solana CLI tools v1.16.0 or later
- Node.js v16 or later
- yarn or npm

## Setup Instructions

### 1. Install Rust and Solana CLI

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup component add rustfmt clippy

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"
```

### 2. Configure Solana CLI

```bash
solana config set --url localhost
solana-keygen new --force
```

### 3. Start a Local Validator

```bash
solana-test-validator
```

Keep this running in a separate terminal window.

### 4. Build and Deploy the Program

```bash
cd program
cargo build-bpf
solana program deploy target/deploy/hello_solana.so
```

Note the program ID that is displayed after deployment.

### 5. Set Up the Client

```bash
cd ../client
npm install
```

Edit `src/index.ts` to update the program ID with the one you received after deployment.

### 6. Run the Client

```bash
npm start
```

## Program Explanation

The Solana program in this example:

1. Creates an account to store message data
2. Provides instructions to initialize and update the message
3. Demonstrates proper account ownership and data handling
4. Shows how to serialize and deserialize account data

## Client Explanation

The TypeScript client demonstrates:

1. Connecting to a Solana cluster
2. Creating a transaction with program instructions
3. Signing and sending transactions
4. Reading data from program accounts

## Next Steps

After understanding this example, try to:
1. Add more functionality to the program
2. Create additional instructions
3. Modify the account structure
4. Experiment with multiple accounts and relationships
