use anchor_lang::{
    prelude::*,
    system_program::{self, CreateAccount},
};
use anchor_spl::{
    token_2022::spl_token_2022::{extension::ExtensionType, instruction::AuthorityType},
    token_interface::{
        group_member_pointer_initialize, group_pointer_initialize, initialize_mint2,
        metadata_pointer_initialize, mint_to, non_transferable_mint_initialize, set_authority,
        spl_token_metadata_interface::state::Field, token_group_initialize,
        token_member_initialize, token_metadata_initialize, token_metadata_update_authority,
        token_metadata_update_field, transfer_fee_initialize, GroupMemberPointerInitialize,
        GroupPointerInitialize, InitializeMint2, MetadataPointerInitialize, MintTo,
        NonTransferableMintInitialize, SetAuthority, TokenGroupInitialize, TokenMemberInitialize,
        TokenMetadataInitialize, TokenMetadataUpdateAuthority, TokenMetadataUpdateField,
        TransferFeeInitialize,
    },
};
use spl_pod::optional_keys::OptionalNonZeroPubkey;

use super::get_mint_space;

pub fn create_uninitialized_mint_account<
    'info,
    P: ToAccountInfo<'info>,
    A: ToAccountInfo<'info>,
    T: ToAccountInfo<'info>,
    M: ToAccountInfo<'info>,
>(
    extension_types: &[ExtensionType],
    mint: &M,
    payer: &A,
    token_program: &T,
    system_program: &P,
) -> Result<()> {
    let cpi_accounts = CreateAccount {
        from: payer.to_account_info(),
        to: mint.to_account_info(),
    };
    let cpi_program = system_program.to_account_info();
    let ctx = CpiContext::new(cpi_program, cpi_accounts);

    // get_account_data_size(token_program_id, mint_pubkey, extension_types);

    let (space, lamports) = get_mint_space(extension_types)?;

    // get_mint
    system_program::create_account(ctx, lamports, space, token_program.to_account_info().key)
}

pub fn initialize_non_transferrable_extension<
    'info,
    P: ToAccountInfo<'info>,
    A: ToAccountInfo<'info>,
>(
    mint: &A,
    token_program: &P,
) -> Result<()> {
    let cpi_accounts = NonTransferableMintInitialize {
        mint: mint.to_account_info(),
        token_program_id: token_program.to_account_info(),
    };
    let cpi_program = token_program.to_account_info();
    let ctx = CpiContext::new(cpi_program, cpi_accounts);
    non_transferable_mint_initialize(ctx)
}

pub fn initialize_transfer_fee_extension<
    'info,
    P: ToAccountInfo<'info>,
    M: ToAccountInfo<'info>,
>(
    transfer_fee_basis_points: u16,
    maximum_fee: u64,
    mint: &M,
    transfer_fee_config_authority: Option<&Pubkey>,
    withdraw_withheld_authority: Option<&Pubkey>,
    token_program: &P,
) -> Result<()> {
    let cpi_accounts = TransferFeeInitialize {
        mint: mint.to_account_info(),
        token_program_id: token_program.to_account_info(),
    };
    let cpi_program = token_program.to_account_info();
    let ctx = CpiContext::new(cpi_program, cpi_accounts);

    transfer_fee_initialize(
        ctx,
        transfer_fee_config_authority,
        withdraw_withheld_authority,
        transfer_fee_basis_points,
        maximum_fee,
    )
}

pub fn initialize_token_metadata_pointer_extension<
    'info,
    P: ToAccountInfo<'info>,
    M: ToAccountInfo<'info>,
>(
    mint: &M,
    metadata_address: Option<Pubkey>,
    metadata_authority: Option<Pubkey>,
    token_program: &P,
) -> Result<()> {
    let cpi_accounts = MetadataPointerInitialize {
        mint: mint.to_account_info(),
        token_program_id: token_program.to_account_info(),
    };
    let cpi_program = token_program.to_account_info();
    let ctx = CpiContext::new(cpi_program, cpi_accounts);
    metadata_pointer_initialize(ctx, metadata_authority, metadata_address)
}

pub fn initialize_group_member_pointer_extension<
    'info,
    P: ToAccountInfo<'info>,
    M: ToAccountInfo<'info>,
>(
    mint: &M,
    member_address: Option<Pubkey>,
    group_member_pointer_authority: Option<Pubkey>,
    token_program: &P,
) -> Result<()> {
    let cpi_accounts = GroupMemberPointerInitialize {
        mint: mint.to_account_info(),
        token_program_id: token_program.to_account_info(),
    };
    let cpi_program = token_program.to_account_info();
    let ctx = CpiContext::new(cpi_program, cpi_accounts);
    group_member_pointer_initialize(ctx, group_member_pointer_authority, member_address)
}

pub fn initialize_group_pointer_extension<
    'info,
    P: ToAccountInfo<'info>,
    M: ToAccountInfo<'info>,
>(
    mint: &M,
    group_address: Option<Pubkey>,
    group_pointer_authority: Option<Pubkey>,
    token_program: &P,
) -> Result<()> {
    let cpi_accounts = GroupPointerInitialize {
        mint: mint.to_account_info(),
        token_program_id: token_program.to_account_info(),
    };
    let cpi_program = token_program.to_account_info();
    let ctx = CpiContext::new(cpi_program, cpi_accounts);
    group_pointer_initialize(ctx, group_pointer_authority, group_address)
}

pub fn create_initialized_mint<
    'info,
    P: ToAccountInfo<'info>,
    A: ToAccountInfo<'info>,
    // L: ToAccountInfo<'info>,
    M: ToAccountInfo<'info>,
>(
    decimals: u8,
    mint: &M,
    mint_authority: &A,
    freeze_authority: Option<&Pubkey>,
    // rent: &L,
    token_program: &P,
) -> Result<()> {
    let cpi_accounts = InitializeMint2 {
        mint: mint.to_account_info(),
        // rent: rent.to_account_info(),
    };
    let cpi_program = token_program.to_account_info();
    let ctx = CpiContext::new(cpi_program, cpi_accounts);

    initialize_mint2(
        ctx,
        decimals,
        mint_authority.to_account_info().key,
        freeze_authority,
    )
}

pub fn initialize_token_metadata_extension<
    'info,
    P: ToAccountInfo<'info>,
    A: ToAccountInfo<'info>,
    M: ToAccountInfo<'info>,
>(
    mint: &M,
    metadata: &M,
    mint_authority: &A,
    update_authority: &A,
    token_program: &P,
    name: String,
    symbol: String,
    uri: String,
) -> Result<()> {
    let initialize_cpi_accounts = TokenMetadataInitialize {
        token_program_id: token_program.to_account_info(),
        mint: mint.to_account_info(),
        metadata: metadata.to_account_info(),
        mint_authority: mint_authority.to_account_info(),
        update_authority: update_authority.to_account_info(),
    };
    let initialize_cpi_ctx =
        CpiContext::new(token_program.to_account_info(), initialize_cpi_accounts);

    token_metadata_initialize(initialize_cpi_ctx, name, symbol, uri)
}

pub fn update_token_metadata_extension_field<
    'info,
    P: ToAccountInfo<'info>,
    A: ToAccountInfo<'info>,
    M: ToAccountInfo<'info>,
>(
    metadata: &M,
    update_authority: &A,
    token_program: &P,
    key: String,
    value: String,
) -> Result<()> {
    let update_field_cpi_accounts = TokenMetadataUpdateField {
        metadata: metadata.to_account_info(),
        token_program_id: token_program.to_account_info(),
        update_authority: update_authority.to_account_info(),
    };
    let update_field_cpi_ctx =
        CpiContext::new(token_program.to_account_info(), update_field_cpi_accounts);
    token_metadata_update_field(update_field_cpi_ctx, Field::Key(key), value)
}

pub fn update_token_metadata_extension_authority<
    'info,
    P: ToAccountInfo<'info>,
    A: ToAccountInfo<'info>,
    M: ToAccountInfo<'info>,
>(
    metadata: &M,
    current_authority: &A,
    new_authority: Option<&A>,
    token_program: &P,
) -> Result<()> {
    let authority = match new_authority {
        Some(f) => f.to_account_info(),
        None => current_authority.to_account_info(),
    };

    let initialize_cpi_accounts = TokenMetadataUpdateAuthority {
        token_program_id: token_program.to_account_info(),
        metadata: metadata.to_account_info(),
        current_authority: current_authority.to_account_info(),
        new_authority: authority.clone(),
    };

    let initialize_cpi_ctx =
        CpiContext::new(token_program.to_account_info(), initialize_cpi_accounts);

    let authority_optional_non_zero_pk = if new_authority.is_some() {
        OptionalNonZeroPubkey(authority.key())
    } else {
        OptionalNonZeroPubkey::default()
    };

    token_metadata_update_authority(initialize_cpi_ctx, authority_optional_non_zero_pk)
}

pub fn initialize_token_group_member_extension<
    'info,
    P: ToAccountInfo<'info>,
    A: ToAccountInfo<'info>,
    M: ToAccountInfo<'info>,
    L: ToAccountInfo<'info>,
    R: ToAccountInfo<'info>,
    S: ToAccountInfo<'info>,
>(
    group: &P,
    group_update_authority: &A,
    member: &L,
    member_mint: &R,
    member_mint_authority: &M,
    token_program: &S,
    signer_seeds: Option<&[&[u8]]>,
) -> Result<()> {
    let cpi_accounts = TokenMemberInitialize {
        group: group.to_account_info(),
        group_update_authority: group_update_authority.to_account_info(),
        member: member.to_account_info(),
        member_mint: member_mint.to_account_info(),
        member_mint_authority: member_mint_authority.to_account_info(),
        token_program_id: token_program.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(token_program.to_account_info(), cpi_accounts);

    if let Some(seeds) = signer_seeds {
        token_member_initialize(cpi_ctx.with_signer(&[seeds]))
    } else {
        token_member_initialize(cpi_ctx)
    }
}

pub fn initialize_token_group_extension<
    'info,
    P: ToAccountInfo<'info>,
    A: ToAccountInfo<'info>,
    M: ToAccountInfo<'info>,
    S: ToAccountInfo<'info>,
>(
    max_size: u32,
    group: &P,
    mint: &M,
    mint_authority: &A,
    update_authority: Option<Pubkey>,
    token_program: &S,
    signer_seeds: Option<&[&[u8]]>,
) -> Result<()> {
    let cpi_accounts = TokenGroupInitialize {
        group: group.to_account_info(),
        mint: mint.to_account_info(),
        mint_authority: mint_authority.to_account_info(),
        token_program_id: token_program.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(token_program.to_account_info(), cpi_accounts);

    if let Some(seeds) = signer_seeds {
        token_group_initialize(cpi_ctx.with_signer(&[seeds]), update_authority, max_size)
    } else {
        token_group_initialize(cpi_ctx, update_authority, max_size)
    }
}

pub fn mint_to_token_account<
    'info,
    P: ToAccountInfo<'info>,
    A: ToAccountInfo<'info>,
    M: ToAccountInfo<'info>,
    T: ToAccountInfo<'info>,
>(
    amount: u64,
    mint: &M,
    mint_authority: &A,
    token_account: &T,
    token_program: &P,
) -> Result<()> {
    let cpi_accounts = MintTo {
        authority: mint_authority.to_account_info(),
        mint: mint.to_account_info(),
        to: token_account.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(token_program.to_account_info(), cpi_accounts);
    mint_to(cpi_ctx, amount)
}

pub fn set_account_or_mint_authority<
    'info,
    P: ToAccountInfo<'info>,
    A: ToAccountInfo<'info>,
    M: ToAccountInfo<'info>,
>(
    account_or_mint: &M,
    current_authority: &A,
    new_authority: Option<Pubkey>,
    authority_type: AuthorityType,
    program: &P,
) -> Result<()> {
    let cpi_accounts = SetAuthority {
        account_or_mint: account_or_mint.to_account_info(),
        current_authority: current_authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(program.to_account_info(), cpi_accounts);
    set_authority(cpi_ctx, authority_type, new_authority)
}
