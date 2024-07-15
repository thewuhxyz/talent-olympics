use utils;

use anchor_lang::prelude::*;

// use anchor_spl::token_interface::TokenAccount;
use spl_tlv_account_resolution::{account::ExtraAccountMeta, seeds::Seed, state::ExtraAccountMetaList};
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

#[derive(Accounts)]
pub struct RoyaltyInitExtraMetas<'info> {
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
    #[account()]
    pub wsol_mint: UncheckedAccount<'info>,
    
    /// CHECK: Program ID
    #[account(address = crate::ID)]
    pub transfer_hook_program_id: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,

    /// CHECK: ...
    pub service_account: AccountInfo<'info>,

    /// CHECK: ...
    pub provider_wsol_token_account: UncheckedAccount<'info>,

    /// CHECK: ...
    pub token_program_classic: UncheckedAccount<'info>,
    /// CHECK: ...
    pub associated_token_program: UncheckedAccount<'info>,
}

pub fn royalty_init_extra_metas(ctx: Context<RoyaltyInitExtraMetas>) -> Result<()> {
    let service_ticket_mint = &mut ctx.accounts.service_ticket_mint;
    let extra_account_metas_list = &mut ctx.accounts.extra_account_metas_list;
    let system_program = &ctx.accounts.system_program;
    let transfer_hook_program_id = &ctx.accounts.transfer_hook_program_id;
    let payer = &ctx.accounts.payer;
    // let provider = &ctx.accounts.provider;
    let wsol_mint = &ctx.accounts.wsol_mint;
    let token_program = &ctx.accounts.token_program_classic;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let provider_wsol_token_account = &ctx.accounts.provider_wsol_token_account;
    // let mint_royalty_config = &mut ctx.accounts.mint_royalty_config;
    

    // initialize_extra_account_meta_list(&ctx);
    initialize_extra_account_meta_list(
        &extra_account_metas_list,
        &transfer_hook_program_id,
        &payer,
        &service_ticket_mint,
        &wsol_mint,
        &provider_wsol_token_account,
        &system_program,
        &token_program,
        &associated_token_program,
    )?;

    Ok(())
}

fn initialize_extra_account_meta_list<'info>(
    extra_account_metas_list: &AccountInfo<'info>,
    transfer_hook_program: &AccountInfo<'info>,
    payer: &AccountInfo<'info>,
    service_ticket_mint: &AccountInfo<'info>,
    wsol_mint: &AccountInfo<'info>,
    provider_wsol_token_account: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    associated_token_program: &AccountInfo<'info>,
) -> Result<()> {
    let account_metas = vec![
            // 5 - mint royalty
            ExtraAccountMeta::new_with_seeds(
                    &[
                        Seed::AccountKey { index: 1 }, // service ticket (mint) index
                    ], false, false
                )?,
            
            // 6 - provider wsol account
            ExtraAccountMeta::new_with_pubkey(&provider_wsol_token_account.key(), false, true)?,

            // 7 - reseller wsol account
            ExtraAccountMeta::new_external_pda_with_seeds(
                11, // associated token program index
                &[
                    Seed::AccountKey { index: 3 }, // reseller index (owner from transfering mint token)
                    Seed::AccountKey { index: 10 }, // token program index
                    Seed::AccountKey { index: 9 }, // wsol mint index
                ],
                false, // is_signer
                true,  // is_writable
            )?,
            
            // 8 - mint royalty wsol
            ExtraAccountMeta::new_external_pda_with_seeds(
                11, // associated token program index
                &[
                    Seed::AccountKey { index: 5 }, // mint royalty index (should hold tokens transferred from buyer)
                    Seed::AccountKey { index: 10 }, // token program index
                    Seed::AccountKey { index: 9 }, // wsol mint index
                ],
                false, // is_signer
                true,  // is_writable
            )?,

            // 9 -  wsol mint
            ExtraAccountMeta::new_with_pubkey(&wsol_mint.key(), false, false)?,
            
            // 10 - token program
            ExtraAccountMeta::new_with_pubkey(&token_program.key(), false, false)?,
            
            // 11 -, associated token program
            ExtraAccountMeta::new_with_pubkey(
                &associated_token_program.key(),
                false,
                false,
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
