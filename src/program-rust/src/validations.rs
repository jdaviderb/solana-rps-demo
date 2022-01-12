use solana_program::{
  account_info::AccountInfo,
  pubkey::Pubkey
};

use crate::config;
use crate::accounts::{
  command
};

pub fn players_can_fight(player1: &AccountInfo, player2: &AccountInfo) -> bool {
  if player1.lamports() < config::player::MIN_LAMPORTS || player2.lamports() < config::player::MIN_LAMPORTS {
    return false;
  }

  true
}

pub fn player_can_bet(
  account: &AccountInfo, 
  command_data: &command::Account
) -> bool {

  if account.lamports() < config::player::MIN_LAMPORTS {
    return false;
  }

  if !config::player::VALID_BETS.contains(&command_data.data) {
    return false;
  }

  true
}

pub fn program_can_run(program_id: &Pubkey, accounts: &[AccountInfo]) -> bool {
  let accounts_iter = &mut accounts.iter();
    
  for account in accounts_iter {
      if account.is_signer {
        continue;
      }

      if account.owner != program_id {
        return false;
      }
  }

  true
}
