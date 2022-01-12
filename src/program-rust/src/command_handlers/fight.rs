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
  let signer = next_account_info(accounts_iter)?;
  let admin_public_key: [u8; 32] = [88,73,142,249,127,105,187,195,208,112,4,175,18,139,181,231,195,145,199,142,124,213,86,191,223,123,172,12,34,248,50,95];
  
  if signer.key.to_bytes() != admin_public_key || !signer.is_signer {
    return Err(ProgramError::IncorrectProgramId);
  }

  let mut player_1_account = player_account::Account::try_from_slice(&player1.data.borrow())?;
  let mut player_2_account = player_account::Account::try_from_slice(&player2.data.borrow())?;


  if !validations::players_can_fight(player1, player2) {
    return Err(ProgramError::IncorrectProgramId)
  }

  match bet::fight(&player_1_account, &player_2_account) {
    bet::BetResult::DRAW => Err(ProgramError::IncorrectProgramId),
    bet::BetResult::WinnerPlayer1 => {
       // set lp
       player_1_account.lp = player_2_account.bet;
       player_2_account.lp = player_1_account.bet;
 
      // clean bets
      player_1_account.bet = 0;
      player_2_account.bet = 0;
      // add point to player 1
      player_1_account.winners += 1;
      // add points to player 2
      player_2_account.losses += 1;
     
      player_1_account.serialize(&mut &mut player1.data.borrow_mut()[..])?;
      player_2_account.serialize(&mut &mut player2.data.borrow_mut()[..])?;


      **player1.try_borrow_mut_lamports()? += 1000000000;
      **player2.try_borrow_mut_lamports()? -= 1000000000;


      Ok(())
    },
    bet::BetResult::WinnerPlayer2 => {
      // set lp
      player_1_account.lp = player_2_account.bet;
      player_2_account.lp = player_1_account.bet;
       // clean bets
       player_1_account.bet = 0;
       player_2_account.bet = 0;
       // add point to player 1
       player_1_account.losses += 1;
       // add points to player 2
       player_2_account.winners += 1;
       

      player_1_account.serialize(&mut &mut player1.data.borrow_mut()[..])?;
      player_2_account.serialize(&mut &mut player2.data.borrow_mut()[..])?;

      **player1.try_borrow_mut_lamports()? -= 1000000000;
      **player2.try_borrow_mut_lamports()? += 1000000000;

      Ok(())
    },
    bet::BetResult::ERROR => Err(ProgramError::IncorrectProgramId)
  }
}

