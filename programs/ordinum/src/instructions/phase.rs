use anchor_lang::prelude::*;

pub fn create_phase(ctx: Context<CreatePhase>) -> Result<()> {
  Ok(())
}

#[derive(Accounts)]
#[instruction(trial_id: String, sponsor_title: String, phase:u8)]
pub struct CreatePhase<'info> {
    pub sponsor_authority: SystemAccount<'info>,
    
}