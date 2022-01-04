use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  pubkey::Pubkey,
  program_error::ProgramError
};

use borsh::{BorshDeserialize, BorshSerialize};
use crate::accounts::player_account;
use crate::bet;
use crate::validations;

pub fn handler(
  _program_id: &Pubkey,
  accounts: &[AccountInfo],
  _instruction_data: &[u8],
) -> ProgramResult {
  let accounts_iter = &mut accounts.iter();

  let player1 = next_account_info(accounts_iter)?;
  let player2 = next_account_info(accounts_iter)?;

  let mut player_1_account = player_account::Account::try_from_slice(&player1.data.borrow())?;
  let mut player_2_account = player_account::Account::try_from_slice(&player2.data.borrow())?;


  if !validations::players_can_fight(player1, player2) {
    return Err(ProgramError::IncorrectProgramId)
  }

  match bet::fight(&player_1_account, &player_2_account) {
    bet::BetResult::DRAW => Err(ProgramError::IncorrectProgramId),
    bet::BetResult::WinnerPlayer1 => {
      // clean bets
      player_1_account.bet = 0;
      player_2_account.bet = 0;
      // add point to player 1
      player_1_account.winners += 1;
      // add points to player 2
      player_2_account.losses += 1;

      player_1_account.serialize(&mut &mut player1.data.borrow_mut()[..])?;
      player_2_account.serialize(&mut &mut player2.data.borrow_mut()[..])?;


      Ok(())
    },
    bet::BetResult::WinnerPlayer2 => {
       // clean bets
       player_1_account.bet = 0;
       player_2_account.bet = 0;
       // add point to player 1
       player_1_account.losses += 1;
       // add points to player 2
       player_2_account.winners += 1;

      player_1_account.serialize(&mut &mut player1.data.borrow_mut()[..])?;
      player_2_account.serialize(&mut &mut player2.data.borrow_mut()[..])?;

      Ok(())
    },
    bet::BetResult::ERROR => Err(ProgramError::IncorrectProgramId)
  }
}

