use anchor_lang::prelude::*;

pub mod instructions;
pub mod states;
pub mod errors;
pub mod constants;
pub mod common;

use instructions::*;
use common::*;
use crate::states::enums::{CoordinatorRole};

declare_id!("HEsXA7uhHXTTJkKNBR8Ydj4h4hmRxcX4eksVb4uMJceT");

#[program]
pub mod ordinum {
    use crate::instructions::coordinator::InitCoordinator;
    use crate::instructions::sponsor::InitSponsor;
    use crate::instructions::escrow::InitEscrow;
    use crate::instructions::trial::CreateTrial;
use crate::states::AccountType;

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
      common::prefund_signer(ctx.accounts.escrow_account.to_account_info(), &mut ctx.accounts.escrow_account, ctx.accounts.signer.to_account_info(), AccountType::Coordinator)    
    }
    
    pub fn prefund_signer_as_pi(ctx: Context<PrefundSignerAsPI>, trial_id: String, sponsor_title: String) -> Result<()> {
      common::prefund_signer(ctx.accounts.escrow_account.to_account_info(), &mut ctx.accounts.escrow_account, ctx.accounts.signer.to_account_info(), AccountType::Coordinator)
    }
    pub fn prefund_signer_as_crc(ctx: Context<PrefundSignerAsCRC>, trial_id: String, sponsor_title: String) -> Result<()> {
      common::prefund_signer(ctx.accounts.escrow_account.to_account_info(), &mut ctx.accounts.escrow_account, ctx.accounts.signer.to_account_info(), AccountType::Patient)
    }
   
    pub fn prefund_signer_as_crc_for_visit(ctx: Context<PrefundSignerAsCRC>, trial_id: String, sponsor_title: String) -> Result<()> {
      common::prefund_signer(ctx.accounts.escrow_account.to_account_info(), &mut ctx.accounts.escrow_account, ctx.accounts.signer.to_account_info(), AccountType::VisitRecord)
    }
    
    pub fn prefund_signer_as_crc_for_phase(ctx: Context<PrefundSignerAsCRC>, trial_id: String, sponsor_title: String) -> Result<()> {
      common::prefund_signer(ctx.accounts.escrow_account.to_account_info(), &mut ctx.accounts.escrow_account, ctx.accounts.signer.to_account_info(), AccountType::Phase)
    }

    pub fn init_patient(ctx: Context<CreatePatient>, trial_id: String, sponsor_title: String, consent_hash: [u8;32]) -> Result<()> {
      instructions::init_patient(ctx, trial_id, sponsor_title, consent_hash)
    }

    pub fn visit_record(ctx: Context<CreateVisitRecord>, trial_id: String, sponsor_title: String, phase: u8, data_hash:[u8;32]) -> Result<()> {
      instructions::create_visit_record(ctx, trial_id, sponsor_title, phase, data_hash)
    }

    pub fn init_phase(ctx: Context<CreatePhase>, trial_id: String, sponsor_title: String, data_hash:[u8;32], total_visits: u16) -> Result<()> {
      instructions::create_phase(ctx, trial_id, sponsor_title, data_hash, total_visits)
    }

}