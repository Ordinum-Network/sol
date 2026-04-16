use anchor_lang::prelude::*;

pub mod instructions;
pub mod states;
pub mod errors;
pub mod constants;

use instructions::*;

declare_id!("HEsXA7uhHXTTJkKNBR8Ydj4h4hmRxcX4eksVb4uMJceT");

#[program]
pub mod ordinum {
    use crate::instructions::initialize_sponsor::InitSponsor;
    use super::*;
    pub fn init_sponsor(ctx: Context<InitSponsor>, name: String) -> Result<()> {
        instructions::initialise_sponsor(ctx, name)
    }

    pub fn init_trial(ctx: Context<CreateTrial>, trialName: String, currentPhase:u8, totalPhases: u8, startDate: i64, endDate: i64, name: String) -> Result<()> {
        instructions::create_trial(ctx, trialName, currentPhase, totalPhases, startDate, endDate, name)
    }
}