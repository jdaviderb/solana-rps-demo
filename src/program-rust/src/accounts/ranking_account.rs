use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Account {
  pub wins: u32,
  pub wallet: [u8; 32]
}
