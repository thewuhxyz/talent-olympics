use crate::state::ServiceAccount;
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
        associated_token::authority = holder,
    )]
    pub service_ticket_token: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mint::decimals = 0,
        extensions::metadata_pointer::metadata_address = service_ticket_mint,
        extensions::group_member_pointer::member_address = service_ticket_mint,
    )]
    pub service_ticket_mint: InterfaceAccount<'info, Mint>,
    
    /// CHECK: current holder of the service nft
    #[account()]
    pub holder: Signer<'info>,
    
    #[account(
        mut,
        has_one=holder, 
        constraint=service_account.mint==service_ticket_mint.key(), 
        seeds=[service_ticket_mint.key().as_ref()],
        bump
    )]
    pub service_account: Account<'info, ServiceAccount>,
    
    pub system_program: Program<'info, System>,
    
    pub associated_token_program: Program<'info, AssociatedToken>,
    
    pub token_program: Program<'info, Token2022>,
}

pub fn relist(ctx: Context<Relist>) -> Result<()> {
    let service_account = &mut ctx.accounts.service_account;
    let service_ticket_token = &ctx.accounts.service_ticket_token;
    let holder = &ctx.accounts.holder;
    let token_program = &ctx.accounts.token_program;
    
    utils::approve_delegate(
        1, 
        service_account, // token account delegate
        service_ticket_token, 
        holder,
        token_program
    )?;
    
    service_account.list();

    Ok(())
}
