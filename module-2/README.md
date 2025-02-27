# Module 2: Rust for Solana

## Getting Started with Rust for Solana Development

### Environment Setup
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI tools
sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"

# Add the BPF target for Solana programs
rustup target add bpfelf-unknown-unknown
```

## Rust Fundamentals for Blockchain

### Memory Safety without Garbage Collection
Rust's ownership system eliminates memory-related bugs at compile time, perfect for blockchain where errors are costly:

```rust
// Ownership example
fn main() {
    let data = String::from("hello"); // data owns the string
    process_data(data);              // ownership transferred to function
    // Using data here would cause a compile error - ownership was moved
}

fn process_data(input: String) {
    println!("Processing: {}", input);
} // input goes out of scope and is freed
```

### Ownership Model and Borrowing Rules
These rules ensure data safety and prevent race conditions:

```rust
fn main() {
    let mut data = String::from("hello");
    
    // Immutable borrow
    let ref1 = &data;
    let ref2 = &data;
    println!("{} and {}", ref1, ref2); // Multiple immutable borrows are fine
    
    // Mutable borrow
    let ref3 = &mut data;
    ref3.push_str(" world");
    println!("{}", ref3);
    
    // Cannot mix mutable and immutable borrows in the same scope
    // println!("{}", ref1); // This would cause a compile error
}
```

### Error Handling and Result/Option Types
Solana programs must handle all possible errors explicitly:

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Or using the ? operator (common in Solana programs)
    fn process() -> Result<(), String> {
        let result = divide(10.0, 0.0)?; // Returns error immediately if Err
        println!("Result: {}", result);
        Ok(())
    }
}
```

## Data Structures and Serialization

### Working with Borsh Serialization
Borsh is the preferred serialization standard for Solana programs:

```rust
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AccountData {
    pub counter: u64,
    pub owner: Pubkey,
    pub data: Vec<u8>,
}

// Serialize
let bytes = account_data.try_to_vec().unwrap();

// Deserialize
let decoded_data = AccountData::try_from_slice(&bytes).unwrap();
```

### Custom Data Types for On-chain Storage
Designing efficient data structures is crucial for on-chain storage:

```rust
#[derive(BorshSerialize, BorshDeserialize)]
pub struct GameState {
    pub is_initialized: bool,
    pub player_one: Pubkey,
    pub player_two: Option<Pubkey>,
    pub board: [u8; 9],
    pub turn: u8,
    pub game_status: GameStatus,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq)]
pub enum GameStatus {
    Active,
    Draw,
    Won { winner: Pubkey },
}
```

### Memory Layout and Optimization
Solana charges rent based on account size, so optimizing is important:

- Use fixed-size arrays instead of vectors when possible
- Consider bit packing for boolean flags
- Align structs to reduce padding
- Use enums with discriminants to save space

## Your First Solana Program

### Program Structure and Entrypoints
Every Solana program has a standard entry point:

```rust
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

// Declare the program entrypoint
entrypoint!(process_instruction);

// Program entrypoint implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    // Program logic goes here
    Ok(())
}
```

### Instruction Data Parsing
Programs interpret the instruction data to determine which action to take:

```rust
#[derive(BorshSerialize, BorshDeserialize)]
pub enum InstructionData {
    Initialize,
    Increment { amount: u64 },
    Decrement { amount: u64 },
    Reset,
}

fn process_instruction(...) -> ProgramResult {
    // Parse instruction data
    let instruction = InstructionData::try_from_slice(instruction_data)?;
    
    match instruction {
        InstructionData::Initialize => initialize_counter(accounts, program_id),
        InstructionData::Increment { amount } => increment_counter(accounts, amount),
        InstructionData::Decrement { amount } => decrement_counter(accounts, amount),
        InstructionData::Reset => reset_counter(accounts),
    }
}
```

### Account Validation and Security Checks
Always validate all accounts before processing instructions:

```rust
fn increment_counter(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    
    // Get account references
    let counter_account = next_account_info(accounts_iter)?;
    let authority = next_account_info(accounts_iter)?;
    
    // Validate authority is a signer
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Validate account ownership
    if counter_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    
    // Process the instruction
    let mut counter_data = Counter::try_from_slice(&counter_account.data.borrow())?;
    counter_data.count += amount;
    
    // Save updated data
    counter_data.serialize(&mut &mut counter_account.data.borrow_mut()[..])?;
    
    Ok(())
}
```

## Advanced Program Concepts

### Program Derived Addresses (PDAs) explained
PDAs allow programs to control accounts without needing a private key:

```rust
// Find a PDA for a counter owned by a specific user
let seeds = &[
    b"counter",
    user.key.as_ref(),
    &[bump_seed], // Bump seed to push the address off the ed25519 curve
];

let (pda, bump) = Pubkey::find_program_address(seeds, program_id);

// Later use the same PDA
let pda = Pubkey::create_program_address(
    &[b"counter", user.key.as_ref(), &[bump]],
    program_id
)?;
```

### Cross-Program Invocation (CPI) Patterns
CPIs allow Solana programs to call other programs:

```rust
// Invoke the System Program to create an account
let ix = solana_program::system_instruction::create_account(
    payer.key,
    new_account.key,
    rent_lamports,
    account_size as u64,
    program_id,
);

solana_program::program::invoke_signed(
    &ix,
    &[payer.clone(), new_account.clone()],
    &[&[b"seed", &[bump]]], // Signer seeds if using PDA
)?;
```

### Atomic Transactions and Error Handling
Solana transactions are atomic - they either completely succeed or fail:

```rust
// If any part fails, the entire transaction is reverted
fn process() -> ProgramResult {
    // Step 1: Validate
    if !validate_something() {
        return Err(ProgramError::InvalidArgument);
    }
    
    // Step 2: Update state
    update_something()?; // Using ? operator for early returns
    
    // Step 3: Final checks
    if !final_check() {
        return Err(CustomError::CheckFailed.into());
    }
    
    Ok(())
}

// Custom error types for better error handling
#[derive(Debug)]
enum CustomError {
    CheckFailed,
    InvalidState,
}

impl From<CustomError> for ProgramError {
    fn from(e: CustomError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
```

## Building the Sample Counter Program

Check out the sample counter program in this module's directory. It demonstrates:
- Basic program structure
- Instruction processing
- Account validation
- Data serialization with Borsh

To build and deploy the sample program:

```bash
# Build the program
cd counter-program
cargo build-bpf

# Deploy to localnet
solana program deploy target/deploy/counter_program.so
```

## Additional Resources

- [Solana Developer Documentation](https://docs.solana.com/developing/programming-model/overview)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Borsh Specification](https://borsh.io/)
- [Solana Cookbook](https://solanacookbook.com/)
