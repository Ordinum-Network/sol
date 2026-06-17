use anchor_lang::prelude::*;

#[account]
pub struct Sponsor {
    pub authority: Pubkey,
    pub sponsor_title: String,
    pub verified: bool,
    pub created_at: i64,
    pub bump: u8,
}

impl Sponsor {
    pub const SIZE: usize = 8 + 32 + (4 + 64) + 1 + 8 + 1;
}
