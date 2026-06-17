use anchor_lang::prelude::*;

#[account]
pub struct Payment {
    pub trial_id: Pubkey,
    pub sponsor: Pubkey,
    pub coordinator: Pubkey,
    pub phase_account: Pubkey,
    pub phase: u8,
    pub visit_record: Pubkey,
    pub reciever_wallet: Pubkey,
    pub timestamp: i64,
    pub amount: u64,
    pub bump: u8,
}

impl Payment {
    pub const SIZE: usize = 
     8
     + 32     // trial_id
     + 32     // sponsor
     + 32     // coordinator
     + 32     // phase_account
     + 1      // phase
     + 32     // visit_record 
     + 32     // reciever_wallet
     + 8      // timestamp
     + 8      // amount
     + 1      // bump
     ;
}
