use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};
use anchor_spl::associated_token::AssociatedToken;
use crate::{constants::{SPONSOR_SEED, TRIAL_SEED, ESCROW_SEED}, errors::OrdinumError, states::{Sponsor, escrow::Escrow, trial::Trial}};


pub fn init_escrow(
  ctx: Context<InitEscrow>
) -> Result<()> {
  Ok(())
}


#[derive(Accounts)]
#[instruction(trial_id: String, sponsor_title: String)]
pub struct InitEscrow<'info> {
   pub usdc_mint: Account<'info, Mint>,
   #[account(
    seeds=[SPONSOR_SEED, signer.key().as_ref(), sponsor_title.as_bytes()],
    bump,
    constraint = sponsor_account.authority == signer.key() @ OrdinumError::Unauthorized
   )]
   pub sponsor_account: Account<'info, Sponsor>,

   #[account(
    seeds=[TRIAL_SEED, signer.key().as_ref(), trial_id.as_bytes(), sponsor_account.key().as_ref()],
    bump,
    constraint = trial_account.sponsor == signer.key() @ OrdinumError::InvalidTrial
   )]
   pub trial_account: Account<'info, Trial>,

   #[account(
    init,
    payer=signer,
    space=Escrow::SIZE,
    seeds=[ESCROW_SEED, trial_id.as_bytes(), sponsor_account.key().as_ref()],
    bump
   )]
   pub escrow_account: Account<'info, Escrow>,

   // associated token account --------

   #[account(mut)]
   pub signer: Signer<'info>,
   pub system_program: Program<'info, System>,
}