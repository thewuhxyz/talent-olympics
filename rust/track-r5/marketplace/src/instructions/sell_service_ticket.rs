use crate::{
    state::ServiceAccount,
    utils::{get_mint_extensible_extension_data, update_account_lamports_to_minimum_balance},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::Metadata,
    token_2022::spl_token_2022::{
        instruction::initialize_non_transferable_mint,
    },
    token_interface::{
        spl_token_metadata_interface::state::TokenMetadata, token_member_initialize,
        token_metadata_initialize, Mint, Token2022, TokenAccount, TokenMemberInitialize,
        TokenMetadataInitialize, transfer_checked_with_fee, non_transferable_mint_initialize,
    },
};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct BuyserviceArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(Accounts)]
#[instruction(args: BuyserviceArgs)]
pub struct SellServiceTicket<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: authority of the service aka service owner
    #[account(mut)]
    pub authority: UncheckedAccount<'info>,

    /// CHECK: receiver of the service nft
    #[account()]
    pub receiver: UncheckedAccount<'info>,

    #[account(
        init,
        signer,
        payer = payer,
        mint::token_program = token_program,
        mint::decimals = 0,
        mint::authority = authority,
        mint::freeze_authority = authority,
        extensions::metadata_pointer::authority = authority,
        extensions::metadata_pointer::metadata_address = mint,
        extensions::group_member_pointer::authority = authority,
        extensions::group_member_pointer::member_address = member,
        extensions::close_authority::authority = authority,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program = token_program,
        mint::decimals = 0,
        mint::authority = authority,
        mint::freeze_authority = authority,
        extensions::metadata_pointer::authority = authority,
        extensions::metadata_pointer::metadata_address = service_ticket,
        extensions::group_pointer::authority = authority,
        extensions::group_pointer::group_address = group,
        extensions::close_authority::authority = authority,
    )]
    pub service_ticket: InterfaceAccount<'info, Mint>,

    /// CHECK: can be any account
    #[account(mut)]
    pub group: UncheckedAccount<'info>,

    /// CHECK: can be any account
    #[account(mut)]
    pub member: UncheckedAccount<'info>,

    #[account(mut)]
    pub service_account: Box<Account<'info, ServiceAccount>>,

    // /// CHECK: can be any account
    // #[account(mut)]
    // pub group_update_authority: UncheckedAccount<'info>,
    #[account(
        init,
        payer = payer,
        associated_token::token_program = token_program,
        associated_token::mint = mint,
        associated_token::authority = receiver,
    )]
    pub mint_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token2022>,
}

impl<'info> SellServiceTicket<'info> {}

pub fn sell_service_ticket(ctx: Context<SellServiceTicket>) -> Result<()> {
    Ok(())
}
