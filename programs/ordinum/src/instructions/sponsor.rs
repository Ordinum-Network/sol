use anchor_lang::prelude::*;

use crate::states::{Sponsor};
use crate::constants::*;

pub fn initialise_sponsor(ctx: Context<InitSponsor>, sponsor_title: String) -> Result<()> {
    let sponsor = &mut ctx.accounts.sponsor_account;
    sponsor.authority = ctx.accounts.signer.key();
    sponsor.verified = false;
    sponsor.sponsor_title = sponsor_title;
    sponsor.trial_count = 0;
    sponsor.created_at = Clock::get()?.unix_timestamp;
    sponsor.bump = ctx.bumps.sponsor_account;
    Ok(())
}

#[derive(Accounts)]
#[instruction(sponsor_title: String)]
pub struct InitSponsor<'info> { 
  #[account(
    init, 
    space=8 + 32 + 4 + 64 + 1 + 8 + 8 + 1, 
    payer=signer, 
    seeds=[SPONSOR_SEED, signer.key().as_ref(), sponsor_title.as_bytes()], 
    bump
  )]
  pub sponsor_account: Account<'info, Sponsor>,
  
  #[account(mut)]  
  pub signer: Signer<'info>, 
  pub system_program: Program<'info, System>
}

