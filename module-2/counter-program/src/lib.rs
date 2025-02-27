use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Define program entrypoint
entrypoint!(process_instruction);

// Counter state stored in the account
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Counter {
    pub count: u64,
    pub owner: Pubkey,
}

// Instructions supported by the program
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum CounterInstruction {
    Initialize,
    Increment { amount: u64 },
    Decrement { amount: u64 },
    Reset,
}

// Program entrypoint implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Decode instruction
    let instruction = CounterInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        CounterInstruction::Initialize => initialize_counter(program_id, accounts),
        CounterInstruction::Increment { amount } => increment_counter(program_id, accounts, amount),
        CounterInstruction::Decrement { amount } => decrement_counter(program_id, accounts, amount),
        CounterInstruction::Reset => reset_counter(program_id, accounts),
    }
}

// Initialize a new counter
fn initialize_counter(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    
    // Get account references
    let counter_account = next_account_info(accounts_iter)?;
    let authority = next_account_info(accounts_iter)?;

    // Security checks
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    if counter_account.owner != program_id {
        msg!("Counter account does not belong to this program");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Initialize counter state
    let counter = Counter {
        count: 0,
        owner: *authority.key,
    };

    // Serialize and store the counter state in the account data
    counter.serialize(&mut &mut counter_account.data.borrow_mut()[..])?;
    msg!("Counter initialized with count 0");

    Ok(())
}

// Increment the counter by a specified amount
fn increment_counter(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    
    // Get account references
    let counter_account = next_account_info(accounts_iter)?;
    let authority = next_account_info(accounts_iter)?;

    // Validate account ownership
    if counter_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    
    // Deserialize account data
    let mut counter_data = Counter::try_from_slice(&counter_account.data.borrow())?;
    
    // Validate authority
    if counter_data.owner != *authority.key {
        return Err(ProgramError::InvalidAccountData);
    }
    
    // Check authority is a signer
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Update counter
    counter_data.count = counter_data.count.checked_add(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    
    // Serialize and store updated state
    counter_data.serialize(&mut &mut counter_account.data.borrow_mut()[..])?;
    
    msg!("Counter incremented by {}. New value: {}", amount, counter_data.count);

    Ok(())
}

// Decrement the counter by a specified amount
fn decrement_counter(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    
    // Get account references
    let counter_account = next_account_info(accounts_iter)?;
    let authority = next_account_info(accounts_iter)?;

    // Validate account ownership
    if counter_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    
    // Deserialize account data
    let mut counter_data = Counter::try_from_slice(&counter_account.data.borrow())?;
    
    // Validate authority
    if counter_data.owner != *authority.key {
        return Err(ProgramError::InvalidAccountData);
    }
    
    // Check authority is a signer
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Update counter with underflow check
    counter_data.count = counter_data.count.checked_sub(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    
    // Serialize and store updated state
    counter_data.serialize(&mut &mut counter_account.data.borrow_mut()[..])?;
    
    msg!("Counter decremented by {}. New value: {}", amount, counter_data.count);

    Ok(())
}

// Reset counter to zero
fn reset_counter(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    
    // Get account references
    let counter_account = next_account_info(accounts_iter)?;
    let authority = next_account_info(accounts_iter)?;

    // Validate account ownership
    if counter_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    
    // Deserialize account data
    let mut counter_data = Counter::try_from_slice(&counter_account.data.borrow())?;
    
    // Validate authority
    if counter_data.owner != *authority.key {
        return Err(ProgramError::InvalidAccountData);
    }
    
    // Check authority is a signer
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    
    // Reset counter
    counter_data.count = 0;
    
    // Serialize and store updated state
    counter_data.serialize(&mut &mut counter_account.data.borrow_mut()[..])?;
    
    msg!("Counter reset to 0");

    Ok(())
}
