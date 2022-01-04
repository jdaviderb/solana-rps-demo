mod command_handlers;
mod accounts;
mod config;
mod bet;

use borsh::{BorshDeserialize};
use solana_program::{
    account_info::{AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::accounts::command;
entrypoint!(process_instruction);


pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    
    // validation account's ownership
    for account in accounts_iter {
        if account.is_signer {
            continue;
        }

        if account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }
    }

    // Serialize Command Data
    let command_data = command::Account::try_from_slice(&instruction_data)?;

    // mapping command handlers
    match command_handlers::Commands::from_code(command_data.command) {
        command_handlers::Commands::CreateBet => command_handlers::create_bet::handler(
            program_id, 
            &accounts, 
            &instruction_data
        ),

        command_handlers::Commands::Fight => command_handlers::fight::handler(
            program_id, 
            &accounts, 
            &instruction_data
        ),

        _ => Err(ProgramError::IncorrectProgramId)
    }
}
