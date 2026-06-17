use anchor_lang::prelude::*;

#[account]
pub struct Phase {
    pub trial_id: Pubkey,
    pub sponsor: Pubkey,
    pub phase_number: u8,
    pub data_hash: [u8; 32],
    pub completed_by: Pubkey,
    pub total_visits: u16,
    pub completed_at: i64,
    pub bump: u8,
}

impl Phase {
    pub const SIZE: usize = 8           // disc
    + 32        // trial_id
    + 32        // sponsor
    + 1         // phase
    + 32        // datahash
    + 32        // completedby
    + 2         // totalvisits
    + 8         // completedat
    + 1; // bump
}
