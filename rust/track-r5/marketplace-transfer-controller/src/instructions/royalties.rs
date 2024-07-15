use anchor_lang::prelude::*;
use anchor_spl::{token_2022::spl_token_2022::{extension::{transfer_hook::TransferHookAccount, BaseStateWithExtensionsMut, PodStateWithExtensionsMut}, pod::PodAccount}, token_interface::Mint};

use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct Royalties<'info> {
    /// CHECK: Perform no checks
    #[account()]
    pub source_token_account: UncheckedAccount<'info>,

    #[account()]
    pub service_ticket_mint: InterfaceAccount<'info, Mint>,

    /// CHECK: Perform no checks
    #[account()]
    pub receiver_token_account: UncheckedAccount<'info>,

    /// CHECK: reseller of the service nft
    #[account()]
    pub reseller: UncheckedAccount<'info>,

    /// CHECK: mint account, yet to be initialized
    #[account(
        // seeds = [b"extra-account-metas", service_ticket_mint.key().as_ref()],
        // bump 
    )]
    pub extra_account_metas_list: UncheckedAccount<'info>,

    /// CHECK: ...
    #[account()]
    pub mint_royalty_config: UncheckedAccount<'info>,

    /// CHECK: mint account, yet to be initialized
    #[account(mut)]
    pub provider_wsol_token_account: UncheckedAccount<'info>,

    /// CHECK: mint account, yet to be initialized
    #[account(mut)]
    pub reseller_wsol_token_account: UncheckedAccount<'info>,
    /// CHECK: mint account, yet to be initialized
    #[account(mut)]
    pub mint_royalty_wsol_token_account: UncheckedAccount<'info>,
    /// CHECK: mint account, yet to be initialized
    pub wsol_mint: UncheckedAccount<'info>,

    /// CHECK: mint account, yet to be initialized
    pub token_program: UncheckedAccount<'info>,
    /// CHECK: mint account, yet to be initialized
    pub associated_token_program: UncheckedAccount<'info>,
}

pub fn royalties(ctx: Context<Royalties>, _amount: u64) -> Result<()> {
    msg!("in tx hook!");

    assert_is_transferring(&ctx)?;

    Ok(())
}

fn assert_is_transferring(ctx: &Context<Royalties>) -> Result<()> {
    let source_token_info = ctx.accounts.source_token_account.to_account_info();
    let mut account_data_ref = source_token_info.try_borrow_mut_data()?;
    let mut account = PodStateWithExtensionsMut::<PodAccount>::unpack(*account_data_ref)?;
    let account_extension = account.get_extension_mut::<TransferHookAccount>()?;

    if !bool::from(account_extension.transferring) {
        return err!(ErrorCode::IsNotCurrentlyTransferring);
    }

    Ok(())
}
