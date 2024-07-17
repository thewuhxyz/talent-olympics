use std::str::FromStr;

use crate::error::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::spl_token_metadata_interface::state::TokenMetadata;

#[account]
#[derive(InitSpace, PartialEq)]
pub struct ServiceAccount {
    pub holder: Pubkey,
    pub mint: Pubkey,
    pub bump: u8,
    pub is_listed: bool,
    pub is_initialized: bool,
}

impl ServiceAccount {
    pub fn init(&mut self, holder: Pubkey, mint: Pubkey, bump: u8) -> Result<()> {
        self.holder = holder;
        self.mint = mint;
        self.bump = bump;
        self.is_initialized = true;
        self.is_listed = false;

        Ok(())
    }

    pub fn update_holder(&mut self, new_holder: Pubkey) {
        self.holder = new_holder;
        self.is_listed = false;
    }

    pub fn unlist(&mut self) {
        self.is_listed = false;
    }

    pub fn list(&mut self) {
        self.is_listed = true;
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ServiceAgreement {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub provider: Pubkey,
    pub receiver: Pubkey,
    pub description: String,
    pub price: u64,
    pub fee_basis_points: u16,
    pub maximum_fee: u64,
    pub transferable: bool,
}

impl ServiceAgreement {
    pub fn to_additional_metadata(&self) -> Vec<(String, String)> {
        vec![
            (
                Self::PROVIDER_KEY.to_string(),
                self.provider.to_string().clone(),
            ),
            (
                Self::RECEIVER_KEY.to_string(),
                self.receiver.to_string().clone(),
            ),
            (Self::DESCRIPTION_KEY.to_string(), self.description.clone()),
            (Self::PRICE_KEY.to_string(), self.price.to_string()),
            (
                Self::TRANSFERABLE_KEY.to_string(),
                self.transferable.to_string(),
            ),
            (
                Self::FEE_BASIS_POINTS_KEY.to_string(),
                self.fee_basis_points.to_string(),
            ),
            (
                Self::MAXIMUM_FEE_KEY.to_string(),
                self.fee_basis_points.to_string(),
            ),
        ]
    }

    pub fn royalties_split(&self) -> Result<(u64, u64)> {
        if let Some(numerator) =
            u128::from(self.price).checked_mul(u128::from(self.fee_basis_points))
        {
            if let Some(provider_fee_u128) = numerator.checked_div(10000) {
                let provider_fee = u64::try_from(provider_fee_u128)?;

                if let Some(reseller_fee) = self.price.checked_sub(provider_fee) {
                    return Ok((reseller_fee, provider_fee));
                };
            }
        };
        err!(ErrorCode::OverflowOccurred)
    }

    pub const DESCRIPTION_KEY: &'static str = "description";
    pub const PRICE_KEY: &'static str = "price";
    pub const FEE_BASIS_POINTS_KEY: &'static str = "fee-basis-points";
    pub const MAXIMUM_FEE_KEY: &'static str = "maximum-fee";
    pub const TRANSFERABLE_KEY: &'static str = "transferable";
    pub const PROVIDER_KEY: &'static str = "provider";
    pub const RECEIVER_KEY: &'static str = "receiver";
}

impl TryFrom<AccountInfo<'_>> for ServiceAgreement {
    type Error = Error;

    fn try_from(mut account_info: AccountInfo) -> Result<Self> {
        let metadata =
            utils::get_mint_extensible_extension_data::<TokenMetadata>(&mut account_info)?;
        let additional_metadata = metadata.additional_metadata;

        let name = metadata.name;
        let symbol = metadata.symbol;
        let uri = metadata.uri;
        let provider =
            get_metadata_value::<Pubkey>(additional_metadata[0].clone(), Self::PROVIDER_KEY)?;
        let receiver =
            get_metadata_value::<Pubkey>(additional_metadata[1].clone(), Self::RECEIVER_KEY)?;
        let description =
            get_metadata_value::<String>(additional_metadata[2].clone(), Self::DESCRIPTION_KEY)?;
        let price = get_metadata_value::<u64>(additional_metadata[3].clone(), Self::PRICE_KEY)?;
        let transferable =
            get_metadata_value::<bool>(additional_metadata[4].clone(), Self::TRANSFERABLE_KEY)?;
        let fee_basis_points =
            get_metadata_value::<u16>(additional_metadata[5].clone(), Self::FEE_BASIS_POINTS_KEY)?;
        let maximum_fee =
            get_metadata_value::<u64>(additional_metadata[6].clone(), Self::MAXIMUM_FEE_KEY)?;

        Ok(ServiceAgreement {
            name,
            symbol,
            uri,
            provider,
            receiver,
            description,
            fee_basis_points,
            price,
            transferable,
            maximum_fee,
        })
    }
}

fn get_metadata_value<T>(metadata: (String, String), key: &str) -> Result<T>
where
    T: FromStr,
{
    if key != metadata.0 {
        return Err(error!(ErrorCode::MetadataKeyMismatch));
    };

    if let Ok(value) = metadata.1.parse() {
        return Ok(value);
    }

    Err(error!(ErrorCode::MetadataValueParseError))
}

// #[derive(Clone)]
// pub struct MintRoyaltyConfig(utils::state::MintRoyaltyConfig);

// impl anchor_lang::Owner for MintRoyaltyConfig {
//     fn owner() -> Pubkey {
//         return marketplace_transfer_controller::ID;
//     }
// }

// impl anchor_lang::AccountDeserialize for MintRoyaltyConfig {
//     fn try_deserialize_unchecked(data: &mut &[u8]) -> Result<Self> {
//         if let Ok(royalty_config) = utils::state::MintRoyaltyConfig::deserialize(data) {
//             return Ok(MintRoyaltyConfig(royalty_config));
//         }
//         err!(ErrorCode::MintRoyaltyParseError)
//     }
// }

// impl anchor_lang::AccountSerialize for MintRoyaltyConfig {
//     fn try_serialize<W: std::io::Write>(&self, _writer: &mut W) -> std::result::Result<(), Error> {
//         Err(error!(ErrorCode::MintRoyaltyParseError))
//     }
// }

// impl Deref for MintRoyaltyConfig {
//     type Target = utils::state::MintRoyaltyConfig;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
