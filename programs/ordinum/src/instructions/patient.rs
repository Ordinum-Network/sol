use anchor_lang::prelude::*;

use crate::{constants::{COORDINATOR_SEED, PATIENT_SEED, SPONSOR_SEED, TRIAL_SEED}, errors::OrdinumError, states::{Coordinator, CoordinatorRole, PatientStatus, Sponsor, Trial, patient::Patient}};

pub fn init_patient(
    ctx: Context<CreatePatient>,
    trial_id: String, 
    sponsor_title: String,
    consent_hash: [u8;32],
) -> Result<()>{
    
    let coordinator = &ctx.accounts.coordinator_account;
    let patient = &mut ctx.accounts.patient_account;
    require!(
      coordinator.role == CoordinatorRole::CRC,
      OrdinumError::Unauthorized,
    );
    
    patient.trial_id = ctx.accounts.trial_account.key();
    patient.sponsor = ctx.accounts.sponsor_account.key();
    patient.wallet = ctx.accounts.patient_wallet.key();
    patient.consent_hash = consent_hash;
    patient.status = PatientStatus::Active;
    patient.enrolled_at = Clock::get()?.unix_timestamp;
    patient.bump = ctx.bumps.patient_account;
    patient.last_modified = Clock::get()?.unix_timestamp;
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(trial_id: String, sponsor_title: String)]
pub struct CreatePatient<'info> {
  pub sponsor_authority: SystemAccount<'info>,
  pub patient_wallet: SystemAccount<'info>,

  #[account(
     seeds=[SPONSOR_SEED, sponsor_authority.key().as_ref(), sponsor_title.as_bytes()],
     bump,
  )]
  pub sponsor_account: Account<'info, Sponsor>,

  #[account(
    seeds=[TRIAL_SEED, sponsor_authority.key().as_ref(), trial_id.as_bytes(), sponsor_account.key().as_ref()],
    bump,
  )]
  pub trial_account: Account<'info, Trial>,

  #[account(
    seeds=[COORDINATOR_SEED, trial_account.key().as_ref(), signer.key().as_ref()],
    bump,
    constraint = coordinator_account.role == CoordinatorRole::CRC @OrdinumError::Unauthorized
  )]
  pub coordinator_account: Account<'info, Coordinator>,

  #[account(
    init,
    payer=signer,
    space=Patient::SIZE,
    seeds=[PATIENT_SEED, trial_account.key().as_ref(), patient_wallet.key().as_ref()],
    bump,
  )]
  pub patient_account: Account<'info, Patient>,

  #[account(mut)]
  pub signer: Signer<'info>,
  pub system_program: Program<'info, System>,
}