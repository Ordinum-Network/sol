use anchor_lang::prelude::*;
use crate::states::enums::TrialStatus;

#[account]
pub struct Trial {
   pub trial_id: String,
   pub sponsor: Pubkey,
   pub site: String,
   pub title: String,
   pub currentPhase: u8,
   pub totalPhases: u8,
   pub status: TrialStatus,
   pub amendmentCount: u8,
   pub startDate: i64,
   pub endDate: i64,
   pub createdDate: i64,
   pub bump: u8
}

// Discriminator → 8 bytes (always)
// String → 4 + max_len
// Pubkey → 32
// u8 → 1
// i64 → 8
// enum (TrialStatus) → usually 1 byte

impl Trial {
    pub const SIZE: usize = 
      8 
      + (4 + 50)  // trial_id
      + 32              // sponsor
      + (4 + 100)       //site
      + (4 + 200)       //title
      
      + 1               // currentPhase
      + 1               //totalPhases
      + 1               //status
      + 1               //amendmentCount
      
      + 8               //startDate
      + 8               //endDate
      + 8               //createdDate
      
      + 1;              //bump
}
