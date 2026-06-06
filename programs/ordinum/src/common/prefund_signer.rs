use anchor_lang::prelude::*;

use crate::{constants::{COORDINATOR_SEED, ESCROW_SEED, SPONSOR_SEED, TRIAL_SEED}, errors::OrdinumError, states::{AccountType, Coordinator, CoordinatorRole, Escrow, Payment, Phase, Sponsor, Trial, VisitRecord, patient::Patient}};

#[derive(Accounts)]
#[instruction(trial_id: String, sponsor_title: String)]
pub struct PrefundSignerAsSponsor<'info> {
    pub sponsor_authority: SystemAccount<'info>,
     
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
        mut,
        seeds=[ESCROW_SEED, trial_id.as_bytes(), sponsor_account.key().as_ref()],
        bump
    )]
    pub escrow_account: Account<'info, Escrow>,

    /// CHECK: recipient to fund
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(trial_id: String, sponsor_title: String)]
pub struct PrefundSignerAsPI<'info> {
    pub sponsor_authority: SystemAccount<'info>,
     
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
        mut,
        seeds=[ESCROW_SEED, trial_id.as_bytes(), sponsor_account.key().as_ref()],
        bump
    )]
    pub escrow_account: Account<'info, Escrow>,

    #[account(
       mut,
        seeds=[COORDINATOR_SEED, trial_account.key().as_ref(), signer.key().as_ref()],
        bump,
        constraint = coordinator.role == CoordinatorRole::PI @ OrdinumError::Unauthorized,
    )]
    pub coordinator: Account<'info, Coordinator>,

    /// CHECK: recipient to fund
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(trial_id: String, sponsor_title: String)]
pub struct PrefundSignerAsCRC<'info> {
    pub sponsor_authority: SystemAccount<'info>,
     
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
        mut,
        seeds=[ESCROW_SEED, trial_id.as_bytes(), sponsor_account.key().as_ref()],
        bump
    )]
    pub escrow_account: Account<'info, Escrow>,

    #[account(
       mut,
        seeds=[COORDINATOR_SEED, trial_account.key().as_ref(), signer.key().as_ref()],
        bump,
        constraint = coordinator.role == CoordinatorRole::CRC @ OrdinumError::Unauthorized,
    )]
    pub coordinator: Account<'info, Coordinator>,

    /// CHECK: recipient to fund
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn prefund_signer<'info>(escrow_info: AccountInfo<'info>, escrow: &mut Escrow, signer_info: AccountInfo<'info>, accountType: AccountType) -> Result<()> {
    let rent = Rent::get()?;
    let mut amount: u64;
    match accountType {
      AccountType::Coordinator => {
        amount = rent.minimum_balance(Coordinator::SIZE);
      }
      AccountType::Patient => {
        amount = rent.minimum_balance(Patient::SIZE);
      }
      AccountType::VisitRecord => {
        amount = rent.minimum_balance(VisitRecord::SIZE);
      }
      AccountType::Phase => {
        amount = rent.minimum_balance(Phase::SIZE);
      }
      AccountType::Payment => {
        amount = rent.minimum_balance(Payment::SIZE);
      }
      AccountType::ATA => {
        amount = 2039280;
      }
    }
    
    require!(
     escrow_info.get_lamports() > amount,
     OrdinumError::InsufficientFunds
  );
    **escrow_info.try_borrow_mut_lamports()? -= amount;
    **signer_info.try_borrow_mut_lamports()? += amount;
    escrow.sol_balance = escrow_info.get_lamports();
    Ok(())
}