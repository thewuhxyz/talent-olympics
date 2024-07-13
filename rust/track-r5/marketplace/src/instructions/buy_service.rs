use crate::{
    constants::SERVICE_ACCOUNT_SEEDS, error::ErrorCode, state::{ServiceAccount, ServiceAgreement}, utils::{
        create_initialized_mint, create_uninitialized_mint_account,
        initialize_group_member_pointer_extension, initialize_non_transferrable_extension,
        initialize_token_group_member_extension, initialize_token_metadata_extension,
        initialize_token_metadata_pointer_extension, initialize_transfer_fee_extension,
        mint_to_token_account, set_account_or_mint_authority, system_program_transfer,
        update_account_lamports_to_minimum_balance, update_token_metadata_extension_authority,
        update_token_metadata_extension_field,
    }
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::spl_token_2022::{extension::ExtensionType, instruction::AuthorityType},
    token_interface::{Mint, Token2022, TokenAccount},
};

#[derive(Accounts)]
pub struct BuyService<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: receiver of the service nft
    #[account()]
    pub receiver: UncheckedAccount<'info>,

    /// CHECK: provider of the service
    #[account(mut)]
    pub provider: UncheckedAccount<'info>,

    /// CHECK: mint account, yet to be initialized
    #[account(mut, signer)]
    pub service_ticket_mint: UncheckedAccount<'info>,

    #[account(
        mint::token_program = token_program,
        mint::decimals = 0,
        mint::authority = service_account.provider,
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
        payer = payer,
        associated_token::token_program = token_program,
        associated_token::mint = service_ticket_mint,
        associated_token::authority = receiver,
    )]
    pub service_ticket_token_account: InterfaceAccount<'info, TokenAccount>,
    
    pub system_program: Program<'info, System>,
    
    pub associated_token_program: Program<'info, AssociatedToken>,
    
    pub token_program: Program<'info, Token2022>,
}

pub fn buy_service(ctx: Context<BuyService>) -> Result<()> {
    let service_ticket = &ctx.accounts.service_ticket_mint;
    let token_program = &ctx.accounts.token_program;
    let system_program = &ctx.accounts.system_program;
    let payer = &mut ctx.accounts.payer;
    let service_account = &mut ctx.accounts.service_account;
    let service = &ctx.accounts.service_mint;
    // let rent = &ctx.accounts.rent;
    let token_account = &mut ctx.accounts.service_ticket_token_account;
    let provider = &mut ctx.accounts.provider;
    let receiver = &ctx.accounts.receiver;

    let (service_account_key, _) = Pubkey::find_program_address(&[SERVICE_ACCOUNT_SEEDS.as_ref(), service.key().as_ref()], &crate::ID);
    
    require_eq!(service_account.key(), service_account_key, ErrorCode::ServiceAccountMismatch);

    let service_agreement = ServiceAgreement::try_from(service.to_account_info())?;

    create_uninitialized_mint_account(&[ExtensionType::GroupPointer, ExtensionType::MetadataPointer, ],service_ticket, payer, system_program, token_program)?;

    initialize_group_member_pointer_extension(
        service_ticket,
        Some(service_ticket.key()),
        None,
        token_program,
    )?;

    initialize_token_metadata_pointer_extension(
        service_ticket,
        Some(service_ticket.key()),
        None,
        token_program,
    )?;

    if service_agreement.transferable {
        initialize_transfer_fee_extension(
            service_agreement.fee_basis_points,
            service_agreement.maximum_fee,
            service_ticket,
            None,
            None,
            token_program,
        )?
    } else {
        initialize_non_transferrable_extension(service_ticket, token_program)?
    }
    
    initialize_token_group_member_extension(
        service,
        service_account,
        service_ticket,
        service_ticket,
        payer,
        token_program,
        Some(&[SERVICE_ACCOUNT_SEEDS.as_ref(), service.key().as_ref(), &[service_account.bump]])
    )?;

    create_initialized_mint(0, service_ticket, payer, None, token_program)?;

    initialize_token_metadata_extension(
        service_ticket,
        service_ticket,
        payer,
        payer,
        token_program,
        service_agreement.name.clone(),
        service_agreement.symbol.clone(),
        service_agreement.uri.clone(),
    )?;

    for i in 0..service_agreement.to_additional_metadata().len() {
        let (key, value) = &service_agreement.to_additional_metadata()[i];

        update_token_metadata_extension_field(
            service_ticket,
            service_account,
            token_program,
            key.clone(),
            if key == ServiceAgreement::RECEIVER_KEY {
                receiver.key().to_string()
            } else {
                value.clone()
            },
        )?
    }

    update_token_metadata_extension_authority(
        service_ticket,
        service_account,
        None,
        token_program,
    )?;

    update_account_lamports_to_minimum_balance(
        service_ticket.to_account_info(),
        payer.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    )?;

    mint_to_token_account(1,service_ticket, payer, token_account, token_program)?;

    system_program_transfer(service_agreement.price, system_program, payer, provider)?;

    set_account_or_mint_authority(
        service_ticket,
        payer,
        None,
        AuthorityType::MintTokens,
        token_program,
    )?;

    Ok(())
}
