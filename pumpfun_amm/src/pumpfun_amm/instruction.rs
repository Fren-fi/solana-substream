use borsh::BorshDeserialize;
use substreams_solana_utils::pubkey::Pubkey;

#[derive(Debug, BorshDeserialize)]
pub enum PumpfunAmmInstruction {
    CreatePool(CreatePoolInstruction),
    Buy(BuyInstruction),
    Sell(SellInstruction),
    Deposit,
    Withdraw,
    Unknown,
}

impl PumpfunAmmInstruction {
    pub fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        let (tag, data) = data.split_at(8);
        match tag {
            [242, 35, 198, 137, 82, 225, 242, 182] => {
                Ok(Self::CreatePool(CreatePoolInstruction::unpack(data)?))
            }
            [102, 6, 61, 18, 1, 218, 235, 234] => Ok(Self::Buy(BuyInstruction::unpack(data)?)),
            [51, 230, 133, 164, 1, 127, 131, 173] => Ok(Self::Sell(SellInstruction::unpack(data)?)),
            [120, 248, 61, 83, 31, 142, 107, 144] => Ok(Self::Deposit),
            [183, 18, 70, 156, 148, 109, 161, 34] => Ok(Self::Withdraw),
            _ => Ok(Self::Unknown),
        }
    }
}

#[derive(Debug, BorshDeserialize)]
pub struct DepositInstruction {
    pub lp_token_amount_out: String,
    pub max_base_amount_in: u64,
    pub max_quote_amount_in: u64,
}

impl DepositInstruction {
    fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        Self::deserialize(&mut &data[..]).map_err(|_| "Failed to deserialize DepositInstruction.")
    }
}
#[derive(Debug, BorshDeserialize)]
pub struct WithdrawInstruction {
    pub lp_token_amount_out: String,
    pub max_base_amount_in: u64,
    pub max_quote_amount_in: u64,
}

impl WithdrawInstruction {
    fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        Self::deserialize(&mut &data[..]).map_err(|_| "Failed to deserialize WithdrawInstruction.")
    }
}

#[derive(Debug, BorshDeserialize)]
pub struct CreatePoolInstruction {
    pub base_amount_in: String,
    pub quote_amount_in: String,
    pub coin_creator: String,
}

impl CreatePoolInstruction {
    fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        Self::deserialize(&mut &data[..])
            .map_err(|_| "Failed to deserialize CreatePoolInstruction.")
    }
}

#[derive(Debug, BorshDeserialize)]
pub struct BuyInstruction {
    pub base_amount_out: u64,
    pub max_quote_amount_in: u64,
}

impl BuyInstruction {
    fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        Self::deserialize(&mut &data[..]).map_err(|_| "Failed to deserialize BuyInstruction.")
    }
}

#[derive(Debug, BorshDeserialize)]
pub struct SellInstruction {
    pub base_amount_in: u64,
    pub min_quote_amount_out: u64,
}

impl SellInstruction {
    fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        Self::deserialize(&mut &data[..]).map_err(|_| "Failed to deserialize SellInstruction.")
    }
}
