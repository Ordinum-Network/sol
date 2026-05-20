use anchor_lang::prelude::*;

#[account] 
pub struct VisitRecord {
   pub patient: Pubkey,
   pub trial: Pubkey,
   pub coordinator: Pubkey,
   pub phase: u8,
   pub visit_number: u64,
   pub data_hash: [u8;32],
   pub timestamp: i64,
   pub bump: u8
}

impl VisitRecord {
    pub const SIZE:usize = 
      8 
      + 32    //patient
      + 32    //trial
      + 32    //coordinator
      + 1     //phase
      + 8     //visitnumber
      + 32    //datahash
      + 8     //timestamp
      + 1;    //bump
}