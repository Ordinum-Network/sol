use crate::states::enums::TrialStatus;
use anchor_lang::prelude::*;

#[account]
pub struct Trial {
    pub trial_id: String,
    pub sponsor: Pubkey,
    pub title: String,
    pub owner_authority: String,
    pub current_phase: u8,
    pub total_phases: u8,
    pub status: TrialStatus,
    pub amendment_count: u8,
    pub start_date: i64,
    pub end_date: i64,
    pub created_date: i64,
    pub bump: u8,
}

// Discriminator → 8 bytes (always)
// String → 4 + max_len
// Pubkey → 32
// u8 → 1
// i64 → 8
// enum (TrialStatus) → usually 1 byte

impl Trial {
    pub const SIZE: usize = 8 
      + (4 + 50)  // trial_id
      + 32              // sponsor
      + (4 + 200)       // title
      + (4 + 200)       // ownerTitle
      + 1               // currentPhase
      + 1               // totalPhases
      + 1               // status
      + 1               // amendmentCount
      
      + 8               // startDate
      + 8               // endDate
      + 8               // createdDate
      
      + 1; // bump
}
