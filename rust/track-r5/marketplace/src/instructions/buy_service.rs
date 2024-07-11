use crate::{
    error::ErrorCode,
    state::ServiceAccount,
    utils::{
        get_mint_extensible_extension_data, get_mint_space,
        update_account_lamports_to_minimum_balance,
    },
};
use anchor_lang::{
    prelude::*,
    system_program::{self, transfer, CreateAccount, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::spl_token_2022::extension::ExtensionType,
    token_interface::{
        group_member_pointer_initialize, initialize_mint, metadata_pointer_initialize, mint_to,
        non_transferable_mint_initialize,
        spl_token_metadata_interface::state::{Field, TokenMetadata},
        token_member_initialize, token_metadata_initialize, token_metadata_update_field,
        transfer_fee_initialize, GroupMemberPointerInitialize, InitializeMint,
        MetadataPointerInitialize, Mint, MintTo, NonTransferableMintInitialize, Token2022,
        TokenAccount, TokenMemberInitialize, TokenMetadataInitialize, TokenMetadataUpdateField,
        TransferFeeInitialize,
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
pub struct BuyService<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: authority of the service aka service owner
    #[account(mut)]
    pub authority: UncheckedAccount<'info>,

    /// CHECK: receiver of the service nft
    #[account()]
    pub receiver: UncheckedAccount<'info>,

    /// CHECK: mint account, yet to be initialized
    #[account(mut)]
    pub mint: Signer<'info>,

    #[account(
        mint::token_program = token_program,
        mint::decimals = 0,
        mint::authority = authority,
        mint::freeze_authority = authority,
        extensions::metadata_pointer::authority = authority,
        extensions::metadata_pointer::metadata_address = service,
        extensions::group_pointer::authority = authority,
        extensions::group_pointer::group_address = group,
        extensions::close_authority::authority = authority,
    )]
    pub service: InterfaceAccount<'info, Mint>,

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

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token2022>,
}

impl<'info> BuyService<'info> {
    fn create_mint_account(&self) -> Result<()> {
        let cpi_accounts = CreateAccount {
            from: self.payer.to_account_info(),
            to: self.mint.to_account_info(),
        };
        let cpi_program = self.system_program.to_account_info();
        let ctx = CpiContext::new(cpi_program, cpi_accounts);

        let (space, lamports) = get_mint_space(&[ExtensionType::Uninitialized])?;

        // get_mint
        system_program::create_account(ctx, lamports, space, &self.token_program.key())
    }

    fn initialize_non_transferrable(&self) -> Result<()> {
        let cpi_accounts = NonTransferableMintInitialize {
            mint: self.mint.to_account_info(),
            token_program_id: self.token_program.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let ctx = CpiContext::new(cpi_program, cpi_accounts);
        non_transferable_mint_initialize(ctx)
    }

    fn initialize_transfer_fee(&self) -> Result<()> {
        let cpi_accounts = TransferFeeInitialize {
            mint: self.mint.to_account_info(),
            token_program_id: self.token_program.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer_fee_initialize(ctx, None, None, 1000, 1000) // todo: Update this asap
    }

    fn initialize_token_metadata_pointer(&self) -> Result<()> {
        let cpi_accounts = MetadataPointerInitialize {
            mint: self.mint.to_account_info(),
            token_program_id: self.token_program.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let ctx = CpiContext::new(cpi_program, cpi_accounts);
        metadata_pointer_initialize(ctx, Some(self.authority.key()), Some(self.mint.key()))
    }

    fn initialize_group_member_pointer(&self) -> Result<()> {
        let cpi_accounts = GroupMemberPointerInitialize {
            mint: self.mint.to_account_info(),
            token_program_id: self.token_program.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let ctx = CpiContext::new(cpi_program, cpi_accounts);
        group_member_pointer_initialize(ctx, Some(self.authority.key()), Some(self.mint.key()))
    }

    fn to_initialize_mint(&self) -> Result<()> {
        let cpi_accounts = InitializeMint {
            mint: self.mint.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let ctx = CpiContext::new(cpi_program, cpi_accounts);
        initialize_mint(ctx, 0, &self.authority.key, None)
    }

    fn initialize_token_metadata(&self) -> Result<()> {
        let mint_data = &mut self.service.to_account_info();

        let service_data = get_mint_extensible_extension_data::<TokenMetadata>(mint_data)?;

        let cpi_accounts = TokenMetadataInitialize {
            token_program_id: self.token_program.to_account_info(),
            mint: self.mint.to_account_info(),
            metadata: self.mint.to_account_info(),
            mint_authority: self.authority.to_account_info(),
            update_authority: self.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        token_metadata_initialize(
            cpi_ctx,
            service_data.name,
            service_data.symbol,
            service_data.uri,
        )?;

        let additional_metadata = service_data.additional_metadata.clone();

        for x in 0..additional_metadata.len() {
            let field = &additional_metadata[x];
            let cpi_accounts_update = TokenMetadataUpdateField {
                metadata: self.mint.to_account_info(),
                token_program_id: self.token_program.to_account_info(),
                update_authority: self.authority.to_account_info(),
            };
            let cpi_ctx_update =
                CpiContext::new(self.token_program.to_account_info(), cpi_accounts_update);
            token_metadata_update_field(
                cpi_ctx_update,
                Field::Key(field.0.clone()),
                field.1.clone(),
            )?;
        }

        Ok(())
    }

    fn initialize_token_group_member(&self) -> Result<()> {
        let cpi_accounts = TokenMemberInitialize {
            group: self.group.to_account_info(),
            group_update_authority: self.authority.to_account_info(),
            member: self.member.to_account_info(),
            member_mint: self.mint.to_account_info(),
            member_mint_authority: self.authority.to_account_info(),
            token_program_id: self.token_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        token_member_initialize(cpi_ctx)
    }

    fn mint_to_token_account(&self) -> Result<()> {
        let cpi_accounts = MintTo {
            authority: self.authority.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.mint_token_account.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        mint_to(cpi_ctx, 1)
    }

    fn transfer_for_service(&self) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.payer.to_account_info(),
            to: self.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), cpi_accounts);
        transfer(cpi_ctx, 1)
    }
}

pub fn buy_service(ctx: Context<BuyService>) -> Result<()> {
    ctx.accounts.create_mint_account()?;

    let mint_data = &mut ctx.accounts.service.to_account_info();
    let service_data = get_mint_extensible_extension_data::<TokenMetadata>(mint_data)?;

    ctx.accounts.initialize_group_member_pointer()?;
    ctx.accounts.initialize_token_metadata_pointer()?;

    let (_, transferable) = &service_data.additional_metadata[1];

    let is_ticket_transferable = match transferable.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(error!(ErrorCode::NotAdmin)),
    }?;

    if is_ticket_transferable {
        ctx.accounts.initialize_transfer_fee()?
    } else {
        ctx.accounts.initialize_non_transferrable()?
    }

    ctx.accounts.to_initialize_mint()?;

    ctx.accounts.initialize_token_metadata()?;
    ctx.accounts.initialize_token_group_member()?;

    update_account_lamports_to_minimum_balance(
        ctx.accounts.member.to_account_info(),
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    )?;

    update_account_lamports_to_minimum_balance(
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    )?;

    ctx.accounts.mint_to_token_account()?;
    ctx.accounts.transfer_for_service()?;

    Ok(())
}
