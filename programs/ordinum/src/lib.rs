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
    pub fn init_sponsor(ctx: Context<InitSponsor>, sponsor_title: String) -> Result<()> {
        instructions::initialise_sponsor(ctx, sponsor_title)
    }

    pub fn init_trial(ctx: Context<CreateTrial>, trial_id: String, sponsor_title: String, total_phases: u8, start_date: i64, end_date: i64) -> Result<()> {
        instructions::create_trial(ctx, trial_id, sponsor_title, total_phases, start_date, end_date)
    }
}