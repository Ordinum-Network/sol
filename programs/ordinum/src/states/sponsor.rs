use anchor_lang::prelude::*;

#[account]
// #[derive(InitSpace)]
pub struct Sponsor {
   pub authority: Pubkey,
   pub sponsor_title: String, 
   pub trial_count: u64,
   pub verified: bool,
   pub created_at: i64,
   pub bump: u8
}
