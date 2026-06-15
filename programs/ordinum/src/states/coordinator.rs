use anchor_lang::prelude::*;

use crate::states::enums::{CoordinatorRole, CoordinatorStatus};

#[account] 
pub struct Coordinator {
   pub trial_id: Pubkey,
   pub sponsor: Pubkey,
   pub role: CoordinatorRole,
   pub status: CoordinatorStatus,
   pub assigned_at: i64,
   pub bump: u8,
}

impl Coordinator {
   pub const SIZE: usize =
     8
     + 32  // trialId
     + 32  // sponsor
     + 1   // role
     + 1   // status
     + 8   // assignedAt
     + 1;  // bump
}