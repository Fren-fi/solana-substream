use borsh::BorshDeserialize;
use substreams_solana_utils::pubkey::Pubkey;

#[derive(Debug, BorshDeserialize)]
pub enum PumpfunAmmCpiInstruction {
    CreatePoolCpi(CreatePoolCpiInstruction),
    BuyCpi(BuyCpiInstruction),
    SellCpi(SellCpiInstruction),
    DepositCpi(DepositCpiInstruction),
    WithdrawCpi(WithdrawCpiInstruction),
    Unknown,
}

impl PumpfunAmmCpiInstruction {
    pub fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() < 16 {
            return Ok(Self::Unknown);
        }
        // substreams::log::println(format!("data: {:?}", data));

        let (_cpi_tag, cpi_data) = data.split_at(8);
        let (tag, data) = cpi_data.split_at(8); // function discrimator

        match tag {
            [177, 49, 12, 210, 160, 118, 167, 116] => {
                Ok(Self::CreatePoolCpi(CreatePoolCpiInstruction::unpack(data)?))
            }
            [103, 244, 82, 31, 44, 245, 119, 119] => {
                Ok(Self::BuyCpi(BuyCpiInstruction::unpack(data)?))
            }
            [62, 47, 55, 10, 165, 3, 220, 42] => {
                Ok(Self::SellCpi(SellCpiInstruction::unpack(data)?))
            }
            [120, 248, 61, 83, 31, 142, 107, 144] => {
                Ok(Self::DepositCpi(DepositCpiInstruction::unpack(data)?))
            }
            [22, 9, 133, 26, 160, 44, 71, 192] => {
                Ok(Self::WithdrawCpi(WithdrawCpiInstruction::unpack(data)?))
            }
            _ => Ok(Self::Unknown),
        }
    }
}

#[derive(Debug, BorshDeserialize)]
pub struct CreatePoolCpiInstruction {
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
impl CreatePoolCpiInstruction {
    fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        Self::deserialize(&mut &data[..])
            .map_err(|_| "Failed to deserialize CreatePoolCpiInstruction.")
    }
}

#[derive(Debug, BorshDeserialize)]
pub struct BuyCpiInstruction {
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

impl BuyCpiInstruction {
    fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        Self::deserialize(&mut &data[..]).map_err(|_| "Failed to deserialize BuyCpiInstruction.")
    }
}

#[derive(Debug, BorshDeserialize)]
pub struct SellCpiInstruction {
    pub timestamp: i64,
    pub base_amount_in: u64,
    pub min_quote_amount_out: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub quote_amount_out: u64,
    pub lp_fee_basis_points: u64,
    pub lp_fee: u64,
    pub protocol_fee_basis_points: u64,
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
    pub coin_creator_fee_basis_points: u64,
    pub coin_creator_fee: u64,
}

impl SellCpiInstruction {
    fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        Self::deserialize(&mut &data[..]).map_err(|_| "Failed to deserialize SellCpiInstruction.")
    }
}

#[derive(Debug, BorshDeserialize)]
pub struct DepositCpiInstruction {
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

impl DepositCpiInstruction {
    fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        Self::deserialize(&mut &data[..])
            .map_err(|_| "Failed to deserialize DepositCpiInstruction.")
    }
}

#[derive(Debug, BorshDeserialize)]
pub struct WithdrawCpiInstruction {
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

impl WithdrawCpiInstruction {
    fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        Self::deserialize(&mut &data[..])
            .map_err(|_| "Failed to deserialize WithdrawCpiInstruction.")
    }
}
