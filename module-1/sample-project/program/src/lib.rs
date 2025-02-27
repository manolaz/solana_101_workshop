use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Define the program's entrypoint
entrypoint!(process_instruction);

// Define the message structure
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MessageAccount {
    pub message: String,
}

// Program instructions
pub enum Instruction {
    Initialize(String),
    UpdateMessage(String),
}

// Instruction processing
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Parse instruction data
    let instruction = if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    } else {
        match instruction_data[0] {
            0 => {
                // Initialize with message
                let message = String::from_utf8(instruction_data[1..].to_vec())
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                Instruction::Initialize(message)
            }
            1 => {
                // Update message
                let message = String::from_utf8(instruction_data[1..].to_vec())
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                Instruction::UpdateMessage(message)
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        }
    };

    // Get account iterator
    let account_info_iter = &mut accounts.iter();
    
    // Get accounts
    let message_account = next_account_info(account_info_iter)?;
    let user_account = next_account_info(account_info_iter)?;

    // Check if user account is the signer
    if !user_account.is_signer {
        msg!("User account must be the signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Check if the message account is owned by the program
    if message_account.owner != program_id {
        if instruction_data[0] == 0 {
            // For initialization, we need to take ownership
            msg!("Initializing message account");
        } else {
            msg!("Message account does not belong to this program");
            return Err(ProgramError::IncorrectProgramId);
        }
    }

    match instruction {
        Instruction::Initialize(message) => {
            msg!("Instruction: Initialize message");
            
            // Create the account
            let mut message_data = MessageAccount {
                message,
            };

            // Serialize and store the message data
            message_data.serialize(&mut &mut message_account.data.borrow_mut()[..])?;
            msg!("Message account initialized successfully");
        }
        Instruction::UpdateMessage(new_message) => {
            msg!("Instruction: Update message");
            
            // Deserialize the account data
            let mut message_data = MessageAccount::try_from_slice(&message_account.data.borrow())?;
            
            // Update the message
            message_data.message = new_message;
            
            // Serialize and store the updated message data
            message_data.serialize(&mut &mut message_account.data.borrow_mut()[..])?;
            msg!("Message updated successfully");
        }
    }

    Ok(())
}
