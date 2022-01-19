use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  pubkey::Pubkey,
  program_error::ProgramError,
};

use borsh::{BorshDeserialize, BorshSerialize};
use crate::accounts::{player_account, ranking_account};
use crate::bet;
use crate::validations;
use crate::config;

pub fn handler(
  _program_id: &Pubkey,
  accounts: &[AccountInfo],
  _instruction_data: &[u8],
) -> ProgramResult {
  let accounts_iter = &mut accounts.iter();

  let player1 = next_account_info(accounts_iter)?;
  let player2 = next_account_info(accounts_iter)?;
  let signer = next_account_info(accounts_iter)?;
  let ranking = next_account_info(accounts_iter)?;
  
  if signer.key.to_bytes() != config::ADMIN_PUBLIC_KEY || !signer.is_signer {
    return Err(ProgramError::IncorrectProgramId);
  }

  let mut player_1_account = player_account::Account::try_from_slice(&player1.data.borrow())?;
  let mut player_2_account = player_account::Account::try_from_slice(&player2.data.borrow())?;
  let mut ranking_account = match <Vec<ranking_account::Account>>::try_from_slice(&ranking.data.borrow()) {
    Ok(data) => data,
    Err(_) => init_ranking()
  };

  if !validations::players_can_fight(player1, player2) {
    return Err(ProgramError::IncorrectProgramId)
  }

  player_1_account.lp = player_2_account.bet;
  player_2_account.lp = player_1_account.bet;

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

      **player1.try_borrow_mut_lamports()? += config::player::MIN_LAMPORTS;
      **player2.try_borrow_mut_lamports()? -= config::player::MIN_LAMPORTS;

      update_ranking(&mut ranking_account, player_1_account.winners, player1.key.to_bytes());
      let data = &mut &mut ranking.data.borrow_mut();
      let updated_data = ranking_account.try_to_vec().unwrap();
      data[..updated_data.len()].copy_from_slice(&updated_data);  


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

      **player1.try_borrow_mut_lamports()? -= config::player::MIN_LAMPORTS;
      **player2.try_borrow_mut_lamports()? += config::player::MIN_LAMPORTS;

      update_ranking(&mut ranking_account, player_2_account.winners, player2.key.to_bytes());

      let data = &mut &mut ranking.data.borrow_mut();
      let updated_data = ranking_account.try_to_vec().unwrap();
      data[..updated_data.len()].copy_from_slice(&updated_data);  

      Ok(())
    },
    bet::BetResult::ERROR => Err(ProgramError::IncorrectProgramId)
  }
}

fn init_ranking() -> Vec<ranking_account::Account> {
  let blank_wallet = [
    00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00,
    00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00
  ];
  vec![
    ranking_account::Account { wallet: blank_wallet, wins: 0 },
    ranking_account::Account { wallet: blank_wallet, wins: 0 },
    ranking_account::Account { wallet: blank_wallet, wins: 0 },
  ]
}

fn update_ranking(
  ranking: &mut Vec<ranking_account::Account>, 
  wins: u32,
  wallet: [u8; 32]
) {
  for ranking_player in ranking.iter_mut() {
    if wins > ranking_player.wins {

      let prev_win = ranking_player.wins;
      let pre_wallet = ranking_player.wallet;

      ranking_player.wins = wins;
      ranking_player.wallet = wallet;

      if pre_wallet != wallet {
        update_ranking(ranking, prev_win, pre_wallet);
      }

      break;
    }
  }
}
