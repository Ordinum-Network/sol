use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum TrialStatus {
    Draft, 
    Active,
    Paused, 
    Completed,
}