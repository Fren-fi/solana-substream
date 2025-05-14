use borsh::BorshDeserialize;
use std::fmt::{self, Display};

#[derive(BorshDeserialize)]
pub struct Pubkey(pub [u8; 32]);

impl Display for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", bs58::encode(self.0).into_string())
    }
}

impl fmt::Debug for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Pubkey")
            .field(&bs58::encode(self.0).into_string())
            .finish()
    }
}

#[derive(Debug)]
pub enum PumpfunAmmLog {
    CreatePool(CreatePoolLog),
    Buy(BuyLog),
    Sell(SellLog),
    Deposit(DepositLog),
    Withdraw(WithdrawLog),
}

impl PumpfunAmmLog {
    pub fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        let (discriminator, data) = data.split_at(8);
        match discriminator {
            [177, 49, 12, 210, 160, 118, 167, 116] => CreatePoolLog::try_from_slice(data)
                .map(Self::CreatePool)
                .map_err(|_| "Failed to unpack CreateEvent."),
            [103, 244, 82, 31, 44, 245, 119, 119] => BuyLog::try_from_slice(data)
                .map(Self::Buy)
                .map_err(|_| "Failed to unpack BuyEvent."),
            [62, 47, 55, 10, 165, 3, 220, 42] => SellLog::try_from_slice(data)
                .map(Self::Sell)
                .map_err(|_| "Failed to unpack SellEvent."),
            [120, 248, 61, 83, 31, 142, 107, 144] => DepositLog::try_from_slice(data)
                .map(Self::Deposit)
                .map_err(|_| "Failed to unpack DepositEvent."),
            [22, 9, 133, 26, 160, 44, 71, 192] => WithdrawLog::try_from_slice(data)
                .map(Self::Withdraw)
                .map_err(|_| "Failed to unpack WithdrawEvent."),
            _ => Err("Unknown PumpfunAmm event."),
        }
    }
}

#[derive(Debug, BorshDeserialize)]
pub struct CreatePoolLog {
    pub timestamp: i64,
    pub index: u16,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_mint_decimals: u8,
    pub quote_mint_decimals: u8,
    pub base_amount_in: u64,
    pub quote_amount_in: u64,
    pub pool_base_amount: u64,
    pub pool_quote_amount: u64,
    pub minimum_liquidity: u64,
    pub initial_liquidity: u64,
    pub lp_token_amount_out: u64,
    pub pool_bump: u8,
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub coin_creator: Pubkey,
}

#[derive(Debug, BorshDeserialize)]
pub struct BuyLog {
    pub timestamp: i64,
    pub base_amount_out: u64,
    pub max_quote_amount_in: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub quote_amount_in: u64,
    pub lp_fee_basis_points: u16,
    pub lp_fee: u64,
    pub protocol_fee_basis_points: u16,
    pub protocol_fee: u64,
    pub quote_amount_in_with_lp_fee: u64,
    pub user_quote_amount_in: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub protocol_fee_recipient_token_account: Pubkey,
    pub coin_creator: Pubkey,
    pub coin_creator_fee_basis_points: u16,
    pub coin_creator_fee: u64,
}

#[derive(Debug, BorshDeserialize)]
pub struct SellLog {
    pub timestamp: i64,
    pub base_amount_in: u64,
    pub min_quote_amount_out: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub quote_amount_out: u64,
    pub lp_fee_basis_points: u16,
    pub lp_fee: u64,
    pub protocol_fee_basis_points: u16,
    pub protocol_fee: u64,
    pub quote_amount_out_without_lp_fee: u64,
    pub user_quote_amount_out: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub protocol_fee_recipient: Pubkey,
    pub protocol_fee_recipient_token_account: Pubkey,
    pub coin_creator: Pubkey,
    pub coin_creator_fee_basis_points: u16,
    pub coin_creator_fee: u64,
}

#[derive(Debug, BorshDeserialize)]
pub struct DepositLog {
    pub timestamp: i64,
    pub lp_token_amount_out: u64,
    pub max_base_amount_in: u64,
    pub max_quote_amount_in: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub base_amount_in: u64,
    pub quote_amount_in: u64,
    pub lp_mint_supply: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub user_pool_token_account: Pubkey,
}

#[derive(Debug, BorshDeserialize)]
pub struct WithdrawLog {
    pub timestamp: i64,
    pub lp_token_amount_in: u64,
    pub min_base_amount_out: u64,
    pub min_quote_amount_out: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub base_amount_out: u64,
    pub quote_amount_out: u64,
    pub lp_mint_supply: u64,
    pub pool: Pubkey,
    pub user: Pubkey,
    pub user_base_token_account: Pubkey,
    pub user_quote_token_account: Pubkey,
    pub user_pool_token_account: Pubkey,
}
