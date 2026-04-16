use anchor_lang::prelude::*;
use crate::{constants::{SPONSOR_SEED, TRIAL_SEED}, errors::*, states::{Sponsor, enums::TrialStatus, trial::Trial}};

pub fn create_trial(
    ctx: Context<CreateTrial>, 
    trialName: String, 
    name: String,
    currentPhase: u8,
    totalPhases: u8,
    startDate: i64,
    endDate: i64
) -> Result<()> {
  let trial = &mut ctx.accounts.trial_account;
 
  require!(
    endDate > startDate,
    OrdinumError::InvalidDate
  );
 
  trial.trial_id = trialName.clone();
  trial.sponsor = ctx.accounts.signer.key();
  trial.title = trialName;
  trial.currentPhase = currentPhase;
  trial.totalPhases = totalPhases;
  trial.status = TrialStatus::Draft;
  trial.amendmentCount = 0;
  trial.startDate = startDate;
  trial.endDate = endDate;
  trial.createdDate = Clock::get()?.unix_timestamp;
  trial.bump = ctx.bumps.trial_account;

  Ok(())
}

#[derive(Accounts)]
#[instruction(trial_id: String, name: String)]
pub struct CreateTrial<'info> {
   #[account(
    init, 
    payer = signer,
    space=Trial::SIZE, 
    seeds=[TRIAL_SEED, signer.key().as_ref(), trial_id.as_bytes()], 
    bump)]
   pub trial_account: Account<'info, Trial>,

   #[account(
    seeds=[SPONSOR_SEED, signer.key().as_ref(), name.as_bytes()],
    bump,
    constraint = sponsor_account.authority == signer.key() @ OrdinumError::Unauthorized
   )]
   pub sponsor_account: Account<'info, Sponsor>,
  
   #[account(mut)]
   pub signer: Signer<'info>,
   pub system_program: Program<'info, System>,
}
