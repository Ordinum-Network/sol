use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub sponsor: Pubkey,
    pub trial: Pubkey,
    pub usdc_mint: Pubkey,
    pub initial_deposit: u64,
    pub total_deposit: u64,
    pub balance: u64,
    pub sol_balance: u64,
    pub bump: u8,
}

impl Escrow {
    pub const SIZE: usize = 8
      + 32      //sponsor
      + 32      //trial
      + 32      //usdc_mint
      + 8       //initial_deposit
      + 8       //total_deposit
      + 8       //balance
      + 8       //sol_balance
      + 1;
}
