use crate::{
    constants::SERVICE_ACCOUNT_SEEDS, error::ErrorCode, state::{ServiceAccount, ServiceAgreement}
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::spl_token_2022::{
        extension::ExtensionType, 
        instruction::AuthorityType},
    token_interface::{Mint, Token2022},
};
use marketplace_transfer_controller::cpi as transfer_controller;
use utils;

#[derive(Accounts)]
pub struct BuyService<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: provider of the service
    #[account(mut)]
    pub provider: UncheckedAccount<'info>,

    /// CHECK: mint account, yet to be initialized
    #[account(mut, signer)]
    pub service_ticket_mint: UncheckedAccount<'info>,

    #[account(
        mut,
        extensions::metadata_pointer::metadata_address = service_mint,
        extensions::group_pointer::group_address = service_mint,
    )]
    pub service_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        has_one=provider, 
        has_one=service_mint,
        seeds=[SERVICE_ACCOUNT_SEEDS, service_mint.key().as_ref()],
        bump=service_account.bump

    )]
    pub service_account: Account<'info, ServiceAccount>,
    
    #[account(
        init,
        payer=payer,
        space=8+ServiceAccount::INIT_SPACE,
        seeds=[SERVICE_ACCOUNT_SEEDS, service_ticket_mint.key().as_ref()],
        bump
    )]
    pub buyer_service_account: Account<'info, ServiceAccount>,

    /// CHECK: ...
    #[account(
        mut,
    )]
    pub service_ticket_token_account: UncheckedAccount<'info>,

    /// CHECK: mint account, yet to be initialized
    #[account(
        mut,
        seeds = [b"extra-account-metas", service_ticket_mint.key().as_ref()],
        bump,
        seeds::program=transfer_hook_program_id.key()
    )]
    pub extra_account_metas_list: UncheckedAccount<'info>,

    /// CHECK: Program ID
    #[account(executable)]
    pub program_id: UncheckedAccount<'info>,

    /// CHECK: Program ID
    #[account(executable)]
    pub transfer_hook_program_id: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
    
    pub associated_token_program: Program<'info, AssociatedToken>,
    
    pub token_program: Program<'info, Token2022>,
    /// CHECK:...
    pub provider_wsol_token_account: UncheckedAccount<'info>,
    /// CHECK:...
    pub wsol_mint: UncheckedAccount<'info>,
    /// CHECK:...
    pub token_program_classic: UncheckedAccount<'info>,
}

pub fn buy_service(ctx: Context<BuyService>) -> Result<()> {
    let service_ticket_mint = &mut ctx.accounts.service_ticket_mint;
    let token_program = &ctx.accounts.token_program;
    let token_program_classic = &ctx.accounts.token_program_classic;
    let service_account = &mut ctx.accounts.service_account;
    let buyer_service_account = &mut ctx.accounts.buyer_service_account;
    let service = &ctx.accounts.service_mint;
    let token_account = &mut ctx.accounts.service_ticket_token_account;
    let provider = &mut ctx.accounts.provider;
    // let receiver = &ctx.accounts.receiver;
    let extra_account_metas_list = &mut ctx.accounts.extra_account_metas_list;
    let system_program = &ctx.accounts.system_program;
    let transfer_hook_program_id = &ctx.accounts.transfer_hook_program_id;
    let payer = &ctx.accounts.payer;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let provider_wsol_token_account = &ctx.accounts.provider_wsol_token_account;
    let wsol_mint = &ctx.accounts.wsol_mint;

    let (service_account_key, _) = Pubkey::find_program_address(&[SERVICE_ACCOUNT_SEEDS.as_ref(), service.key().as_ref()], &ctx.program_id);
    
    require_eq!(service_account.key(), service_account_key, ErrorCode::ServiceAccountMismatch);

    let service_agreement = ServiceAgreement::try_from(service.to_account_info())?;

    let extension_type = if service_agreement.transferable  {
            vec![
                ExtensionType::GroupMemberPointer, 
                ExtensionType::MetadataPointer, 
                ExtensionType::TransferHook 
            ]
            } else { 
            vec![
                ExtensionType::GroupMemberPointer, 
                ExtensionType::MetadataPointer, 
            ]
        };

    utils::create_uninitialized_mint_account(
            &extension_type,
            service_ticket_mint, 
            payer, 
            token_program, 
            system_program
        )?;

    utils::initialize_group_member_pointer_extension(
        service_ticket_mint,
        Some(service_ticket_mint.key()),
        None,
        token_program,
    )?;

    utils::initialize_token_metadata_pointer_extension(
        service_ticket_mint,
        Some(service_ticket_mint.key()),
        None,
        token_program,
    )?;

    if service_agreement.transferable {
        utils::initialize_transfer_hook_extension(None, Some(transfer_hook_program_id.key()), service_ticket_mint, token_program)?
    } else {
        utils::initialize_non_transferrable_extension(service_ticket_mint, token_program)?
    }
    
    utils::create_initialized_mint(0, service_ticket_mint, payer, None, token_program)?;

    // initialize_extra_account_meta_list(&ctx);
    initialize_royalty_config(
        extra_account_metas_list, 
        payer, 
        // mint_royalty_config, 
        // provider, 
        &buyer_service_account.to_account_info(), 
        service_ticket_mint, 
        // mint_royalty_wsol_token_account, 
        provider_wsol_token_account, 
        wsol_mint, 
        transfer_hook_program_id, 
        associated_token_program, 
        token_program_classic, 
        system_program,            
    )?;
    // initialize_extra_account_meta_list(&extra_account_metas_list, &system_program, &transfer_hook_program_id, &program_id, &payer, &service_ticket)?;
    
    utils::initialize_token_group_member_extension(
        service,
        service_account,
        service_ticket_mint,
        service_ticket_mint,
        payer,
        token_program,
        Some(&[SERVICE_ACCOUNT_SEEDS.as_ref(), service.key().as_ref(), &[service_account.bump]])
    )?;

    utils::initialize_token_metadata_extension(
        service_ticket_mint,
        service_ticket_mint,
        payer,
        payer,
        token_program,
        service_agreement.name.clone(),
        service_agreement.symbol.clone(),
        service_agreement.uri.clone(),
    )?;

    for i in 0..service_agreement.to_additional_metadata().len() {
        let (key, value) = &service_agreement.to_additional_metadata()[i];

        utils::update_token_metadata_extension_field(
            service_ticket_mint,
            payer,
            token_program,
            key.clone(),
            if key == ServiceAgreement::RECEIVER_KEY {
                payer.key().to_string()
            } else {
                value.clone()
            },
        )?
    }

    utils::update_token_metadata_extension_authority(
        service_ticket_mint,
        payer,
        None,
        token_program,
    )?;

    utils::update_account_lamports_to_minimum_balance(
        service_ticket_mint.to_account_info(),
        payer.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    )?;

    utils::create_associated_token_account_idempotent(token_account, payer, service_ticket_mint, payer, token_program, system_program, associated_token_program, None)?;

    utils::mint_to_token_account(1,service_ticket_mint, payer, token_account, token_program)?;

    utils::system_program_transfer(service_agreement.price, system_program, payer, provider)?;

    utils::set_account_or_mint_authority(
        service_ticket_mint,
        payer,
        None,
        AuthorityType::MintTokens,
        token_program,
    )?;

    buyer_service_account.init(payer.key(), service_ticket_mint.key(), ctx.bumps.buyer_service_account)?;

    Ok(())
}

fn initialize_royalty_config<'info>(
    extra_account_metas_list: &AccountInfo<'info>,
    payer: &AccountInfo<'info>,
    service_account: &AccountInfo<'info>,
    service_ticket_mint: &AccountInfo<'info>,
    provider_wsol_token_account: &AccountInfo<'info>,
    wsol_mint: &AccountInfo<'info>,
    transfer_hook_program_id: &AccountInfo<'info>,
    associated_token_program: &AccountInfo<'info>,
    token_program_classic: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    // signer_seeds: &[&[u8]]
) -> Result<()> {
    
    transfer_controller::royalty_init_extra_metas(
    CpiContext::new(
        transfer_hook_program_id.to_account_info(), 
        transfer_controller::accounts::RoyaltyInitExtraMetas{
            extra_account_metas_list: extra_account_metas_list.to_account_info(),
            payer: payer.to_account_info(),
            transfer_hook_program_id: transfer_hook_program_id.to_account_info(),
            system_program: system_program.to_account_info(),
            associated_token_program: associated_token_program.to_account_info(),
            provider_wsol_token_account: provider_wsol_token_account.to_account_info(),
            service_account: service_account.to_account_info(),
            service_ticket_mint: service_ticket_mint.to_account_info(),
            token_program_classic: token_program_classic.to_account_info(),
            wsol_mint: wsol_mint.to_account_info(),
        })
        // .with_signer(&[signer_seeds])
    )?;

    Ok(())
}
