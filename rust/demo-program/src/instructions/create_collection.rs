// use anchor_lang::prelude::*;

// use anchor_spl::{
//     associated_token::AssociatedToken,
//     token_interface::{
//         token_group_initialize, token_metadata_initialize, Mint, Token2022, TokenAccount,
//         TokenGroupInitialize, TokenMetadataInitialize,
//     },
// };

// use crate::utils::update_account_lamports_to_minimum_balance;

// #[derive(AnchorDeserialize, AnchorSerialize)]
// pub struct CreateCollectionMintAccountArgs {
//     pub name: String,
//     pub symbol: String,
//     pub uri: String,
//     pub max_size: u32,
// }

// #[derive(Accounts)]
// #[instruction(args: CreateCollectionMintAccountArgs)]
// pub struct CreateCollection<'info> {
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
//         extensions::group_pointer::authority = authority,
//         extensions::group_pointer::group_address = group,
//         extensions::close_authority::authority = authority,
//         extensions::permanent_delegate::delegate = authority,
//     )]
//     pub mint: Box<InterfaceAccount<'info, Mint>>,

//     #[account(mut)]
//     pub group: UncheckedAccount<'info>,

//     #[account(mut)]
//     pub group_update_authority: Option<UncheckedAccount<'info>>,

//     #[account(
//         init,
//         payer = payer,
//         associated_token::token_program = token_program,
//         associated_token::mint = mint,
//         associated_token::authority = receiver,
//     )]
//     pub mint_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
//     pub system_program: Program<'info, System>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub token_program: Program<'info, Token2022>,
// }

// impl<'info> CreateCollection<'info> {
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

//     fn initialize_token_group(&self, max_size: u32) -> Result<()> {
//         let cpi_accounts = TokenGroupInitialize {
//             group: self.group.to_account_info(),
//             token_program_id: self.token_program.to_account_info(),
//             mint: self.mint.to_account_info(),
//             mint_authority: self.authority.to_account_info(),
//         };
//         let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
//         let update_authority = match &self.group_update_authority {
//             Some(authority) => Some(authority.key()),
//             None => None,
//         };
//         token_group_initialize(cpi_ctx, update_authority, max_size)
//     }
// }

// pub fn handler(ctx: Context<CreateCollection>, args: CreateCollectionMintAccountArgs) -> Result<()> {
//     ctx.accounts.initialize_token_metadata(
//         args.name.clone(),
//         args.symbol.clone(),
//         args.uri.clone(),
//     )?;

//     ctx.accounts.mint.reload()?;

//     ctx.accounts.initialize_token_group(args.max_size)?;

//     update_account_lamports_to_minimum_balance(
//         ctx.accounts.group.to_account_info(),
//         ctx.accounts.payer.to_account_info(),
//         ctx.accounts.system_program.to_account_info(),
//     )?;

//     update_account_lamports_to_minimum_balance(
//         ctx.accounts.mint.to_account_info(),
//         ctx.accounts.payer.to_account_info(),
//         ctx.accounts.system_program.to_account_info(),
//     )?;

//     Ok(())
// }
