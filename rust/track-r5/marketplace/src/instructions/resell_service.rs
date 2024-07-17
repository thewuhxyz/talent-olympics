use crate::{
    constants::SERVICE_ACCOUNT_SEEDS, state::{ServiceAccount, ServiceAgreement}, error::ErrorCode
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token_2022::{self, Token2022}, token_interface::{Mint, TokenAccount as ITokenAccount, TokenInterface}
};
use marketplace_transfer_controller::state::MintRoyaltyConfig;
// use utils;

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
    #[account()]
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
    #[account(mut)]
    pub provider: UncheckedAccount<'info>,

    /// CHECK: receiver of the service nft
    #[account(mut)]
    pub reseller: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// CHECK:...
    #[account(
        mut,
        // zero,
        seeds=[service_ticket_mint.key().as_ref()],
        bump,
        seeds::program=transfer_hook_program.key(),
        owner=transfer_hook_program.key()
    )]
    pub mint_royalty_config: Account<'info, MintRoyaltyConfig>,

    pub system_program: Program<'info, System>,
    
    pub associated_token_program: Program<'info, AssociatedToken>,
    
    /// CHECK:...
    #[account(executable)]
    pub transfer_hook_program: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token2022>,
}

pub fn resell<'info>(ctx: Context<'_, '_, 'info, 'info, Resell<'info>>) -> Result<()> {
    msg!("start");
    let service_ticket = &ctx.accounts.service_ticket_mint;
    let reseller_service_ticket_token = &ctx.accounts.reseller_service_ticket_token;
    let payer_service_ticket_token = &ctx.accounts.payer_service_ticket_token;
    let token_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let payer = &ctx.accounts.payer;
    let reseller = &ctx.accounts.reseller;
    let provider = &ctx.accounts.provider;
    let service_account = &ctx.accounts.service_account;

    // perfrom massive checks on all remaining accounts
    let payment_token_program = &Interface::<TokenInterface>::try_from(&ctx.remaining_accounts[0])?;
    let payment_token_mint = InterfaceAccount::<Mint>::try_from(&ctx.remaining_accounts[1])?;

    let provider_payment_token_account_unchecked = &ctx.remaining_accounts[2];
    let reseller_payment_token_account_unchecked = &ctx.remaining_accounts[3];
    let payer_payment_token_account_unchecked = &ctx.remaining_accounts[4];
    
    let extra_account_metas_list = &ctx.remaining_accounts[5];
    // let mint_royalty_config = &ctx.remaining_accounts[6];
    let mint_royalty_config = &ctx.accounts.mint_royalty_config;
    let transfer_hook_program = &ctx.accounts.transfer_hook_program;

    // perform massive check
    init_if_needed_token_account(&ctx, provider_payment_token_account_unchecked, provider)?;
    init_if_needed_token_account(&ctx, reseller_payment_token_account_unchecked, reseller)?;
    init_if_needed_token_account(&ctx, payer_payment_token_account_unchecked, payer)?;

    msg!("3");
    
    let provider_payment_token_account = InterfaceAccount::<ITokenAccount>::try_from(provider_payment_token_account_unchecked)?;
    let reseller_payment_token_account = InterfaceAccount::<ITokenAccount>::try_from(reseller_payment_token_account_unchecked)?;
    let payer_payment_token_account = InterfaceAccount::<ITokenAccount>::try_from(payer_payment_token_account_unchecked)?;
    
    msg!("4");
    
    if !service_account.is_listed {
        return err!(ErrorCode::IsNotListed)
    }
    
    update_royalty_config(&ctx, true)?;

    let (reseller_amount, provider_amount) = ServiceAgreement::try_from(service_ticket.to_account_info())?.royalties_split()?;

    token_2022::spl_token_2022::onchain::invoke_transfer_checked(
        token_program.key,
        reseller_service_ticket_token.clone().to_account_info(),
        service_ticket.clone().to_account_info(),
        payer_service_ticket_token.clone().to_account_info(),
        service_account.clone().to_account_info(), // token account delegate
        &[
            extra_account_metas_list.to_account_info(),
            mint_royalty_config.to_account_info(), 
            transfer_hook_program.to_account_info(),
            ],
        1,
        service_ticket.decimals,
        &[
             &[ b"service-account".as_ref(),
                ctx.accounts.service_ticket_mint.key().as_ref(),
                &[ctx.accounts.service_account.bump]]
            ],
    )?;

    if utils::is_native_mint(&payment_token_mint.key()) {
        utils::system_program_transfer(
            reseller_amount, 
            system_program, 
            payer, 
            &reseller_payment_token_account
        )?;
        
        utils::system_program_transfer(
            provider_amount, 
            system_program, 
            payer, 
            &provider_payment_token_account
        )?;
        
        utils::token_sync_native(&provider_payment_token_account, payment_token_program)?;
        utils::token_sync_native(&reseller_payment_token_account, payment_token_program)?;

    } else {
        utils::token_transfer_checked_transfer(
            reseller_amount, 
            payment_token_mint.decimals, 
            &payer_payment_token_account, 
            &reseller_payment_token_account, 
            &payment_token_mint, 
            &payer, 
            &payment_token_program
        )?;
        
        utils::token_transfer_checked_transfer(
            reseller_amount, 
            payment_token_mint.decimals, 
            &payer_payment_token_account, 
            &provider_payment_token_account, 
            &payment_token_mint, 
            &payer, 
            &payment_token_program
        )?;
    }

    ctx.accounts.service_account.update_holder(payer.key());
    
    update_royalty_config(&ctx, false)?;
    
    Ok(())
}

fn update_royalty_config<'info>(ctx: &Context<'_, '_, 'info, 'info, Resell<'info>>, is_selling: bool) -> Result<()> {
    let mint_royalty_config = ctx.accounts.mint_royalty_config.to_account_info();
    marketplace_transfer_controller::cpi::royalty_config_update(
        CpiContext::new_with_signer(
            ctx.accounts.transfer_hook_program.to_account_info(), 
            marketplace_transfer_controller::cpi::accounts::RoyaltyConfigUpdate {
                mint_royalty_config,
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

fn init_if_needed_token_account<'info>(ctx: &Context<'_, '_, 'info, 'info, Resell<'info>>, associated_token_account: &AccountInfo<'info>, authority:&AccountInfo<'info> ) -> Result<()> {
    utils::create_associated_token_account_idempotent(
        associated_token_account, 
        authority, 
        &ctx.remaining_accounts[1], // payment token mint
        &ctx.accounts.payer, 
        &ctx.remaining_accounts[0], // payment token program
        &ctx.accounts.system_program, 
        &ctx.accounts.associated_token_program, 
        None
    )
}