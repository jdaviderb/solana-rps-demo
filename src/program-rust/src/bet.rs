use crate::accounts::player_account::Account;

pub enum Bet {
  PAPER,
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

pub fn fight(player1: &Account, player2: &Account) -> BetResult {

  match Bet::from_u32(player1.bet) {

    Bet::PAPER => match Bet::from_u32(player2.bet) {
      Bet::PAPER => BetResult::DRAW,
      Bet::ROCK => BetResult::WinnerPlayer1,
      Bet::SCISSORS => BetResult::WinnerPlayer2,
      Bet::UNKNOWN => BetResult::ERROR
    },

    Bet::ROCK => match Bet::from_u32(player2.bet) {
      Bet::ROCK => BetResult::DRAW,
      Bet::PAPER => BetResult::WinnerPlayer2,
      Bet::SCISSORS => BetResult::WinnerPlayer1,
      Bet::UNKNOWN => BetResult::ERROR
    },

    Bet::SCISSORS => match Bet::from_u32(player2.bet) {
      Bet::SCISSORS => BetResult::DRAW,
      Bet::PAPER => BetResult::WinnerPlayer1,
      Bet::ROCK => BetResult::WinnerPlayer2,
      Bet::UNKNOWN => BetResult::ERROR
    },

    Bet::UNKNOWN => match Bet::from_u32(player2.bet) {
      Bet::PAPER => BetResult::ERROR,
      Bet::ROCK => BetResult::ERROR,
      Bet::SCISSORS => BetResult::ERROR,
      Bet::UNKNOWN => BetResult::ERROR
    }
  }
}
