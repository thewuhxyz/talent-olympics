use anchor_lang::prelude::*;
use anchor_spl::{token_2022::spl_token_2022::{extension::{transfer_hook::TransferHookAccount, BaseStateWithExtensionsMut, PodStateWithExtensionsMut}, pod::PodAccount}, token_interface::Mint};

use crate::{error::ErrorCode, state::MintRoyaltyConfig};

#[derive(Accounts)]
pub struct TransferControl<'info> {
    /// CHECK: Perform no checks
    #[account()]
    pub source_token_account: UncheckedAccount<'info>,

    #[account()]
    pub service_ticket_mint: InterfaceAccount<'info, Mint>,

    /// CHECK: Perform no checks
    #[account()]
    pub receiver_token_account: UncheckedAccount<'info>,

    /// CHECK: service account as the authorized delegate for the transfer
    #[account(
        // seeds=[service_ticket_mint.key().as_ref()],
        // bump,
        // seeds::program=utils::ids::MARKETPLACE_ID
    )]
    pub service_account: UncheckedAccount<'info>,

    /// CHECK: mint account, yet to be initialized
    #[account(
        seeds = [utils::META_LIST_ACCOUNT_SEED, service_ticket_mint.key().as_ref()],
        bump 
    )]
    pub extra_account_metas_list: UncheckedAccount<'info>,

    #[account(
        seeds=[service_ticket_mint.key().as_ref()],
        bump,
    )]
    pub mint_royalty_config: Account<'info, MintRoyaltyConfig>,
}

pub fn transfer_control(ctx: Context<TransferControl>, _amount: u64) -> Result<()> {
    assert_is_transferring(&ctx)?;
    
    ctx.accounts.mint_royalty_config.reload()?;
    
    let is_selling = &ctx.accounts.mint_royalty_config.is_selling;

    msg!("is selling: {is_selling}",);

    if !is_selling {return err!(ErrorCode::TransferOutsideMarketplaceNotAllowed) }

    Ok(())
}

fn assert_is_transferring(ctx: &Context<TransferControl>) -> Result<()> {
    let source_token_info = ctx.accounts.source_token_account.to_account_info();
    let mut account_data_ref = source_token_info.try_borrow_mut_data()?;
    let mut account = PodStateWithExtensionsMut::<PodAccount>::unpack(*account_data_ref)?;
    let account_extension = account.get_extension_mut::<TransferHookAccount>()?;

    if !bool::from(account_extension.transferring) {
        return err!(ErrorCode::IsNotCurrentlyTransferring);
    }

    Ok(())
}
