use anchor_lang::prelude::*;

use crate::{
    constants::{COORDINATOR_SEED, PATIENT_SEED, PHASE, SPONSOR_SEED, TRIAL_SEED, VISIT_RECORD},
    errors::OrdinumError,
    instructions::{coordinator, phase, trial, visit_record},
    states::{Coordinator, CoordinatorRole, Phase, Sponsor, Trial, VisitRecord, patient::Patient},
};

pub fn create_visit_record(
    ctx: Context<CreateVisitRecord>,
    trial_id: String,
    sponsor_title: String,
    phase: u8,
    data_hash: [u8; 32],
) -> Result<()> {
    let coordinator: &Account<'_, Coordinator> = &ctx.accounts.coordinator_account;
    require!(
        coordinator.role == CoordinatorRole::CRC,
        OrdinumError::Unauthorized
    );
    let visit_record = &mut ctx.accounts.visit_record_account;
    let patient = &mut ctx.accounts.patient_account;
    let phase_account = &mut ctx.accounts.phase_account;

    visit_record.patient = patient.key();
    visit_record.trial = ctx.accounts.trial_account.key();
    visit_record.coordinator = coordinator.key();
    visit_record.phase = phase;
    visit_record.visit_number = patient.number_of_visits + 1;
    visit_record.data_hash = data_hash;
    visit_record.timestamp = Clock::get()?.unix_timestamp;
    visit_record.bump = ctx.bumps.visit_record_account;

    patient.number_of_visits = patient.number_of_visits + 1;
    phase_account.total_visits = phase_account.total_visits + 1;

    Ok(())
}

#[derive(Accounts)]
#[instruction(trial_id: String, sponsor_title: String, phase: u8)]

pub struct CreateVisitRecord<'info> {
    pub sponsor_authority: SystemAccount<'info>,
    pub patient_wallet: SystemAccount<'info>,

    #[account(
        seeds=[SPONSOR_SEED, sponsor_authority.key().as_ref(), sponsor_title.as_bytes()],
        bump
    )]
    pub sponsor_account: Account<'info, Sponsor>,

    #[account(
        seeds=[TRIAL_SEED, sponsor_authority.key().as_ref(), trial_id.as_bytes(), sponsor_account.key().as_ref()],
        bump,
        constraint = trial_account.sponsor == sponsor_account.key() @OrdinumError::InvalidTrial
    )]
    pub trial_account: Account<'info, Trial>,

    #[account(
        seeds=[COORDINATOR_SEED, trial_account.key().as_ref(), signer.key().as_ref()],
        bump,
        constraint = coordinator_account.role == CoordinatorRole::CRC @ OrdinumError::Unauthorized
    )]
    pub coordinator_account: Account<'info, Coordinator>,

    #[account(
       mut,
       seeds=[PATIENT_SEED, trial_account.key().as_ref(), patient_wallet.key().as_ref()],
       bump
    )]
    pub patient_account: Account<'info, Patient>,

    #[account(
        mut,
        seeds=[PHASE, trial_account.key().as_ref(), &(phase-1).to_le_bytes()],
        bump
    )]
    pub phase_account: Account<'info, Phase>,

    #[account(
        init,
        space=VisitRecord::SIZE,
        payer=signer,
        seeds=[VISIT_RECORD, trial_account.key().as_ref(), patient_account.key().as_ref(), &phase.to_le_bytes(), &patient_account.number_of_visits.to_le_bytes()],
        bump
    )]
    pub visit_record_account: Account<'info, VisitRecord>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
