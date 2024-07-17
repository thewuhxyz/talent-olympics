use crate::{
    constants::SERVICE_ACCOUNT_SEEDS, state::ServiceAccount
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, Token2022, TokenAccount},
};
#[derive(Accounts)]
pub struct Relist<'info> {
    #[account(
        mut,
        constraint = service_ticket_token.amount == 1,
        associated_token::token_program = token_program,
        associated_token::mint = service_ticket_mint,
        associated_token::authority = reseller,
    )]
    pub service_ticket_token: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: mint account, yet to be initialized
    #[account()]
    pub service_ticket_mint: InterfaceAccount<'info, Mint>,
    
    #[account()]
    pub service_mint: InterfaceAccount<'info, Mint>,

    /// CHECK: receiver of the service nft
    #[account()]
    pub reseller: Signer<'info>,
    
    #[account(
        mut,
        constraint=service_account.holder==reseller.key(), 
        constraint=service_account.mint==service_ticket_mint.key(), 
        seeds=[SERVICE_ACCOUNT_SEEDS, service_ticket_mint.key().as_ref()],
        bump=service_account.bump

    )]
    pub service_account: Account<'info, ServiceAccount>,
    
    pub system_program: Program<'info, System>,
    
    pub associated_token_program: Program<'info, AssociatedToken>,
    
    pub token_program: Program<'info, Token2022>,

    /// CHECK: royalty config
    pub delegate_signer: UncheckedAccount<'info>,
}

pub fn relist(ctx: Context<Relist>) -> Result<()> {
    let service_account = &mut ctx.accounts.service_account;
    let service_ticket_token = &ctx.accounts.service_ticket_token;
    let reseller = &ctx.accounts.reseller;
    let token_program = &ctx.accounts.token_program;
    let _delegate_signer = &ctx.accounts.delegate_signer;
    
    utils::approve_delegate(
        1, 
        service_account, // token account delegate
        service_ticket_token, 
        reseller,
        token_program
    )?;
    
    service_account.list();

    Ok(())
}
