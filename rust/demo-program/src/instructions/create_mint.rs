// use anchor_lang::prelude::*;
// use anchor_spl::{
//     associated_token::AssociatedToken,
//     metadata::{
//         create_master_edition_v3, create_metadata_accounts_v3,
//         mpl_token_metadata::{
//             accounts::{MasterEdition, Metadata},
//             types::DataV2,
//         },
//         CreateMasterEditionV3, CreateMetadataAccountsV3, Metadata as TokenMetadata,
//     },
//     token::{mint_to, Mint, MintTo, Token, TokenAccount},
//     token_interface::MetadataPointerInitialize,
// };

// #[derive(Accounts)]
// pub struct InitNFT<'info> {
//     #[account(mut)]
//     pub signer: Signer<'info>,

//     #[account(
//         init,
//         payer = signer,
//         mint::decimals = 0,
//         mint::authority = signer.key(),
//         mint::freeze_authority = signer.key(),
//     )]
//     pub mint: Account<'info, Mint>,

//     #[account(
//         init_if_needed,
//         payer = signer,
//         associated_token::mint = mint,
//         associated_token::authority = signer
//     )]
//     pub associated_token_account: Account<'info, TokenAccount>,

//     /// CHECK - address
//     #[account(
//         mut,
//         address=Metadata::find_pda(&mint.key()).0,
//     )]
//     pub metadata_account: UncheckedAccount<'info>,

//     /// CHECK: address
//     #[account(
//         mut,
//         address=MasterEdition::find_pda(&mint.key()).0,
//     )]
//     pub master_edition_account: UncheckedAccount<'info>,

//     pub token_program: Program<'info, Token>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub token_metadata_program: Program<'info, TokenMetadata>,
//     pub system_program: Program<'info, System>,
//     pub rent: Sysvar<'info, Rent>,
// }

// impl<'info> InitNFT<'info> {
//     pub fn initialize_pointer(
//         &self,
//     ) -> CpiContext<'_, '_, '_, 'info, MetadataPointerInitialize<'info>> {
//         let accounts = MetadataPointerInitialize {
//             token_program_id: self.token_program.to_account_info(),
//             mint: self.mint.to_account_info(),
//         };

//         CpiContext::new(self.token_program.to_account_info(), accounts)
//     }
// }

// pub fn init_nft(ctx: Context<InitNFT>, name: String, symbol: String, uri: String) -> Result<()> {
//     // create mint account
//     let cpi_context = CpiContext::new(
//         ctx.accounts.token_program.to_account_info(),
//         MintTo {
//             mint: ctx.accounts.mint.to_account_info(),
//             to: ctx.accounts.associated_token_account.to_account_info(),
//             authority: ctx.accounts.signer.to_account_info(),
//         },
//     );

//     mint_to(cpi_context, 1)?;

//     // create metadata account
//     let cpi_context = CpiContext::new(
//         ctx.accounts.token_metadata_program.to_account_info(),
//         CreateMetadataAccountsV3 {
//             metadata: ctx.accounts.metadata_account.to_account_info(),
//             mint: ctx.accounts.mint.to_account_info(),
//             mint_authority: ctx.accounts.signer.to_account_info(),
//             update_authority: ctx.accounts.signer.to_account_info(),
//             payer: ctx.accounts.signer.to_account_info(),
//             system_program: ctx.accounts.system_program.to_account_info(),
//             rent: ctx.accounts.rent.to_account_info(),
//         },
//     );

//     let data_v2 = DataV2 {
//         name,
//         symbol,
//         uri,
//         seller_fee_basis_points: 0,
//         creators: None,
//         collection: None,
//         uses: None,
//     };

//     create_metadata_accounts_v3(cpi_context, data_v2, false, true, None)?;

//     //create master edition account
//     let cpi_context = CpiContext::new(
//         ctx.accounts.token_metadata_program.to_account_info(),
//         CreateMasterEditionV3 {
//             edition: ctx.accounts.master_edition_account.to_account_info(),
//             mint: ctx.accounts.mint.to_account_info(),
//             update_authority: ctx.accounts.signer.to_account_info(),
//             mint_authority: ctx.accounts.signer.to_account_info(),
//             payer: ctx.accounts.signer.to_account_info(),
//             metadata: ctx.accounts.metadata_account.to_account_info(),
//             token_program: ctx.accounts.token_program.to_account_info(),
//             system_program: ctx.accounts.system_program.to_account_info(),
//             rent: ctx.accounts.rent.to_account_info(),
//         },
//     );

//     create_master_edition_v3(cpi_context, None)?;

//     Ok(())
// }
