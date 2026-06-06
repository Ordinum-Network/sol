use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount, Transfer, transfer}};

use crate::{constants::{COORDINATOR_SEED, ESCROW_SEED, PATIENT_SEED, PAYMENT, PHASE, SPONSOR_SEED, TRIAL_SEED, USDC_MINT, VISIT_RECORD}, errors::OrdinumError, instructions::sponsor, states::{Coordinator, CoordinatorRole, Escrow, Payment, Phase, Sponsor, Trial, VisitRecord, patient::Patient}};


pub fn create_payment(
   ctx: Context<CreatePayment>,
   trial_id: String, 
   sponsor_title: String,
   phase: u8,
   amount: u64
) -> Result<()> {
    let payment_acc = &mut ctx.accounts.payment_acc;
    let sponsor_key = ctx.accounts.sponsor_account.key();
   
    payment_acc.trial_id = ctx.accounts.trial_account.key();
    payment_acc.sponsor = ctx.accounts.sponsor_account.key();
    payment_acc.coordinator = ctx.accounts.signer.key();
    payment_acc.phase = phase;
    payment_acc.phase_account = ctx.accounts.phase_account.key();
    payment_acc.visit_record = ctx.accounts.visit_record.key();
    payment_acc.reciever_wallet = ctx.accounts.reciever_wallet.key();
    payment_acc.timestamp = Clock::get()?.unix_timestamp;
    payment_acc.amount = amount;
    payment_acc.bump = ctx.bumps.payment_acc;

    //initiate_transfer ----
 let escrow_bump = ctx.accounts.escrow_account.bump;
let escrow_seeds = &[
    ESCROW_SEED,
    trial_id.as_bytes(),
    sponsor_key.as_ref(),
    &[escrow_bump],
];
let signer_seeds = &[&escrow_seeds[..]];

let cpi_accounts = Transfer {
    from: ctx.accounts.escrow_token_account.to_account_info(),
    to: ctx.accounts.reciever_token_account.to_account_info(),
    authority: ctx.accounts.escrow_account.to_account_info(), // escrow PDA, not sponsor
};

let cpi_ctx = CpiContext::new_with_signer(
    ctx.accounts.token_program.to_account_info(),
    cpi_accounts,
    signer_seeds,
);

transfer(cpi_ctx, amount)?;

    Ok(())
}


#[derive(Accounts)]
#[instruction(trial_id: String, sponsor_title: String, phase: u8)]
pub struct CreatePayment<'info> {
    pub sponsor_authority: SystemAccount<'info>,
    pub reciever_wallet: SystemAccount<'info>,
    
    #[account(
      address = USDC_MINT
    )]
    pub usdc_mint: Box<Account<'info, Mint>>,

    #[account(
       seeds=[SPONSOR_SEED, sponsor_authority.key().as_ref(), sponsor_title.as_bytes()],
       bump
    )]
    pub sponsor_account: Box<Account<'info, Sponsor>>,
    
    #[account(
       seeds=[TRIAL_SEED, sponsor_authority.key().as_ref(), trial_id.as_bytes(), sponsor_account.key().as_ref()],
       bump
    )]
    pub trial_account: Box<Account<'info, Trial>>,
    
    #[account(
       seeds=[COORDINATOR_SEED, trial_account.key().as_ref(), signer.key().as_ref()],
       bump,
       constraint = coordinator_account.role == CoordinatorRole::CRC @ OrdinumError::Unauthorized
    )]
    pub coordinator_account: Box<Account<'info, Coordinator>>,
    
    #[account(
      seeds=[PATIENT_SEED, trial_account.key().as_ref(), reciever_wallet.key().as_ref()],
      bump,
    )]
    pub patient_account: Box<Account<'info, Patient>>,

    #[account(
     seeds=[PHASE, trial_account.key().as_ref(), &phase.to_le_bytes()],
     bump
    )]
    pub phase_account: Box<Account<'info, Phase>>,

    #[account(
      mut, 
      seeds=[ESCROW_SEED, trial_id.as_bytes(), sponsor_account.key().as_ref()],
      bump
    )]
    pub escrow_account: Box<Account<'info, Escrow>>,
    
    #[account(
      mut,
      associated_token::mint=usdc_mint,
      associated_token::authority=escrow_account
    )]
    pub escrow_token_account: Box<Account<'info, TokenAccount>>,
    
    #[account(
      seeds=[VISIT_RECORD, trial_account.key().as_ref(), patient_account.key().as_ref(), &phase_account.phase_number.to_le_bytes(), &(patient_account.number_of_visits-1).to_le_bytes()],
      bump
    )]
    pub visit_record: Box<Account<'info, VisitRecord>>,

    #[account(
      init,
      space=Payment::SIZE,
      payer=signer,
      seeds=[PAYMENT, trial_account.key().as_ref(), &phase.to_le_bytes(), visit_record.key().as_ref(), patient_account.key().as_ref()],      
      bump
    )]
    pub payment_acc: Box<Account<'info, Payment>>,

    //recipients ATA
    #[account(
      init_if_needed, 
      payer=signer,
      associated_token::mint=usdc_mint,
      associated_token::authority = reciever_wallet
    )]
    pub reciever_token_account: Box<Account<'info, TokenAccount>>,

    ////
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

