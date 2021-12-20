use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  pubkey::Pubkey,
  program_error::ProgramError
};

use borsh::{BorshDeserialize, BorshSerialize};
use crate::accounts::{player_account, command};
use crate::config;

pub fn handler(
  _program_id: &Pubkey,
  accounts: &[AccountInfo],
  instruction_data: &[u8],
) -> ProgramResult {
  let accounts_iter = &mut accounts.iter();
  let account = next_account_info(accounts_iter)?;

  let mut player_account = player_account::Account::try_from_slice(&account.data.borrow())?;
  let command_data = command::Account::try_from_slice(&instruction_data)?;

  if account.lamports() < config::player::MIN_LAMPORTS {
    return Err(ProgramError::IncorrectProgramId);
  }

  if !config::player::VALID_BETS.contains(&command_data.data) {
    return Err(ProgramError::IncorrectProgramId);
  }

  if player_account.bet > 0 {
    return Err(ProgramError::IncorrectProgramId);
  }

  player_account.bet = command_data.data;
  player_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

  Ok(())
}
