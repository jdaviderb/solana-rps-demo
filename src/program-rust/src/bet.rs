use solana_program::{
  account_info::AccountInfo,
};
use crate::accounts::player_account;
use borsh::{BorshDeserialize};

pub enum Bet {
  PAPER = 0,
  ROCK,
  SCISSORS,
  UNKNOWN
}

pub enum BetResult {
  ERROR,
  DRAW,
  WinnerPlayer1,
  WinnerPlayer2
}

impl Bet {
  fn from_u32(value: u32) -> Bet {
    match value {
      1 => Bet::PAPER,
      2 => Bet::ROCK,
      3 => Bet::SCISSORS,
      _ => Bet::UNKNOWN
    }
  }
}

pub fn fight(player1: &AccountInfo, player2: &AccountInfo) -> BetResult {
  let player_1_data = player_account::Account::try_from_slice(&player1.data.borrow()).unwrap();
  let player_2_data = player_account::Account::try_from_slice(&player2.data.borrow()).unwrap();

  match Bet::from_u32(player_1_data.bet) {

    Bet::PAPER => match Bet::from_u32(player_2_data.bet) {
      Bet::PAPER => BetResult::DRAW,
      Bet::ROCK => BetResult::WinnerPlayer1,
      Bet::SCISSORS => BetResult::WinnerPlayer2,
      Bet::UNKNOWN => BetResult::ERROR
    },

    Bet::ROCK => match Bet::from_u32(player_2_data.bet) {
      Bet::ROCK => BetResult::DRAW,
      Bet::PAPER => BetResult::WinnerPlayer2,
      Bet::SCISSORS => BetResult::WinnerPlayer1,
      Bet::UNKNOWN => BetResult::ERROR
    },

    Bet::SCISSORS => match Bet::from_u32(player_2_data.bet) {
      Bet::SCISSORS => BetResult::DRAW,
      Bet::PAPER => BetResult::WinnerPlayer1,
      Bet::ROCK => BetResult::WinnerPlayer2,
      Bet::UNKNOWN => BetResult::ERROR
    },

    Bet::UNKNOWN => match Bet::from_u32(player_2_data.bet) {
      Bet::PAPER => BetResult::DRAW,
      Bet::ROCK => BetResult::ERROR,
      Bet::SCISSORS => BetResult::ERROR,
      Bet::UNKNOWN => BetResult::ERROR
    }
  }
}
