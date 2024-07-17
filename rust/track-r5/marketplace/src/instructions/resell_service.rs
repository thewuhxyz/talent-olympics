use crate::{state::{ServiceAccount, ServiceAgreement}, error::ErrorCode};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token_2022::{self, Token2022}, token_interface::{Mint, TokenAccount as ITokenAccount, TokenInterface}
};
use marketplace_transfer_controller::state::MintRoyaltyConfig;

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
        payer=buyer,
        associated_token::token_program = token_program,
        associated_token::mint = service_ticket_mint,
        associated_token::authority = buyer,
    )]
    pub buyer_service_ticket_token: Box<InterfaceAccount<'info, ITokenAccount>>,

    #[account(
        mint::decimals = 0,
        extensions::metadata_pointer::metadata_address = service_ticket_mint,
        extensions::group_member_pointer::member_address = service_ticket_mint,
    )]
    pub service_ticket_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        constraint=service_account.holder==reseller.key(), 
        constraint=service_account.mint==service_ticket_mint.key(), 
        seeds=[service_ticket_mint.key().as_ref()],
        bump,
    )]
    pub service_account: Box<Account<'info, ServiceAccount>>,
    
    /// CHECK: service provider
    #[account(mut)]
    pub provider: UncheckedAccount<'info>,

    /// CHECK: current holder of the service ticket nft
    #[account(mut)]
    pub reseller: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    /// CHECK:...
    #[account(
        mut,
        seeds=[service_ticket_mint.key().as_ref()],
        bump,
        seeds::program=transfer_hook_program.key(),
        owner=transfer_hook_program.key()
    )]
    pub mint_royalty_config: Account<'info, MintRoyaltyConfig>,

    pub system_program: Program<'info, System>,
    
    pub associated_token_program: Program<'info, AssociatedToken>,
    
    /// CHECK: transfer_controller_program
    #[account(executable, address=marketplace_transfer_controller::ID)]
    pub transfer_hook_program: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token2022>,
}

pub fn resell<'info>(ctx: Context<'_, '_, 'info, 'info, Resell<'info>>) -> Result<()> {
    let service_ticket = &ctx.accounts.service_ticket_mint;
    let reseller_service_ticket_token = &ctx.accounts.reseller_service_ticket_token;
    let buyer_service_ticket_token = &ctx.accounts.buyer_service_ticket_token;
    let token_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let buyer = &ctx.accounts.buyer;
    let reseller = &ctx.accounts.reseller;
    let provider = &ctx.accounts.provider;
    let service_account = &ctx.accounts.service_account;

    // perfrom massive checks on all remaining accounts
    let payment_token_program = &Interface::<TokenInterface>::try_from(&ctx.remaining_accounts[0])?;
    let payment_token_mint = InterfaceAccount::<Mint>::try_from(&ctx.remaining_accounts[1])?;

    let provider_payment_token_account_unchecked = &ctx.remaining_accounts[2];
    let reseller_payment_token_account_unchecked = &ctx.remaining_accounts[3];
    let buyer_payment_token_account_unchecked = &ctx.remaining_accounts[4];
    
    let extra_account_metas_list = &ctx.remaining_accounts[5];
    let mint_royalty_config = &ctx.accounts.mint_royalty_config;
    let transfer_hook_program = &ctx.accounts.transfer_hook_program;

    init_if_needed_token_account(&ctx, provider_payment_token_account_unchecked, provider)?;
    init_if_needed_token_account(&ctx, reseller_payment_token_account_unchecked, reseller)?;
    init_if_needed_token_account(&ctx, buyer_payment_token_account_unchecked, buyer)?;
    
    // todo: perform massive checks
    let provider_payment_token_account = InterfaceAccount::<ITokenAccount>::try_from(provider_payment_token_account_unchecked)?;
    let reseller_payment_token_account = InterfaceAccount::<ITokenAccount>::try_from(reseller_payment_token_account_unchecked)?;
    let buyer_payment_token_account = InterfaceAccount::<ITokenAccount>::try_from(buyer_payment_token_account_unchecked)?;
        
    if !service_account.is_listed {
        return err!(ErrorCode::IsNotListed)
    }
    
    update_royalty_config(&ctx, true)?;

    let service_agreement = ServiceAgreement::try_from(service_ticket.to_account_info())?;

    require_keys_eq!(provider.key(), service_agreement.provider);

    let (reseller_amount, provider_amount) = service_agreement.royalties_split()?;

    token_2022::spl_token_2022::onchain::invoke_transfer_checked(
        token_program.key,
        reseller_service_ticket_token.clone().to_account_info(),
        service_ticket.clone().to_account_info(),
        buyer_service_ticket_token.clone().to_account_info(),
        service_account.clone().to_account_info(), // token account delegate
        &[
            extra_account_metas_list.to_account_info(),
            mint_royalty_config.to_account_info(), 
            transfer_hook_program.to_account_info(),
            ],
        1,
        service_ticket.decimals,
        &[
            &[
                ctx.accounts.service_ticket_mint.key().as_ref(),
                &[ctx.bumps.service_account]]
            ],
    )?;

    if utils::is_native_mint(&payment_token_mint.key()) {
        utils::system_program_transfer(
            reseller_amount, 
            system_program, 
            buyer, 
            &reseller_payment_token_account
        )?;
        
        utils::system_program_transfer(
            provider_amount, 
            system_program, 
            buyer, 
            &provider_payment_token_account
        )?;
        
        utils::token_sync_native(&provider_payment_token_account, payment_token_program)?;
        utils::token_sync_native(&reseller_payment_token_account, payment_token_program)?;

    } else {
        utils::token_transfer_checked_transfer(
            reseller_amount, 
            payment_token_mint.decimals, 
            &buyer_payment_token_account, 
            &reseller_payment_token_account, 
            &payment_token_mint, 
            &buyer, 
            &payment_token_program
        )?;
        
        utils::token_transfer_checked_transfer(
            reseller_amount, 
            payment_token_mint.decimals, 
            &buyer_payment_token_account, 
            &provider_payment_token_account, 
            &payment_token_mint, 
            &buyer, 
            &payment_token_program
        )?;
    }

    ctx.accounts.service_account.update_holder(buyer.key());
    
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
               &[ctx.accounts.service_ticket_mint.key().as_ref(), &[ctx.bumps.service_account]]
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
        &ctx.accounts.buyer, 
        &ctx.remaining_accounts[0], // payment token program
        &ctx.accounts.system_program, 
        &ctx.accounts.associated_token_program, 
        None
    )
}