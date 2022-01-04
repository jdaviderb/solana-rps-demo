pub mod create_bet;
pub mod fight;

pub enum Commands {
  CreateBet,
  Fight,
  Unknown
}

impl Commands {
  pub fn from_code(code: u32) -> Self {
    match code {
      1 => Commands::CreateBet,
      2 => Commands::Fight,
      _ => Commands::Unknown
    }
  }
}
