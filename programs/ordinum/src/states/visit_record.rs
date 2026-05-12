use anchor_lang::prelude::*;

#[account] 
pub struct VisitRecord {
   pub patient: Pubkey,
   pub trial: Pubkey,
   pub coordinator: Pubkey,
   pub phase: u8,
   pub visit_number: u8,
   pub data_hash: [u8;32],
   pub timestamp: i64,
   pub bump: u8
}

impl VisitRecord {
    pub const SIZE:usize = 0;
}