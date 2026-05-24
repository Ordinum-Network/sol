use std::cmp::Ordering;

use anchor_lang::prelude::*;

use crate::{constants::{COORDINATOR_SEED, PHASE, SPONSOR_SEED, TRIAL_SEED}, errors::OrdinumError, instructions::sponsor, program::Ordinum, states::{Coordinator, CoordinatorRole, Phase, Sponsor, Trial}};

pub fn create_phase(
  ctx: Context<CreatePhase>,
  trial_id: String,
  sponsor_title: String,
  data_hash: [u8;32],
  total_visits: u16
) -> Result<()> {
  
  let pi = &ctx.accounts.coordinator_account;
  let trial = &ctx.accounts.trial_account;
  
  require!(
    pi.role == CoordinatorRole::CRC,
    OrdinumError::Unauthorized,
  );
  
  let phase = &mut ctx.accounts.phase;

  phase.trial_id = trial.key();
  phase.sponsor = ctx.accounts.sponsor_account.key();
  phase.phase_number = trial.current_phase+1;
  phase.data_hash = data_hash;
  phase.completed_by = ctx.accounts.coordinator_account.key();
  phase.total_visits = total_visits;
  phase.completed_at = 0;
  phase.bump = ctx.bumps.phase;
  
  Ok(())
}

#[derive(Accounts)]
#[instruction(trial_id: String, sponsor_title: String)]
pub struct CreatePhase<'info> {
    pub sponsor_authority: SystemAccount<'info>,
    
    #[account(
      mut,
      seeds=[SPONSOR_SEED, sponsor_authority.key().as_ref(), sponsor_title.as_bytes()],
      bump
    )]
    pub sponsor_account: Account<'info, Sponsor>,

    #[account(
      mut,
      seeds=[TRIAL_SEED, sponsor_authority.key().as_ref(), trial_id.as_bytes(), sponsor_account.key().as_ref()],
      bump
    )]
    pub trial_account: Account<'info, Trial>,
    
    #[account(
      mut,
      seeds=[COORDINATOR_SEED, trial_account.key().as_ref()],
      bump,
      constraint = coordinator_account.role == CoordinatorRole::CRC @ OrdinumError::Unauthorized,
    )]
    pub coordinator_account: Account<'info, Coordinator>,

    #[account(
      init,
      payer=signer,
      space=Phase::SIZE,
      seeds=[PHASE, trial_account.key().as_ref(), &trial_account.current_phase.to_le_bytes()],
      bump
    )]
    pub phase: Account<'info, Phase>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    
}