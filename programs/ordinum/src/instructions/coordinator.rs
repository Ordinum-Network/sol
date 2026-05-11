use anchor_lang::{accounts::signer, prelude::*};

use crate::{constants::{COORDINATOR_SEED, ESCROW_SEED, SPONSOR_SEED, TRIAL_SEED}, errors::OrdinumError, instructions::{coordinator, trial}, states::{Sponsor, coordinator::Coordinator, enums::{CoordinatorRole, CoordinatorStatus}, escrow::Escrow, trial::Trial}};


pub fn create_coordinator(
    ctx: Context<InitCoordinator>,
    trial_id: String,
    sponsor_title: String,
    coordinator_pubkey: Pubkey,
    role: CoordinatorRole,
) -> Result<()> {
  // let rent = Rent::get()?;
  // let coordinator_rent = rent.minimum_balance(Coordinator::SIZE);

  // let escrow_account = &ctx.accounts.escrow_account;
  let trial_account = &ctx.accounts.trial_account;


  let sponsor_key = ctx.accounts.sponsor_account.key();
  
  //reimbursing signer from escrow ----
  // **ctx.accounts.escrow_account.to_account_info().try_borrow_mut_lamports()? -= coordinator_rent;
  // **ctx.accounts.signer.to_account_info().try_borrow_mut_lamports()? += coordinator_rent;

  //init coordinator acc as usual ---
  ctx.accounts.escrow_account.sol_balance = ctx.accounts.escrow_account.get_lamports();
  let coordinator = &mut ctx.accounts.coordinator_account;
  coordinator.trial_id = trial_account.key();
  coordinator.sponsor = sponsor_key;
  coordinator.role = role;
  coordinator.status = CoordinatorStatus::Active;
  coordinator.assigned_at = Clock::get()?.unix_timestamp;
  coordinator.bump = ctx.bumps.coordinator_account;

  Ok(())
}

pub fn create_coordinator_with_pi(
  ctx: Context<InitCoordinatorWithPI>, 
  trial_id: String, 
  sponsor_title: String,
  coordinator_pubkey: Pubkey,
  role: CoordinatorRole,
) -> Result<()>{
  // let rent = Rent::get()?;
  // let coordinator_rent = rent.minimum_balance(Coordinator::SIZE);

  let escrow_account = &ctx.accounts.escrow_account;
  let trial_account = &ctx.accounts.trial_account;
  let pi = &ctx.accounts.coordinator;

  require!(
    pi.role == CoordinatorRole::PI,
    OrdinumError::Unauthorized,
  );

  // **ctx.accounts.escrow_account.to_account_info().try_borrow_mut_lamports()? -= coordinator_rent;
  // **ctx.accounts.signer.to_account_info().try_borrow_mut_lamports()? +=coordinator_rent;

  ctx.accounts.escrow_account.sol_balance = ctx.accounts.escrow_account.get_lamports();
  let sponsor_key = ctx.accounts.sponsor_account.key();
  let coordinator = &mut ctx.accounts.coordinator_account;

  coordinator.trial_id = trial_account.key();
  coordinator.sponsor = sponsor_key;
  coordinator.role = role;
  coordinator.status = CoordinatorStatus::Active;
  coordinator.assigned_at = Clock::get()?.unix_timestamp;
  coordinator.bump = ctx.bumps.coordinator_account;

  Ok(())
}

#[derive(Accounts)]
#[instruction(trial_id: String, sponsor_title: String, coordinator_pubkey: Pubkey)]
pub struct InitCoordinator<'info> {
  #[account(
   mut,
   seeds=[ESCROW_SEED, trial_id.as_bytes(), sponsor_account.key().as_ref()],
   bump
  )]
  pub escrow_account: Account<'info, Escrow>,

   #[account(
     seeds=[SPONSOR_SEED, signer.key().as_ref(), sponsor_title.as_bytes()],
     bump, 
     constraint = sponsor_account.authority == signer.key() @OrdinumError::Unauthorized
   )] 
   pub sponsor_account: Account<'info, Sponsor>,

   #[account(
     seeds=[TRIAL_SEED, signer.key().as_ref(), trial_id.as_bytes(), sponsor_account.key().as_ref()],
     bump,
     constraint = trial_account.sponsor == sponsor_account.key() @ OrdinumError::InvalidTrial
   )] 
   pub trial_account: Account<'info, Trial>, 
   
   #[account(
    init,
    payer=signer,
    space=Coordinator::SIZE,
    seeds=[COORDINATOR_SEED, trial_account.key().as_ref(), coordinator_pubkey.as_ref()],
    bump
   )]
   pub coordinator_account: Account<'info, Coordinator>,
  
   #[account(mut)]  
   pub signer: Signer<'info>,
   pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(trial_id: String, sponsor_title: String, coordinator_pubkey: Pubkey)]
pub struct InitCoordinatorWithPI<'info> {
  pub sponsor_authority: SystemAccount<'info>,

  #[account(
    mut, 
    seeds=[SPONSOR_SEED, sponsor_authority.key().as_ref(), sponsor_title.as_bytes()],
    bump
  )]
  pub sponsor_account: Account<'info, Sponsor>, 

  #[account(
    mut,
    seeds=[ESCROW_SEED, trial_id.as_bytes(), sponsor_account.key().as_ref()],
    bump
  )]
  pub escrow_account: Account<'info, Escrow>,

  #[account(
    seeds=[TRIAL_SEED, sponsor_authority.key().as_ref(), trial_id.as_bytes(), sponsor_account.key().as_ref()],
    bump,
  )]
  pub trial_account: Account<'info, Trial>,

  #[account(
    mut,
    seeds=[COORDINATOR_SEED, trial_account.key().as_ref(), signer.key().as_ref()],
    bump,
    constraint = coordinator.role == CoordinatorRole::PI @ OrdinumError::Unauthorized,
  )]
  pub coordinator: Account<'info, Coordinator>,

  #[account(
    init, 
    seeds=[COORDINATOR_SEED, trial_account.key().as_ref(), coordinator_pubkey.as_ref()],
    payer=signer,
    space=Coordinator::SIZE,
    bump,
  )]
  pub coordinator_account: Account<'info, Coordinator>,

  #[account(mut)]
  pub signer: Signer<'info>,
  pub system_program: Program<'info, System>,
}

pub fn prefund_signer<'info>(escrow_info: AccountInfo<'info>, escrow: &mut Escrow, signer_info: AccountInfo<'info>) -> Result<()> {
    let rent = Rent::get()?;
    let amount = rent.minimum_balance(Coordinator::SIZE);
    
    require!(
     escrow_info.get_lamports() > amount,
     OrdinumError::InsufficientFunds
  );
    **escrow_info.try_borrow_mut_lamports()? -= amount;
    **signer_info.try_borrow_mut_lamports()? += amount;
    escrow.sol_balance = escrow_info.get_lamports();
    Ok(())
}

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