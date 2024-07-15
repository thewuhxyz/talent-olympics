use utils;

use anchor_lang::prelude::*;

use spl_tlv_account_resolution::{account::ExtraAccountMeta, seeds::Seed, state::ExtraAccountMetaList};
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

#[derive(Accounts)]
pub struct TransferControlInit<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: mint account, yet to be initialized
    #[account(mut, signer)]
    pub service_ticket_mint: UncheckedAccount<'info>,

    /// CHECK: mint account, yet to be initialized
    #[account(
        mut,
        seeds = [b"extra-account-metas", service_ticket_mint.key().as_ref()],
        bump 
    )]
    pub extra_account_metas_list: UncheckedAccount<'info>,
    
    /// CHECK: Program ID
    #[account(address = crate::ID)]
    pub transfer_hook_program_id: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn transfer_control_init(ctx: Context<TransferControlInit>) -> Result<()> {
    let service_ticket_mint = &mut ctx.accounts.service_ticket_mint;
    let extra_account_metas_list = &mut ctx.accounts.extra_account_metas_list;
    let system_program = &ctx.accounts.system_program;
    let transfer_hook_program = &ctx.accounts.transfer_hook_program_id;
    let payer = &ctx.accounts.payer;
    
    initialize_extra_account_meta_list(
        &extra_account_metas_list,
        &transfer_hook_program,
        &payer,
        &service_ticket_mint,
        &system_program,
    )?;

    Ok(())
}

fn initialize_extra_account_meta_list<'info>(
    extra_account_metas_list: &AccountInfo<'info>,
    transfer_hook_program: &AccountInfo<'info>,
    payer: &AccountInfo<'info>,
    service_ticket_mint: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
) -> Result<()> {
    let account_metas = vec![
            // 5 - mint royalty config
            ExtraAccountMeta::new_with_seeds(
                    &[
                        Seed::AccountKey { index: 1 }, // service ticket (mint) index
                    ], false, false
                )?,
        ];
    
    utils::create_uninitialized_extra_account_metas_account(&account_metas, extra_account_metas_list, service_ticket_mint, payer, transfer_hook_program, system_program)?;

    // initialize ExtraAccountMetaList account with extra accounts
    ExtraAccountMetaList::init::<ExecuteInstruction>(
        &mut extra_account_metas_list.try_borrow_mut_data()?,
        &account_metas,
    )?;

    Ok(())
}
