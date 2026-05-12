use anchor_lang::prelude::*;

use crate::states::Sponsor;

#[derive(Accounts)]
#[instruction(trial_id: String, sponsor_title: String)]

pub struct CreateVisitRecord<'info> {
    pub sponsor_authority: SystemAccount<'info>,
    
    #[account(
        seeds=[],
        bump
    )]
    pub sponsor_account: Account<'info, Sponsor>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}