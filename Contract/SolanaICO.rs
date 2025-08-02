use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("PROGRAM_ID");

#[error_code]
pub enum ErrorCode {}

#[program]
pub mod ico{

  pub const ICO_MINT_ADDRESS: &str = "";
  pub const LAMPORTS_PER_TOKEN: u64 = ; // 0.001 sol in lamports
  pub const TOKEN_DECIMALS: u64 = ;  //10^9 for SPL token decimals
  use super::*;

  pub fn create_ico_ata(){}

  pub fn deposite_ico_in_ata(){}

  pub fn buy_tokens(){}

  #[derive(Accounts)]
  pub struct CreateIcoATA<'info> {}

  #[derive(Accounts)]
  pub struct DepositeIcoATA<'info> {}

  #[derive(Accounts)]
  #[instruction(_ico_ata_for_ico_program_bump: u8)]
  pub struct BuyTokens<'info> {}

  #[account]
  pub struct Data{}

}
