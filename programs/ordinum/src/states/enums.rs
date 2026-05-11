use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum TrialStatus {
    Draft, 
    Active,
    Paused, 
    Completed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum CoordinatorRole {
    PI, CRC, CRA
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum CoordinatorStatus {
    Active, Inactive
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum PatientStatus {
    Active, Completed, Withdrawn
}