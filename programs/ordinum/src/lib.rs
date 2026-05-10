use anchor_lang::prelude::*;

pub mod instructions;
pub mod states;
pub mod errors;
pub mod constants;

use instructions::*;
use crate::states::enums::{CoordinatorRole};

declare_id!("HEsXA7uhHXTTJkKNBR8Ydj4h4hmRxcX4eksVb4uMJceT");

#[program]
pub mod ordinum {
    use crate::instructions::coordinator::InitCoordinator;
    use crate::instructions::sponsor::InitSponsor;
    use crate::instructions::escrow::InitEscrow;
    use crate::instructions::trial::CreateTrial;

    use super::*;

    pub fn init_sponsor(ctx: Context<InitSponsor>, sponsor_title: String) -> Result<()> {
        instructions::initialise_sponsor(ctx, sponsor_title)
    }

    pub fn init_trial(ctx: Context<CreateTrial>, trial_id: String, sponsor_title: String, total_phases: u8, start_date: i64, end_date: i64) -> Result<()> {
        instructions::create_trial(ctx, trial_id, sponsor_title, total_phases, start_date, end_date)
    }

    pub fn init_escrow(ctx: Context<InitEscrow>, trial_id: String, sponsor_title: String, initial_deposit: u64, sol_deposit: u64) -> Result<()> {
        instructions::init_escrow(ctx, trial_id, sponsor_title, initial_deposit, sol_deposit)
    }

    pub fn init_coordinator(ctx: Context<InitCoordinator>, trial_id: String, sponsor_title: String, coordinator_pubkey: Pubkey, role: CoordinatorRole) -> Result<()> {
        instructions::create_coordinator(ctx, trial_id, sponsor_title, coordinator_pubkey, role)
    }
    
    pub fn init_coordinator_with_pi(ctx: Context<InitCoordinatorWithPI>, trial_id: String, sponsor_title: String, coordinator_pubkey: Pubkey, role: CoordinatorRole) -> Result<()> {
        instructions::create_coordinator_with_pi(ctx, trial_id, sponsor_title, coordinator_pubkey, role)
    }

    pub fn prefund_signer_as_sponsor(ctx: Context<PrefundSignerAsSponsor>, trial_id: String, sponsor_title: String) -> Result<()> {
      instructions::prefund_signer(ctx.accounts.escrow_account.to_account_info(), &mut ctx.accounts.escrow_account, ctx.accounts.signer.to_account_info())    }
    
    pub fn prefund_signer_as_pi(ctx: Context<PrefundSignerAsPI>, trial_id: String, sponsor_title: String) -> Result<()> {
      instructions::prefund_signer(ctx.accounts.escrow_account.to_account_info(), &mut ctx.accounts.escrow_account, ctx.accounts.signer.to_account_info())
    }
}