use anchor_lang::prelude::*;

use crate::states::PatientStatus;

#[account]
pub struct Patient {
  pub trial_id: Pubkey,
  pub sponsor: Pubkey,
  pub wallet: Pubkey,
  pub consent_hash: [u8;32],
  pub status: PatientStatus,
  pub enrolled_at: i64,
  pub bump: u8,
  pub number_of_visits: u64,
  pub last_modified: i64
}

impl Patient {
    pub const SIZE: usize = 
     8
     + 32    // trial_id
     + 32    // sponsor
     + 32    // wallet
     + 32    // consent_hash
     + 1     // status
     + 8     // enrolled_at
     + 1     // bump
     + 8     // numberofvisits
     + 8     // last_modified
     ;
}