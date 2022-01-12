use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Account {
  pub winners: u32,
  pub losses: u32,
  pub bet: u32,
  pub lp: u32
}
