// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrensBlockEvents {
    #[prost(message, repeated, tag="2")]
    pub transactions: ::prost::alloc::vec::Vec<FrensTransactionEvents>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrensTransactionEvents {
    #[prost(string, tag="1")]
    pub signature: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub events: ::prost::alloc::vec::Vec<FrensEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrensEvent {
    #[prost(oneof="frens_event::Event", tags="1, 2, 3, 4")]
    pub event: ::core::option::Option<frens_event::Event>,
}
/// Nested message and enum types in `FrensEvent`.
pub mod frens_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="1")]
        PoolCreateEvent(super::PoolCreateEventEvent),
        #[prost(message, tag="2")]
        TradeEvent(super::TradeEventEvent),
        #[prost(message, tag="3")]
        ClaimVested(super::ClaimVestedEventEvent),
        #[prost(message, tag="4")]
        CreateVestingEvent(super::CreateVestingEventEvent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    #[prost(message, repeated, tag="1")]
    pub claim_vested_event_event_list: ::prost::alloc::vec::Vec<ClaimVestedEventEvent>,
    #[prost(message, repeated, tag="2")]
    pub create_vesting_event_event_list: ::prost::alloc::vec::Vec<CreateVestingEventEvent>,
    #[prost(message, repeated, tag="3")]
    pub pool_create_event_event_list: ::prost::alloc::vec::Vec<PoolCreateEventEvent>,
    #[prost(message, repeated, tag="4")]
    pub trade_event_event_list: ::prost::alloc::vec::Vec<TradeEventEvent>,
    #[prost(message, repeated, tag="5")]
    pub buy_exact_in_instruction_list: ::prost::alloc::vec::Vec<BuyExactInInstruction>,
    #[prost(message, repeated, tag="6")]
    pub buy_exact_out_instruction_list: ::prost::alloc::vec::Vec<BuyExactOutInstruction>,
    #[prost(message, repeated, tag="7")]
    pub claim_platform_fee_instruction_list: ::prost::alloc::vec::Vec<ClaimPlatformFeeInstruction>,
    #[prost(message, repeated, tag="8")]
    pub claim_vested_token_instruction_list: ::prost::alloc::vec::Vec<ClaimVestedTokenInstruction>,
    #[prost(message, repeated, tag="9")]
    pub collect_fee_instruction_list: ::prost::alloc::vec::Vec<CollectFeeInstruction>,
    #[prost(message, repeated, tag="10")]
    pub collect_migrate_fee_instruction_list: ::prost::alloc::vec::Vec<CollectMigrateFeeInstruction>,
    #[prost(message, repeated, tag="11")]
    pub create_config_instruction_list: ::prost::alloc::vec::Vec<CreateConfigInstruction>,
    #[prost(message, repeated, tag="12")]
    pub create_platform_config_instruction_list: ::prost::alloc::vec::Vec<CreatePlatformConfigInstruction>,
    #[prost(message, repeated, tag="13")]
    pub create_vesting_account_instruction_list: ::prost::alloc::vec::Vec<CreateVestingAccountInstruction>,
    #[prost(message, repeated, tag="14")]
    pub initialize_instruction_list: ::prost::alloc::vec::Vec<InitializeInstruction>,
    #[prost(message, repeated, tag="15")]
    pub migrate_to_amm_instruction_list: ::prost::alloc::vec::Vec<MigrateToAmmInstruction>,
    #[prost(message, repeated, tag="16")]
    pub migrate_to_cpswap_instruction_list: ::prost::alloc::vec::Vec<MigrateToCpswapInstruction>,
    #[prost(message, repeated, tag="17")]
    pub sell_exact_in_instruction_list: ::prost::alloc::vec::Vec<SellExactInInstruction>,
    #[prost(message, repeated, tag="18")]
    pub sell_exact_out_instruction_list: ::prost::alloc::vec::Vec<SellExactOutInstruction>,
    #[prost(message, repeated, tag="19")]
    pub update_config_instruction_list: ::prost::alloc::vec::Vec<UpdateConfigInstruction>,
    #[prost(message, repeated, tag="20")]
    pub update_platform_config_instruction_list: ::prost::alloc::vec::Vec<UpdatePlatformConfigInstruction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimVestedEventEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pool_state: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub beneficiary: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub claim_amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVestingEventEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pool_state: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub beneficiary: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub share_amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolCreateEventEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub platform_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub pool_state: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub config: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub base_mint_param: ::core::option::Option<MintParams>,
    #[prost(enumeration="CurveParamsEnum", tag="7")]
    pub curve_param: i32,
    #[prost(message, optional, tag="8")]
    pub vesting_param: ::core::option::Option<VestingParams>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeEventEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub platform_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub pool_state: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub total_base_sell: u64,
    #[prost(uint64, tag="5")]
    pub virtual_base: u64,
    #[prost(uint64, tag="6")]
    pub virtual_quote: u64,
    #[prost(uint64, tag="7")]
    pub real_base_before: u64,
    #[prost(uint64, tag="8")]
    pub real_quote_before: u64,
    #[prost(uint64, tag="9")]
    pub real_base_after: u64,
    #[prost(uint64, tag="10")]
    pub real_quote_after: u64,
    #[prost(uint64, tag="11")]
    pub amount_in: u64,
    #[prost(uint64, tag="12")]
    pub amount_out: u64,
    #[prost(uint64, tag="13")]
    pub protocol_fee: u64,
    #[prost(uint64, tag="14")]
    pub platform_fee: u64,
    #[prost(uint64, tag="15")]
    pub share_fee: u64,
    #[prost(enumeration="TradeDirectionEnum", tag="16")]
    pub trade_direction: i32,
    #[prost(enumeration="PoolStatusEnum", tag="17")]
    pub pool_status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyExactInInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub amount_in: u64,
    #[prost(uint64, tag="3")]
    pub minimum_amount_out: u64,
    #[prost(uint64, tag="4")]
    pub share_fee_rate: u64,
    #[prost(string, tag="5")]
    pub acct_payer: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_authority: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_global_config: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_platform_config: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_pool_state: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_user_base_token: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_user_quote_token: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_base_vault: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_quote_vault: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_base_token_mint: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_quote_token_mint: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_base_token_program: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyExactOutInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub amount_out: u64,
    #[prost(uint64, tag="3")]
    pub maximum_amount_in: u64,
    #[prost(uint64, tag="4")]
    pub share_fee_rate: u64,
    #[prost(string, tag="5")]
    pub acct_payer: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_authority: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_global_config: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_platform_config: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_pool_state: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_user_base_token: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_user_quote_token: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_base_vault: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_quote_vault: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_base_token_mint: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_quote_token_mint: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_base_token_program: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimPlatformFeeInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_platform_fee_wallet: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_authority: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_pool_state: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_platform_config: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_quote_vault: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_recipient_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_quote_mint: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimVestedTokenInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_beneficiary: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_authority: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_pool_state: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_vesting_record: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_base_vault: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_user_base_token: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_base_token_mint: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectFeeInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_owner: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_authority: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_pool_state: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_global_config: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_quote_vault: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_quote_mint: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_recipient_token_account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectMigrateFeeInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_owner: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_authority: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_pool_state: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_global_config: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_quote_vault: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_quote_mint: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_recipient_token_account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConfigInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub curve_type: u64,
    #[prost(uint64, tag="3")]
    pub index: u64,
    #[prost(uint64, tag="4")]
    pub migrate_fee: u64,
    #[prost(uint64, tag="5")]
    pub trade_fee_rate: u64,
    #[prost(string, tag="6")]
    pub acct_global_config: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_quote_token_mint: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_protocol_fee_owner: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_migrate_fee_owner: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_migrate_to_amm_wallet: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_migrate_to_cpswap_wallet: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePlatformConfigInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub platform_params: ::core::option::Option<PlatformParams>,
    #[prost(string, tag="3")]
    pub acct_platform_admin: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_platform_fee_wallet: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_platform_nft_wallet: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_platform_config: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVestingAccountInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub share_amount: u64,
    #[prost(string, tag="3")]
    pub acct_creator: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_beneficiary: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_pool_state: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_vesting_record: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub base_mint_param: ::core::option::Option<MintParams>,
    #[prost(enumeration="CurveParamsEnum", tag="3")]
    pub curve_param: i32,
    #[prost(message, optional, tag="4")]
    pub vesting_param: ::core::option::Option<VestingParams>,
    #[prost(string, tag="5")]
    pub acct_payer: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_creator: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_global_config: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_platform_config: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_authority: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_pool_state: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_base_mint: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_quote_mint: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_base_vault: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_quote_vault: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_metadata_account: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrateToAmmInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub base_lot_size: u64,
    #[prost(uint64, tag="3")]
    pub quote_lot_size: u64,
    #[prost(uint64, tag="4")]
    pub market_vault_signer_nonce: u64,
    #[prost(string, tag="5")]
    pub acct_payer: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_base_mint: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_quote_mint: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_market: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_request_queue: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_event_queue: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_bids: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_asks: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_market_vault_signer: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_market_base_vault: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_market_quote_vault: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_amm_pool: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_amm_authority: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub acct_amm_open_orders: ::prost::alloc::string::String,
    #[prost(string, tag="19")]
    pub acct_amm_lp_mint: ::prost::alloc::string::String,
    #[prost(string, tag="20")]
    pub acct_amm_base_vault: ::prost::alloc::string::String,
    #[prost(string, tag="21")]
    pub acct_amm_quote_vault: ::prost::alloc::string::String,
    #[prost(string, tag="22")]
    pub acct_amm_target_orders: ::prost::alloc::string::String,
    #[prost(string, tag="23")]
    pub acct_amm_config: ::prost::alloc::string::String,
    #[prost(string, tag="24")]
    pub acct_amm_create_fee_destination: ::prost::alloc::string::String,
    #[prost(string, tag="25")]
    pub acct_authority: ::prost::alloc::string::String,
    #[prost(string, tag="26")]
    pub acct_pool_state: ::prost::alloc::string::String,
    #[prost(string, tag="27")]
    pub acct_global_config: ::prost::alloc::string::String,
    #[prost(string, tag="28")]
    pub acct_base_vault: ::prost::alloc::string::String,
    #[prost(string, tag="29")]
    pub acct_quote_vault: ::prost::alloc::string::String,
    #[prost(string, tag="30")]
    pub acct_pool_lp_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrateToCpswapInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_payer: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_base_mint: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_quote_mint: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_platform_config: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_cpswap_pool: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_cpswap_authority: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_cpswap_lp_mint: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_cpswap_base_vault: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_cpswap_quote_vault: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_cpswap_config: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_cpswap_create_pool_fee: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_cpswap_observation: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_lock_authority: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_lock_lp_vault: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_authority: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_pool_state: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub acct_global_config: ::prost::alloc::string::String,
    #[prost(string, tag="19")]
    pub acct_base_vault: ::prost::alloc::string::String,
    #[prost(string, tag="20")]
    pub acct_quote_vault: ::prost::alloc::string::String,
    #[prost(string, tag="21")]
    pub acct_pool_lp_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SellExactInInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub amount_in: u64,
    #[prost(uint64, tag="3")]
    pub minimum_amount_out: u64,
    #[prost(uint64, tag="4")]
    pub share_fee_rate: u64,
    #[prost(string, tag="5")]
    pub acct_payer: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_authority: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_global_config: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_platform_config: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_pool_state: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_user_base_token: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_user_quote_token: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_base_vault: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_quote_vault: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_base_token_mint: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_quote_token_mint: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_base_token_program: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SellExactOutInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub amount_out: u64,
    #[prost(uint64, tag="3")]
    pub maximum_amount_in: u64,
    #[prost(uint64, tag="4")]
    pub share_fee_rate: u64,
    #[prost(string, tag="5")]
    pub acct_payer: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_authority: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_global_config: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_platform_config: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_pool_state: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_user_base_token: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_user_quote_token: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_base_vault: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_quote_vault: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_base_token_mint: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_quote_token_mint: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_base_token_program: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConfigInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub param: u64,
    #[prost(uint64, tag="3")]
    pub value: u64,
    #[prost(string, tag="4")]
    pub acct_global_config: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePlatformConfigInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(enumeration="PlatformConfigParamEnum", tag="2")]
    pub param: i32,
    #[prost(string, tag="3")]
    pub acct_platform_admin: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_platform_config: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrateNftInfo {
    #[prost(uint64, tag="1")]
    pub platform_scale: u64,
    #[prost(uint64, tag="2")]
    pub creator_scale: u64,
    #[prost(uint64, tag="3")]
    pub burn_scale: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintParams {
    #[prost(uint64, tag="1")]
    pub decimals: u64,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub uri: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlatformParams {
    #[prost(message, optional, tag="1")]
    pub migrate_nft_info: ::core::option::Option<MigrateNftInfo>,
    #[prost(uint64, tag="2")]
    pub fee_rate: u64,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub web: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub img: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VestingParams {
    #[prost(uint64, tag="1")]
    pub total_locked_amount: u64,
    #[prost(uint64, tag="2")]
    pub cliff_period: u64,
    #[prost(uint64, tag="3")]
    pub unlock_period: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CurveParamsEnum {
    CurveParamsConstant = 0,
    CurveParamsFixed = 1,
    CurveParamsLinear = 2,
}
impl CurveParamsEnum {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CurveParamsEnum::CurveParamsConstant => "CURVE_PARAMS_CONSTANT",
            CurveParamsEnum::CurveParamsFixed => "CURVE_PARAMS_FIXED",
            CurveParamsEnum::CurveParamsLinear => "CURVE_PARAMS_LINEAR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CURVE_PARAMS_CONSTANT" => Some(Self::CurveParamsConstant),
            "CURVE_PARAMS_FIXED" => Some(Self::CurveParamsFixed),
            "CURVE_PARAMS_LINEAR" => Some(Self::CurveParamsLinear),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlatformConfigParamEnum {
    PlatformConfigParamFeeWallet = 0,
    PlatformConfigParamNFTWallet = 1,
    PlatformConfigParamMigrateNftInfo = 2,
    PlatformConfigParamFeeRate = 3,
    PlatformConfigParamName = 4,
    PlatformConfigParamWeb = 5,
    PlatformConfigParamImg = 6,
}
impl PlatformConfigParamEnum {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PlatformConfigParamEnum::PlatformConfigParamFeeWallet => "PLATFORM_CONFIG_PARAM_FEE_WALLET",
            PlatformConfigParamEnum::PlatformConfigParamNFTWallet => "PLATFORM_CONFIG_PARAM_N_F_T_WALLET",
            PlatformConfigParamEnum::PlatformConfigParamMigrateNftInfo => "PLATFORM_CONFIG_PARAM_MIGRATE_NFT_INFO",
            PlatformConfigParamEnum::PlatformConfigParamFeeRate => "PLATFORM_CONFIG_PARAM_FEE_RATE",
            PlatformConfigParamEnum::PlatformConfigParamName => "PLATFORM_CONFIG_PARAM_NAME",
            PlatformConfigParamEnum::PlatformConfigParamWeb => "PLATFORM_CONFIG_PARAM_WEB",
            PlatformConfigParamEnum::PlatformConfigParamImg => "PLATFORM_CONFIG_PARAM_IMG",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PLATFORM_CONFIG_PARAM_FEE_WALLET" => Some(Self::PlatformConfigParamFeeWallet),
            "PLATFORM_CONFIG_PARAM_N_F_T_WALLET" => Some(Self::PlatformConfigParamNFTWallet),
            "PLATFORM_CONFIG_PARAM_MIGRATE_NFT_INFO" => Some(Self::PlatformConfigParamMigrateNftInfo),
            "PLATFORM_CONFIG_PARAM_FEE_RATE" => Some(Self::PlatformConfigParamFeeRate),
            "PLATFORM_CONFIG_PARAM_NAME" => Some(Self::PlatformConfigParamName),
            "PLATFORM_CONFIG_PARAM_WEB" => Some(Self::PlatformConfigParamWeb),
            "PLATFORM_CONFIG_PARAM_IMG" => Some(Self::PlatformConfigParamImg),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PoolStatusEnum {
    PoolStatusFund = 0,
    PoolStatusMigrate = 1,
    PoolStatusTrade = 2,
}
impl PoolStatusEnum {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PoolStatusEnum::PoolStatusFund => "Pool_Status_Fund",
            PoolStatusEnum::PoolStatusMigrate => "Pool_Status_Migrate",
            PoolStatusEnum::PoolStatusTrade => "Pool_Status_TRADE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Pool_Status_Fund" => Some(Self::PoolStatusFund),
            "Pool_Status_Migrate" => Some(Self::PoolStatusMigrate),
            "Pool_Status_TRADE" => Some(Self::PoolStatusTrade),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TradeDirectionEnum {
    TradeDirectionBuy = 0,
    TradeDirectionSell = 1,
}
impl TradeDirectionEnum {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TradeDirectionEnum::TradeDirectionBuy => "Trade_Direction_BUY",
            TradeDirectionEnum::TradeDirectionSell => "Trade_Direction_SELL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Trade_Direction_BUY" => Some(Self::TradeDirectionBuy),
            "Trade_Direction_SELL" => Some(Self::TradeDirectionSell),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
