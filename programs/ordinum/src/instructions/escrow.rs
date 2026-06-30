use crate::{
    constants::{ESCROW_SEED, SPONSOR_SEED, TRIAL_SEED, USDC_MINT},
    errors::OrdinumError,
    states::{escrow::Escrow, trial::Trial, Sponsor},
};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

pub fn init_escrow(
    ctx: Context<InitEscrow>,
    trial_id: String,
    sponsor_title: String,
    initial_deposit: u64,
    sol_deposit: u64,
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

    //transferring minimum sol ------
    let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.signer.key(),
        &escrow_acc.key(),
        sol_deposit,
    );

    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            ctx.accounts.signer.to_account_info(),
            escrow_acc.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    escrow_acc.sol_balance = sol_deposit;

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
    constraint = trial_account.sponsor == sponsor_account.key() @ OrdinumError::InvalidTrial
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

pub fn top_up_sol(
    ctx: Context<UpdateEscrow>,
    trial_id: String,
    sponsor_title: String,
    sol: u64,
) -> Result<()> {
    let escrow_account: &mut Account<'_, Escrow> = &mut ctx.accounts.escrow_account;

    let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
      &ctx.accounts.signer.key(),
      &escrow_account.key(),
      sol
    );

    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            ctx.accounts.signer.to_account_info(),
            escrow_account.to_account_info(),
            ctx.accounts.system_program.to_account_info()
        ]
    )?;

    // **ctx.accounts.signer.to_account_info().try_borrow_mut_lamports()? -= sol;
    // **escrow_account.to_account_info().try_borrow_mut_lamports()? += sol;
    
    escrow_account.sol_balance = escrow_account.get_lamports();

    Ok(())
}

pub fn top_up_tokens(
    ctx: Context<UpdateEscrow>,
    trial_id: String,
    sponsor_title: String,
    usdc: u64
) -> Result<()> {
    let cpi_accounts = Transfer {
        from: ctx.accounts.sponsor_token_account.to_account_info(),
        to: ctx.accounts.escrow_token_account.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
    let escrow_acc = &mut ctx.accounts.escrow_account;
    let escrow_token_account = &ctx.accounts.escrow_token_account;
    
    transfer(cpi_ctx, usdc)?;
    
    escrow_acc.balance = escrow_token_account.amount;

    Ok(())
}

#[derive(Accounts)]
#[instruction(trial_id: String, sponsor_title: String)]
pub struct UpdateEscrow<'info> {
   #[account(
    address = USDC_MINT
    )]
    pub usdc_mint: Account<'info, Mint>,

    #[account(
    seeds=[SPONSOR_SEED, signer.key().as_ref(), sponsor_title.as_bytes()],
    bump,
    constraint = sponsor_account.authority == signer.key() @ OrdinumError::Unauthorized
   )]
    pub sponsor_account: Box<Account<'info, Sponsor>>,

    #[account(
    mut,
    associated_token::mint = usdc_mint,
    associated_token::authority = signer,
   )]
    pub sponsor_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
    seeds=[TRIAL_SEED, signer.key().as_ref(), trial_id.as_bytes(), sponsor_account.key().as_ref()],
    bump,
    constraint = trial_account.sponsor == sponsor_account.key() @ OrdinumError::InvalidTrial
   )]
    pub trial_account: Box<Account<'info, Trial>>,

   #[account(
    mut,
    seeds=[ESCROW_SEED, trial_id.as_bytes(), sponsor_account.key().as_ref()],
    bump
   )]
   pub escrow_account: Box<Account<'info, Escrow>>,

    // associated token account --------
   #[account(
    mut,
    associated_token::mint = usdc_mint,
    associated_token::authority=escrow_account
   )]
    pub escrow_token_account: Box<Account<'info, TokenAccount>>,
    ////
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}
