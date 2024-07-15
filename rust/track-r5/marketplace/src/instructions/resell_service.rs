use crate::{
    constants::SERVICE_ACCOUNT_SEEDS, state::{ServiceAccount, ServiceAgreement},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token_2022::{self, Token2022}, token_interface::{Mint, TokenAccount as ITokenAccount}, token::TokenAccount
};
use utils;

#[derive(Accounts)]
pub struct Resell<'info> {
    #[account(
        mut,
        constraint = reseller_service_ticket_token.amount == 1,
        associated_token::token_program = token_program,
        associated_token::mint = service_ticket_mint,
        associated_token::authority = reseller,
    )]
    pub reseller_service_ticket_token: Box<InterfaceAccount<'info, ITokenAccount>>,
    
    #[account(
        init_if_needed,
        payer=payer,
        associated_token::token_program = token_program,
        associated_token::mint = service_ticket_mint,
        associated_token::authority = payer,
    )]
    pub payer_service_ticket_token: Box<InterfaceAccount<'info, ITokenAccount>>,

    /// CHECK: mint account, yet to be initialized
    #[account(
        mut,
    )]
    pub service_ticket_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        constraint=service_account.holder==reseller.key(), 
        constraint=service_account.mint==service_ticket_mint.key(), 
        seeds=[SERVICE_ACCOUNT_SEEDS, service_ticket_mint.key().as_ref()],
        bump=service_account.bump

    )]
    pub service_account: Box<Account<'info, ServiceAccount>>,
    
    /// CHECK: receiver of the service nft
    #[account()]
    pub provider: Signer<'info>,

    /// CHECK: receiver of the service nft
    #[account()]
    pub reseller: Signer<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: ...
    #[account(
        mut
        // seeds = [b"extra-account-metas", service_ticket_mint.key().as_ref()],
        // bump,
        // seeds::program =  
    )]
    pub extra_account_metas_list: UncheckedAccount<'info>,
    
    /// CHECK:...
    pub wsol_mint: UncheckedAccount<'info>,
    /// CHECK:...
    #[account(mut)]
    pub mint_royalty_wsol_token_account: UncheckedAccount<'info>,
    /// CHECK:...
    #[account(
        init_if_needed,
        payer=payer,
        associated_token::token_program=token_program_classic,
        associated_token::authority=reseller,
        associated_token::mint=wsol_mint,
    )]
    pub reseller_wsol_token_account: Box<Account<'info, TokenAccount>>,
    
    /// CHECK:...
    #[account(
        init_if_needed,
        payer=payer,
        associated_token::token_program=token_program_classic,
        associated_token::authority=provider,
        associated_token::mint=wsol_mint,
    )]
    pub provider_wsol_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK:...
    #[account(
        mut
        // seeds = [b"extra-account-metas", service_ticket_mint.key().as_ref()],
        // bump,
        // seeds::program =  
    )]
    pub mint_royalty_config: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    
    pub associated_token_program: Program<'info, AssociatedToken>,
    
    /// CHECK:...
    #[account(executable)]
    pub transfer_hook_program: UncheckedAccount<'info>,

    /// CHECK:...
    #[account(executable)]
    pub token_program_classic: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token2022>,
}

pub fn resell<'info>(ctx: Context<'_, '_, 'info, 'info, Resell<'info>>) -> Result<()> {
    let service_ticket = &ctx.accounts.service_ticket_mint;
    let reseller_service_ticket_token = &ctx.accounts.reseller_service_ticket_token;
    let payer_service_ticket_token = &ctx.accounts.payer_service_ticket_token;
    let token_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let payer = &ctx.accounts.payer;
    let reseller = &ctx.accounts.reseller;
    let extra_account_metas_list = &ctx.accounts.extra_account_metas_list;
    let token_program_classic = &ctx.accounts.token_program_classic;
    let transfer_hook_program = &ctx.accounts.transfer_hook_program;
    let mint_royalty_wsol_token_account = &ctx.accounts.mint_royalty_wsol_token_account;
    let reseller_wsol_token_account = &ctx.accounts.reseller_wsol_token_account;
    let provider_wsol_token_account = &ctx.accounts.provider_wsol_token_account;
    let mint_royalty_config = &ctx.accounts.mint_royalty_config;
    
    update_royalty_config(&ctx, true)?;

    let (reseller_amount, provider_amount) = ServiceAgreement::try_from(service_ticket.to_account_info())?.royalties_split()?;

    utils::system_program_transfer(
        reseller_amount + provider_amount, 
        system_program, 
        payer, 
        &mint_royalty_wsol_token_account.to_account_info()
    )?;

    token_2022::spl_token_2022::onchain::invoke_transfer_checked(
        token_program.key,
        reseller_service_ticket_token.clone().to_account_info(),
        service_ticket.clone().to_account_info(),
        payer_service_ticket_token.clone().to_account_info(),
        reseller.clone().to_account_info(),
        &[
            extra_account_metas_list.to_account_info(),
            mint_royalty_config.to_account_info(),
            transfer_hook_program.clone().to_account_info(),
            ],
        1,
        service_ticket.decimals,
        &[],
    )?;

    utils::token_sync_native(&**provider_wsol_token_account, token_program_classic)?;
    utils::token_sync_native(&**reseller_wsol_token_account, token_program_classic)?;

    update_royalty_config(&ctx, false)?;
    Ok(())
}


fn update_royalty_config(ctx: &Context<Resell>, is_selling: bool) -> Result<()> {
    marketplace_transfer_controller::cpi::royalty_config_update(
        CpiContext::new_with_signer(
            ctx.accounts.transfer_hook_program.to_account_info(), 
            marketplace_transfer_controller::cpi::accounts::RoyaltyConfigUpdate {
                mint_royalty_config: ctx.accounts.mint_royalty_config.to_account_info(),
                service_account: ctx.accounts.service_account.to_account_info(),
                service_ticket_mint: ctx.accounts.service_ticket_mint.to_account_info(),
            },
            &[
               &[ b"service-account".as_ref(),
                ctx.accounts.service_ticket_mint.key().as_ref(),
                &[ctx.accounts.service_account.bump]]
            ]
        ), 
        is_selling
    )
}