use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        token_group_initialize, token_metadata_initialize, Mint, Token2022, TokenAccount,
        TokenGroupInitialize, TokenMetadataInitialize,
    },
};

use crate::{
    constants::SERVICE_ACCOUNT_SEEDS, state::ServiceAccount,
    utils::update_account_lamports_to_minimum_balance,
};

#[derive(Accounts)]
#[instruction(args: ListServiceArgs)]
pub struct ListService<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: can be any account
    #[account(mut)]
    pub authority: Signer<'info>,

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
        extensions::group_pointer::authority = authority,
        extensions::group_pointer::group_address = service,
        extensions::close_authority::authority = authority,
        extensions::permanent_delegate::delegate = authority,
        // extensions::non_transferable::address = authority,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    /// CHECK: can be any account. This is the group/collection account
    #[account(mut)]
    pub service: UncheckedAccount<'info>,

    /// CHECK: this account we use to take note of listings
    #[account(
        init,
        payer=payer,
        space=8+ServiceAccount::INIT_SPACE,
        seeds=[SERVICE_ACCOUNT_SEEDS, mint.key().as_ref()],
        bump
    )]
    pub service_account: Box<Account<'info, ServiceAccount>>,

    // /// CHECK: can be any account
    // #[account(mut)]
    // pub service_update_authority: Option<UncheckedAccount<'info>>,
    #[account(
        init,
        payer = payer,
        associated_token::token_program = token_program,
        associated_token::mint = mint,
        associated_token::authority = authority, // todo: change this to service account
    )]
    pub mint_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token2022>,
}

impl<'info> ListService<'info> {
    fn initialize_token_metadata(&self, name: String, symbol: String, uri: String) -> Result<()> {
        let cpi_accounts = TokenMetadataInitialize {
            token_program_id: self.token_program.to_account_info(),
            mint: self.mint.to_account_info(),
            metadata: self.mint.to_account_info(),
            mint_authority: self.authority.to_account_info(),
            update_authority: self.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        token_metadata_initialize(cpi_ctx, name, symbol, uri)?;
        Ok(())
    }

    fn initialize_token_group(&self) -> Result<()> {
        let cpi_accounts = TokenGroupInitialize {
            group: self.service.to_account_info(),
            token_program_id: self.token_program.to_account_info(),
            mint: self.mint.to_account_info(),
            mint_authority: self.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        // let update_authority = match &self.service_update_authority {
        //     Some(authority) => Some(authority.key()),
        //     None => None,
        // };
        token_group_initialize(cpi_ctx, Some(self.authority.key()), u32::MAX)
    }
}

pub fn list_service(ctx: Context<ListService>, args: ListServiceArgs) -> Result<()> {
    ctx.accounts.initialize_token_metadata(
        args.name.clone(),
        args.symbol.clone(),
        args.uri.clone(),
    )?;

    ctx.accounts.mint.reload()?;

    ctx.accounts.initialize_token_group()?;

    // non_transferable_mint_initialize(ctx)

    update_account_lamports_to_minimum_balance(
        ctx.accounts.service.to_account_info(),
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    )?;

    update_account_lamports_to_minimum_balance(
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    )?;

    Ok(())
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct ListServiceArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub max_size: u32,
}
