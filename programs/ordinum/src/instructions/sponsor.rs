use anchor_lang::prelude::*;
use crate::states::{Sponsor};
use crate::constants::*;

pub fn initialise_sponsor(ctx: Context<InitSponsor>, sponsor_title: String) -> Result<()> {
    let sponsor = &mut ctx.accounts.sponsor_account;
    sponsor.authority = ctx.accounts.signer.key();
    sponsor.verified = false;
    sponsor.sponsor_title = sponsor_title;
    sponsor.created_at = Clock::get()?.unix_timestamp;
    sponsor.bump = ctx.bumps.sponsor_account;
    Ok(())
}

pub fn update_sponsor_verified(ctx: Context<UpdateSponsor>, sponsor_title: String, verified: bool) -> Result<()> {
    let sponsor = &mut ctx.accounts.sponsor_acc;
    sponsor.verified = verified;
    Ok(())
}

// pub fn update_sponsor_trialcount(ctx: Context<UpdateSponsor>, sponsor_title: String, trialCount:)

#[derive(Accounts)]
#[instruction(sponsor_title: String)]
pub struct InitSponsor<'info> { 
  #[account(
    init, 
    space=Sponsor::SIZE, 
    payer=signer, 
    seeds=[SPONSOR_SEED, signer.key().as_ref(), sponsor_title.as_bytes()], 
    bump
  )]
  pub sponsor_account: Account<'info, Sponsor>,
  
  #[account(mut)]  
  pub signer: Signer<'info>, 
  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(sponsor_title: String)]
pub struct UpdateSponsor<'info> {
  #[account(
    mut,
    has_one=authority,
    seeds=[SPONSOR_SEED, authority.key().as_ref()],
    bump=sponsor_acc.bump
  )]
  pub sponsor_acc: Account<'info, Sponsor>,

  pub authority: Signer<'info>
}
