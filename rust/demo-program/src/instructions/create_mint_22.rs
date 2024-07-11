// use anchor_lang::prelude::*;

// use anchor_spl::{
//     associated_token::AssociatedToken,
//     token_interface::{
//         token_metadata_initialize, Mint, Token2022, TokenAccount, TokenMetadataInitialize,
//     },
// };

// use crate::utils::{
//     get_meta_list_size, update_account_lamports_to_minimum_balance, META_LIST_ACCOUNT_SEED,
// };

// #[derive(AnchorDeserialize, AnchorSerialize)]
// pub struct CreateMintAccountArgs {
//     pub name: String,
//     pub symbol: String,
//     pub uri: String,
// }

// #[derive(Accounts)]
// #[instruction(args: CreateMintAccountArgs)]
// pub struct CreateMintAccount<'info> {
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     #[account(mut)]
//     /// CHECK: can be any account
//     pub authority: Signer<'info>,
//     #[account()]
//     /// CHECK: can be any account
//     pub receiver: UncheckedAccount<'info>,
//     #[account(
//         init,
//         signer,
//         payer = payer,
//         mint::token_program = token_program,
//         mint::decimals = 0,
//         mint::authority = authority,
//         mint::freeze_authority = authority,
//         extensions::metadata_pointer::authority = authority,
//         extensions::metadata_pointer::metadata_address = mint,
//         extensions::group_member_pointer::authority = authority,
//         extensions::group_member_pointer::member_address = mint,
//         extensions::transfer_hook::authority = authority,
//         extensions::transfer_hook::program_id = crate::ID,
//         extensions::close_authority::authority = authority,
//         extensions::permanent_delegate::delegate = authority,
//     )]
//     pub mint: Box<InterfaceAccount<'info, Mint>>,
//     #[account(
//         init,
//         payer = payer,
//         associated_token::token_program = token_program,
//         associated_token::mint = mint,
//         associated_token::authority = receiver,
//     )]
//     pub mint_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
//     /// CHECK: This account's data is a buffer of TLV data
//     #[account(
//         init,
//         space = get_meta_list_size(None),
//         seeds = [META_LIST_ACCOUNT_SEED, mint.key().as_ref()],
//         bump,
//         payer = payer,
//     )]
//     pub extra_metas_account: UncheckedAccount<'info>,
//     pub system_program: Program<'info, System>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub token_program: Program<'info, Token2022>,
// }

// impl<'info> CreateMintAccount<'info> {
//     fn initialize_token_metadata(&self, name: String, symbol: String, uri: String) -> Result<()> {
//         let cpi_accounts = TokenMetadataInitialize {
//             token_program_id: self.token_program.to_account_info(),
//             mint: self.mint.to_account_info(),
//             metadata: self.mint.to_account_info(), // metadata account is the mint, since data is stored in mint
//             mint_authority: self.authority.to_account_info(),
//             update_authority: self.authority.to_account_info(),
//         };
//         let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
//         token_metadata_initialize(cpi_ctx, name, symbol, uri)?;
//         Ok(())
//     }
// }

// pub fn handlerz(ctx: Context<CreateMintAccount>, args: CreateMintAccountArgs) -> Result<()> {
//     ctx.accounts.initialize_token_metadata(
//         args.name.clone(),
//         args.symbol.clone(),
//         args.uri.clone(),
//     )?;

//     ctx.accounts.mint.reload()?;

//     // ctx.accounts.mint_token_account.state

//     update_account_lamports_to_minimum_balance(
//         ctx.accounts.mint.to_account_info(),
//         ctx.accounts.payer.to_account_info(),
//         ctx.accounts.system_program.to_account_info(),
//     )?;

//     Ok(())
// }
