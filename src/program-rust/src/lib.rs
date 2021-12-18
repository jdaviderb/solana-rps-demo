mod command_handlers;
mod accounts;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
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
    let account = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let command_data = command::Account::try_from_slice(&instruction_data)?;
    return match command_data.data {
        1 => command_handlers::create_bet::handler(
            program_id, 
            &accounts, 
            &instruction_data
        ),
        _ => Err(ProgramError::IncorrectProgramId)
    }
}
