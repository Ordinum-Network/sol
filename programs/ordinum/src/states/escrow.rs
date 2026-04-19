use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
   sponsor: Pubkey,
   trial: Pubkey,
   usdc_mint: Pubkey,
   initial_deposit: u64,
   total_deposit: u64,
   balance: u64,
   bump: u8,
}

impl Escrow {
    pub const SIZE: usize = 
      8
      + 32      //sponsor
      + 32      //trial
      + 32      //usdc_mint
      + 8       //
      + 8       //
      + 8
      + 1;
}