use anchor_lang::prelude::*;
use crate::{constants::{SPONSOR_SEED, TRIAL_SEED}, errors::*, states::{Sponsor, enums::TrialStatus, trial::Trial}};

pub fn create_trial(
    ctx: Context<CreateTrial>, 
    trial_id: String, 
    sponsor_title: String,
    total_phases: u8,
    start_date: i64,
    end_date: i64
) -> Result<()> {
  let trial = &mut ctx.accounts.trial_account;
 
  require!(
    end_date > start_date,
    OrdinumError::InvalidDate
  );
 
  trial.trial_id = trial_id.clone();
  trial.sponsor = ctx.accounts.sponsor_account.key();
  trial.title = trial_id;
  trial.current_phase = 0;
  trial.total_phases = total_phases;
  trial.status = TrialStatus::Draft;
  trial.amendment_count = 0;
  trial.start_date = start_date;
  trial.end_date = end_date;
  trial.created_date = Clock::get()?.unix_timestamp;
  trial.bump = ctx.bumps.trial_account;
  trial.owner_authority = ctx.accounts.sponsor_account.sponsor_title.clone();

  Ok(())
}

#[derive(Accounts)]
#[instruction(trial_id: String, sponsor_title: String)]
pub struct CreateTrial<'info> {
   #[account(
    seeds=[SPONSOR_SEED, signer.key().as_ref(), sponsor_title.as_bytes()],
    bump,
    constraint = sponsor_account.authority == signer.key() @ OrdinumError::Unauthorized
   )]
   pub sponsor_account: Account<'info, Sponsor>,
  
   #[account(
    init, 
    payer = signer,
    space=Trial::SIZE, 
    seeds=[TRIAL_SEED, signer.key().as_ref(), trial_id.as_bytes(), sponsor_account.key().as_ref()], 
    bump)]
   pub trial_account: Account<'info, Trial>,
  
   #[account(mut)]
   pub signer: Signer<'info>,
   pub system_program: Program<'info, System>,
}
