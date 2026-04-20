use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer, transfer};
use anchor_spl::associated_token::AssociatedToken;
use crate::{constants::{SPONSOR_SEED, TRIAL_SEED, ESCROW_SEED, USDC_MINT}, errors::OrdinumError, states::{Sponsor, escrow::Escrow, trial::Trial}};


pub fn init_escrow(
  ctx: Context<InitEscrow>,
  trial_id: String,
  sponsor_title: String,
  initial_deposit: u64,
) -> Result<()> {
  let escrow_acc: &mut Account<'_, Escrow> = &mut ctx.accounts.escrow_account;

  escrow_acc.sponsor = ctx.accounts.signer.key();
  escrow_acc.trial = ctx.accounts.trial_account.key();
  escrow_acc.usdc_mint = ctx.accounts.usdc_mint.key();
  escrow_acc.initial_deposit = initial_deposit;
  escrow_acc.total_deposit = initial_deposit;
  escrow_acc.balance = initial_deposit;
  escrow_acc.bump = ctx.bumps.escrow_account;
  
  //transfer usdc from sponsor ATA to ecsrow ATA -----
  let cpi_accounts = Transfer {
     from: ctx.accounts.sponsor_token_account.to_account_info(),
     to: ctx.accounts.escrow_token_account.to_account_info(),
     authority: ctx.accounts.signer.to_account_info(),
  };
  let cpi_program = ctx.accounts.token_program.to_account_info();
  let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

  transfer(cpi_ctx, initial_deposit)?;
  Ok(())
}


#[derive(Accounts)]
#[instruction(trial_id: String, sponsor_title: String)]
pub struct InitEscrow<'info> {
   #[account(
    address = USDC_MINT
   )]
   pub usdc_mint: Account<'info, Mint>,
   
   #[account(
    seeds=[SPONSOR_SEED, signer.key().as_ref(), sponsor_title.as_bytes()],
    bump,
    constraint = sponsor_account.authority == signer.key() @ OrdinumError::Unauthorized
   )]
   pub sponsor_account: Account<'info, Sponsor>,

   #[account(
    mut,
    associated_token::mint = usdc_mint,
    associated_token::authority = signer,
   )]
   pub sponsor_token_account: Account<'info, TokenAccount>,

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

   #[account(
    init,
    payer = signer,
    associated_token::mint = usdc_mint,
    associated_token::authority=escrow_account
   )]
   pub escrow_token_account: Account<'info, TokenAccount>,
   ////
   pub token_program: Program<'info, Token>,
   pub associated_token_program: Program<'info, AssociatedToken>,

   #[account(mut)]
   pub signer: Signer<'info>,
   pub system_program: Program<'info, System>,
}