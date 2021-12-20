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
    
    for account in accounts_iter {
        if account.is_signer {
            continue;
        }

        if account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }
    }

    let command_data = command::Account::try_from_slice(&instruction_data)?;
    return match command_data.data {
        config::handlers::CREATE_BET_COMMAND => command_handlers::create_bet::handler(
            program_id, 
            &accounts, 
            &instruction_data
        ),

        config::handlers::FIGHT_COMMAND => command_handlers::fight::handler(
            program_id, 
            &accounts, 
            &instruction_data
        ),
        _ => Err(ProgramError::IncorrectProgramId)
    }
}
