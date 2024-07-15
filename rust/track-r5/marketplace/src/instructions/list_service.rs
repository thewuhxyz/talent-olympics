use anchor_lang::prelude::*;

use crate::{
    constants::SERVICE_ACCOUNT_SEEDS,
    state::{ServiceAccount, ServiceAgreement},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::spl_token_2022::instruction::AuthorityType,
    token_interface::{Mint, Token2022, TokenAccount},
};
use utils::{
    initialize_token_group_extension, initialize_token_metadata_extension, mint_to_token_account,
    set_account_or_mint_authority, update_account_lamports_to_minimum_balance,
    update_token_metadata_extension_authority, update_token_metadata_extension_field,
};

#[derive(Accounts)]
pub struct ListService<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: can be any account
    #[account(mut)]
    pub provider: UncheckedAccount<'info>,

    #[account(
        init,
        signer,
        payer = payer,
        mint::token_program = token_program,
        mint::decimals = 0,
        mint::authority = payer,
        extensions::metadata_pointer::metadata_address = service_mint,
        extensions::group_pointer::group_address = service_mint,
        extensions::close_authority::authority = service_account,
    )]
    pub service_mint: InterfaceAccount<'info, Mint>,

    /// CHECK: token account
    #[account(
        init,
        payer = payer,
        associated_token::token_program = token_program,
        associated_token::mint = service_mint,
        associated_token::authority = payer,
    )]
    pub service_token_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: this account we use to take note of listings
    #[account(
        init,
        payer=payer,
        space=8+ServiceAccount::INIT_SPACE,
        seeds=[SERVICE_ACCOUNT_SEEDS, service_mint.key().as_ref()],
        bump
    )]
    pub service_account: Account<'info, ServiceAccount>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token2022>,
}

pub fn list_service(
    ctx: Context<ListService>,
    service_agreement_config: ServiceAgreementConfig,
) -> Result<()> {
    let service_mint = &ctx.accounts.service_mint;
    let service_account = &ctx.accounts.service_account;
    let service_token_account = &ctx.accounts.service_token_account;
    let provider = &ctx.accounts.provider;
    let payer = &ctx.accounts.payer;
    let system_program = &ctx.accounts.system_program;
    let token_program = &ctx.accounts.token_program;

    initialize_token_group_extension(
        u32::MAX,
        service_mint,
        service_mint,
        payer,
        Some(service_account.key()),
        token_program,
        None,
    )?;

    let service_agreement = ServiceAgreement {
        provider: provider.key(),
        receiver: Pubkey::default(),
        description: service_agreement_config.description,
        fee_basis_points: service_agreement_config.fee_basis_points,
        maximum_fee: service_agreement_config.maximum_fee,
        name: service_agreement_config.name,
        price: service_agreement_config.price,
        symbol: service_agreement_config.symbol,
        transferable: service_agreement_config.transferable,
        uri: service_agreement_config.uri,
    };

    initialize_token_metadata_extension(
        service_mint,
        service_mint,
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
            service_mint,
            payer,
            token_program,
            key.clone(),
            value.clone(),
        )?
    }

    update_token_metadata_extension_authority(service_mint, payer, None, token_program)?;

    update_account_lamports_to_minimum_balance(
        service_mint.to_account_info(),
        payer.to_account_info(),
        system_program.to_account_info(),
    )?;

    mint_to_token_account(1, service_mint, payer, service_token_account, token_program)?;

    set_account_or_mint_authority(
        service_mint,
        payer,
        None,
        AuthorityType::MintTokens,
        token_program,
    )?;

    ctx.accounts.service_account.init(
        provider.key(),
        service_mint.key(),
        ctx.bumps.service_account,
    )?;
    Ok(())
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct ServiceAgreementConfig {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub description: String,
    pub price: u64,
    pub fee_basis_points: u16,
    pub maximum_fee: u64,
    pub transferable: bool,
}
