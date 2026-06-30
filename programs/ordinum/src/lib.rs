use anchor_lang::prelude::*;

pub mod common;
pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;

use crate::states::enums::CoordinatorRole;
use common::*;
use instructions::*;
use crate::states::TrialStatus;

declare_id!("HEsXA7uhHXTTJkKNBR8Ydj4h4hmRxcX4eksVb4uMJceT");

#[program]
pub mod ordinum {
    use crate::instruction::UpdateSponsorVerified;
    use crate::instructions::coordinator::InitCoordinator;
    use crate::instructions::escrow::InitEscrow;
    use crate::instructions::sponsor::InitSponsor;
    use crate::instructions::trial::CreateTrial;
    use crate::states::{AccountType};

    use super::*;

    pub fn init_sponsor(ctx: Context<InitSponsor>, sponsor_title: String) -> Result<()> {
        instructions::initialise_sponsor(ctx, sponsor_title)
    }

    pub fn init_trial(
        ctx: Context<CreateTrial>,
        trial_id: String,
        sponsor_title: String,
        total_phases: u8,
        start_date: i64,
        end_date: i64,
    ) -> Result<()> {
        instructions::create_trial(
            ctx,
            trial_id,
            sponsor_title,
            total_phases,
            start_date,
            end_date,
        )
    }

    pub fn init_escrow(
        ctx: Context<InitEscrow>,
        trial_id: String,
        sponsor_title: String,
        initial_deposit: u64,
        sol_deposit: u64,
    ) -> Result<()> {
        instructions::init_escrow(ctx, trial_id, sponsor_title, initial_deposit, sol_deposit)
    }

    pub fn init_coordinator(
        ctx: Context<InitCoordinator>,
        trial_id: String,
        sponsor_title: String,
        coordinator_pubkey: Pubkey,
        role: CoordinatorRole,
    ) -> Result<()> {
        instructions::create_coordinator(ctx, trial_id, sponsor_title, coordinator_pubkey, role)
    }

    pub fn init_coordinator_with_pi(
        ctx: Context<InitCoordinatorWithPI>,
        trial_id: String,
        sponsor_title: String,
        coordinator_pubkey: Pubkey,
        title: String,
        role: CoordinatorRole,
    ) -> Result<()> {
        instructions::create_coordinator_with_pi(
            ctx,
            trial_id,
            sponsor_title,
            coordinator_pubkey,
            title,
            role,
        )
    }

    pub fn prefund_signer_as_sponsor(
        ctx: Context<PrefundSignerAsSponsor>,
        trial_id: String,
        sponsor_title: String,
    ) -> Result<()> {
        common::prefund_signer(
            ctx.accounts.escrow_account.to_account_info(),
            &mut ctx.accounts.escrow_account,
            ctx.accounts.signer.to_account_info(),
            AccountType::Coordinator,
        )
    }

    pub fn prefund_signer_as_pi(
        ctx: Context<PrefundSignerAsPI>,
        trial_id: String,
        sponsor_title: String,
    ) -> Result<()> {
        common::prefund_signer(
            ctx.accounts.escrow_account.to_account_info(),
            &mut ctx.accounts.escrow_account,
            ctx.accounts.signer.to_account_info(),
            AccountType::Coordinator,
        )
    }
    pub fn prefund_signer_as_crc(
        ctx: Context<PrefundSignerAsCRC>,
        trial_id: String,
        sponsor_title: String,
    ) -> Result<()> {
        common::prefund_signer(
            ctx.accounts.escrow_account.to_account_info(),
            &mut ctx.accounts.escrow_account,
            ctx.accounts.signer.to_account_info(),
            AccountType::Patient,
        )
    }

    pub fn prefund_signer_as_crc_for_visit(
        ctx: Context<PrefundSignerAsCRC>,
        trial_id: String,
        sponsor_title: String,
    ) -> Result<()> {
        common::prefund_signer(
            ctx.accounts.escrow_account.to_account_info(),
            &mut ctx.accounts.escrow_account,
            ctx.accounts.signer.to_account_info(),
            AccountType::VisitRecord,
        )
    }

    pub fn prefund_signer_as_crc_for_phase(
        ctx: Context<PrefundSignerAsCRC>,
        trial_id: String,
        sponsor_title: String,
    ) -> Result<()> {
        common::prefund_signer(
            ctx.accounts.escrow_account.to_account_info(),
            &mut ctx.accounts.escrow_account,
            ctx.accounts.signer.to_account_info(),
            AccountType::Phase,
        )
    }

    pub fn prefund_signer_as_crc_for_payment(
        ctx: Context<PrefundSignerAsCRC>,
        trial_id: String,
        sponsor_title: String,
    ) -> Result<()> {
        common::prefund_signer(
            ctx.accounts.escrow_account.to_account_info(),
            &mut ctx.accounts.escrow_account,
            ctx.accounts.signer.to_account_info(),
            AccountType::Payment,
        )
    }

    pub fn prefund_signer_as_crc_for_ATA(
        ctx: Context<PrefundSignerAsCRC>,
        trial_id: String,
        sponsor_title: String,
    ) -> Result<()> {
        common::prefund_signer(
            ctx.accounts.escrow_account.to_account_info(),
            &mut ctx.accounts.escrow_account,
            ctx.accounts.signer.to_account_info(),
            AccountType::ATA,
        )
    }
    pub fn prefund_signer_for_update(
        ctx: Context<PrefundSignerAsSponsor>,
        trial_id: String,
        sponsor_title: String,
    ) -> Result<()> {
        common::prefund_signer(
            ctx.accounts.escrow_account.to_account_info(),
            &mut ctx.accounts.escrow_account,
            ctx.accounts.signer.to_account_info(),
            AccountType::Update,
        )
    }

    pub fn init_patient(
        ctx: Context<CreatePatient>,
        trial_id: String,
        sponsor_title: String,
        consent_hash: [u8; 32],
    ) -> Result<()> {
        instructions::init_patient(ctx, trial_id, sponsor_title, consent_hash)
    }

    pub fn visit_record(
        ctx: Context<CreateVisitRecord>,
        trial_id: String,
        sponsor_title: String,
        phase: u8,
        data_hash: [u8; 32],
    ) -> Result<()> {
        instructions::create_visit_record(ctx, trial_id, sponsor_title, phase, data_hash)
    }

    pub fn init_phase(
        ctx: Context<CreatePhase>,
        trial_id: String,
        sponsor_title: String,
        data_hash: [u8; 32],
        total_visits: u16,
    ) -> Result<()> {
        instructions::create_phase(ctx, trial_id, sponsor_title, data_hash, total_visits)
    }

    pub fn init_paymentacc(
        ctx: Context<CreatePayment>,
        trial_id: String,
        sponsor_title: String,
        phase: u8,
        amount: u64,
    ) -> Result<()> {
        instructions::create_payment(ctx, trial_id, sponsor_title, phase, amount)
    }

    pub fn update_sponsor_verified(
        ctx: Context<UpdateSponsor>,
        sponsor_title: String,
        verified: bool,
    ) -> Result<()> {
        instructions::update_sponsor_verified(ctx, sponsor_title, verified)
    }

    pub fn update_total_phases_in_trial(
        ctx: Context<UpdateTrial>,
        sponsor_title: String, 
        trial_id: String,
        total_phases: u8,
    ) -> Result<()> {
        instructions::udpate_total_phases(ctx, trial_id, sponsor_title, total_phases)
    }

    pub fn update_status_in_trial(
        ctx: Context<UpdateTrial>,
        sponsor_title: String,
        trial_id: String,
        status: TrialStatus
    ) -> Result<()> {
        instructions::update_status(ctx, trial_id, sponsor_title, status)
    }

    pub fn update_completed_by_in_phase(
        mut ctx: Context<UpdatePhase>,
        trial_id: String,
        sponsor_title: String,
        phase: u8,
        completed_at: i64
    ) -> Result<()> {
        instructions::update_completed_at(ctx, trial_id, sponsor_title, phase, completed_at)
    }

    pub fn top_up_sol(ctx: Context<UpdateEscrow>, trial_id: String, sponsor_title: String, sol: u64) -> Result<()> {
        instructions::top_up_sol(ctx, trial_id, sponsor_title, sol)
    }

    pub fn top_up_tokens(ctx: Context<UpdateEscrow>, trial_id: String, sponsor_title: String, usdc: u64) -> Result<()> {
        instructions::top_up_tokens(ctx, trial_id, sponsor_title, usdc)
    }

    pub fn update_coordinator_inactive(ctx: Context<UpdateCoordinator>, trial_id: String, sponsor_title: String, coordinator_pubkey: Pubkey) -> Result<()> {
        instructions::update_coordinator_status_inactive(ctx, trial_id, sponsor_title, coordinator_pubkey)
    }
}
