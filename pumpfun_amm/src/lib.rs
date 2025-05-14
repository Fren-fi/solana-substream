use anyhow::{anyhow, Context, Error};

use substreams_solana::pb::sf::solana::r#type::v1::Block;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;

pub mod pumpfun_amm;
use pumpfun_amm::constants::PUMPFUN_AMM_PROGRAM_ID;
use pumpfun_amm::instruction::PumpfunAmmInstruction;
use pumpfun_amm::log::PumpfunAmmLog;

use substreams_solana_utils as utils;
use utils::instruction::{
    get_structured_instructions, StructuredInstruction, StructuredInstructions,
};
use utils::log::Log;
use utils::transaction::{get_context, TransactionContext};

pub mod pb;
use pb::pumpfun_amm::pumpfun_amm_event::Event;
use pb::pumpfun_amm::*;

#[substreams::handlers::map]
fn pumpfun_amm_events(block: Block) -> Result<PumpfunAmmBlockEvents, Error> {
    let transactions = parse_block(&block);
    Ok(PumpfunAmmBlockEvents { transactions })
}

pub fn parse_block(block: &Block) -> Vec<PumpfunAmmTransactionEvents> {
    let mut block_events: Vec<PumpfunAmmTransactionEvents> = Vec::new();
    for transaction in block.transactions.iter() {
        if let Ok(events) = parse_transaction(transaction) {
            if !events.is_empty() {
                block_events.push(PumpfunAmmTransactionEvents {
                    signature: utils::transaction::get_signature(&transaction),
                    events,
                });
            }
        }
    }
    block_events
}

pub fn parse_transaction(
    transaction: &ConfirmedTransaction,
) -> Result<Vec<PumpfunAmmEvent>, Error> {
    if let Some(_) = transaction.meta.as_ref().unwrap().err {
        return Ok(Vec::new());
    }

    let mut events: Vec<PumpfunAmmEvent> = Vec::new();

    let mut context = get_context(transaction)?;
    let instructions = get_structured_instructions(transaction)?;
    for instruction in instructions.flattened().iter() {
        context.update_balance(&instruction.instruction);
        if instruction.program_id() != PUMPFUN_AMM_PROGRAM_ID {
            continue;
        }

        match parse_instruction(&instruction, &context) {
            Ok(Some(event)) => events.push(PumpfunAmmEvent { event: Some(event) }),
            Ok(None) => (),
            Err(error) => substreams::log::println(format!(
                "Failed to process instruction of transaction {}: {}",
                &context.signature, error
            )),
        }
    }
    Ok(events)
}

pub fn parse_instruction<'a>(
    instruction: &StructuredInstruction<'a>,
    context: &TransactionContext,
) -> Result<Option<Event>, Error> {
    if instruction.program_id() != PUMPFUN_AMM_PROGRAM_ID {
        return Err(anyhow!("Not a Pumpfun Amm instruction."));
    }

    let unpacked = PumpfunAmmInstruction::unpack(instruction.data()).map_err(|x| anyhow!(x))?;
    match unpacked {
        PumpfunAmmInstruction::CreatePool(create) => Ok(Some(Event::CreatePool(
            _parse_create_pool_instruction(instruction, context, create)?,
        ))),
        PumpfunAmmInstruction::Buy(buy) => Ok(Some(Event::Swap(_parse_buy_instruction(
            instruction,
            context,
            buy,
        )?))),
        PumpfunAmmInstruction::Sell(sell) => Ok(Some(Event::Swap(_parse_sell_instruction(
            instruction,
            context,
            sell,
        )?))),
        PumpfunAmmInstruction::Deposit => Ok(Some(Event::Liquidity(_parse_deposit_instruction(
            instruction,
            context,
        )?))),
        PumpfunAmmInstruction::Withdraw => Ok(Some(Event::Liquidity(_parse_withdraw_instruction(
            instruction,
            context,
        )?))),
        _ => Ok(None),
    }
}

fn _parse_create_pool_instruction(
    instruction: &StructuredInstruction,
    _context: &TransactionContext,
    create: pumpfun_amm::instruction::CreatePoolInstruction,
) -> Result<CreatePoolEvent, Error> {
    let pool_event = match parse_pumpfun_amm_log(instruction) {
        Ok(PumpfunAmmLog::CreatePool(create)) => Some(create),
        _ => None,
    };

    let pool = pool_event
        .as_ref()
        .map(|x| x.pool.to_string())
        .unwrap_or_default();
    let creator = pool_event.as_ref().map(|x| x.creator.to_string());
    let coin_creator = pool_event.as_ref().map(|x| x.coin_creator.to_string());
    let base_mint = pool_event.as_ref().map(|x| x.base_mint.to_string());
    let quote_mint = pool_event.as_ref().map(|x| x.quote_mint.to_string());
    let base_mint_decimals = pool_event.as_ref().map(|x| x.base_mint_decimals as u32);
    let quote_mint_decimals = pool_event.as_ref().map(|x| x.quote_mint_decimals as u32);
    let base_amount_in = pool_event.as_ref().map(|x| x.base_amount_in);
    let quote_amount_in = pool_event.as_ref().map(|x| x.quote_amount_in);
    let pool_base_amount: Option<u64> = pool_event.as_ref().map(|x| x.pool_base_amount);
    let pool_quote_amount: Option<u64> = pool_event.as_ref().map(|x| x.pool_quote_amount);
    let timestamp = pool_event.as_ref().map(|x| x.timestamp);

    Ok(CreatePoolEvent {
        pool,
        creator,
        coin_creator,
        base_mint,
        quote_mint,
        base_mint_decimals,
        quote_mint_decimals,
        base_amount_in,
        quote_amount_in,
        pool_base_amount,
        pool_quote_amount,
        timestamp,
    })
}

fn _parse_buy_instruction<'a>(
    instruction: &StructuredInstruction<'a>,
    context: &TransactionContext,
    buy: pumpfun_amm::instruction::BuyInstruction,
) -> Result<SwapEvent, Error> {
    let mint = instruction.accounts()[0].to_string(); // pool
    let user = instruction.accounts()[1].to_string();
    let bonding_curve = instruction.accounts()[3].to_string();

    let trade = match parse_pumpfun_amm_log(instruction) {
        Ok(PumpfunAmmLog::Buy(buy)) => Some(buy),
        _ => None,
    };

    let base_amount_in: u64 = trade
        .as_ref()
        .map(|x| x.quote_amount_in)
        .unwrap_or_default();
    let min_quote_amount_out: u64 = trade
        .as_ref()
        .map(|x| x.base_amount_out)
        .unwrap_or_default();
    let virtual_sol_reserves: Option<u64> = Some(0);
    let virtual_token_reserves: Option<u64> = Some(0);
    let real_sol_reserves: Option<u64> = trade.as_ref().map(|x| x.pool_quote_token_reserves);
    let real_token_reserves: Option<u64> = trade.as_ref().map(|x| x.pool_base_token_reserves);
    let protocol_fee: Option<u64> = trade.as_ref().map(|x| x.protocol_fee);
    let coin_creator_fee: Option<u64> = trade.as_ref().map(|x| x.coin_creator_fee);
    let timestamp: Option<i64> = trade.as_ref().map(|x| x.timestamp);
    let user_token_pre_balance: Option<u64> = Some(0);
    let direction = "token".to_string();
    let is_buy = true;

    Ok(SwapEvent {
        user,
        mint,
        bonding_curve,
        min_quote_amount_out,
        base_amount_in,
        direction,
        is_buy,
        virtual_sol_reserves,
        virtual_token_reserves,
        real_sol_reserves,
        real_token_reserves,
        user_token_pre_balance,
        protocol_fee,
        coin_creator_fee,
        timestamp,
    })
}

fn _parse_sell_instruction<'a>(
    instruction: &StructuredInstruction<'a>,
    _context: &TransactionContext,
    _buy: pumpfun_amm::instruction::SellInstruction,
) -> Result<SwapEvent, Error> {
    let mint = instruction.accounts()[0].to_string(); // pool
    let user = instruction.accounts()[1].to_string();
    let bonding_curve = instruction.accounts()[3].to_string();

    let trade = match parse_pumpfun_amm_log(instruction) {
        Ok(PumpfunAmmLog::Sell(sell)) => Some(sell),
        _ => None,
    };

    let base_amount_in: u64 = trade.as_ref().map(|x| x.base_amount_in).unwrap_or_default();
    let min_quote_amount_out: u64 = trade
        .as_ref()
        .map(|x| x.quote_amount_out)
        .unwrap_or_default();
    let virtual_sol_reserves: Option<u64> = Some(0);
    let virtual_token_reserves: Option<u64> = Some(0);
    let real_sol_reserves: Option<u64> = trade.as_ref().map(|x| x.pool_quote_token_reserves);
    let real_token_reserves: Option<u64> = trade.as_ref().map(|x| x.pool_base_token_reserves);
    let protocol_fee: Option<u64> = trade.as_ref().map(|x| x.protocol_fee);
    let coin_creator_fee: Option<u64> = trade.as_ref().map(|x| x.coin_creator_fee);
    let timestamp: Option<i64> = trade.as_ref().map(|x| x.timestamp);
    let user_token_pre_balance: Option<u64> = Some(0);
    let direction = "sol".to_string();
    let is_buy = false;

    Ok(SwapEvent {
        user,
        mint,
        bonding_curve,
        min_quote_amount_out,
        base_amount_in,
        direction,
        is_buy,
        virtual_sol_reserves,
        virtual_token_reserves,
        real_sol_reserves,
        real_token_reserves,
        user_token_pre_balance,
        protocol_fee,
        coin_creator_fee,
        timestamp,
    })
}

fn _parse_deposit_instruction(
    instruction: &StructuredInstruction,
    _context: &TransactionContext,
) -> Result<LiquidityEvent, Error> {
    let pool = instruction.accounts()[0].to_string();
    let user = instruction.accounts()[2].to_string();

    let liquidity = match parse_pumpfun_amm_log(instruction) {
        Ok(PumpfunAmmLog::Withdraw(withdraw)) => Some(withdraw),
        _ => None,
    };

    let pool_base_token_reserves: Option<u64> =
        liquidity.as_ref().map(|x| x.pool_base_token_reserves);
    let pool_quote_token_reserves: Option<u64> =
        liquidity.as_ref().map(|x| x.pool_quote_token_reserves);

    let is_add = true;

    Ok(LiquidityEvent {
        pool,
        user,
        is_add,
        pool_base_token_reserves,
        pool_quote_token_reserves,
    })
}

fn _parse_withdraw_instruction(
    instruction: &StructuredInstruction,
    _context: &TransactionContext,
) -> Result<LiquidityEvent, Error> {
    let pool = instruction.accounts()[0].to_string();
    let user = instruction.accounts()[2].to_string();

    let liquidity = match parse_pumpfun_amm_log(instruction) {
        Ok(PumpfunAmmLog::Withdraw(withdraw)) => Some(withdraw),
        _ => None,
    };

    let pool_base_token_reserves: Option<u64> =
        liquidity.as_ref().map(|x| x.pool_base_token_reserves);
    let pool_quote_token_reserves: Option<u64> =
        liquidity.as_ref().map(|x| x.pool_quote_token_reserves);

    let is_add = false;

    Ok(LiquidityEvent {
        pool,
        user,
        is_add,
        pool_base_token_reserves,
        pool_quote_token_reserves,
    })
}

fn parse_pumpfun_amm_log(instruction: &StructuredInstruction) -> Result<PumpfunAmmLog, Error> {
    let data = instruction
        .logs()
        .as_ref()
        .context("Failed to parse logs due to truncation")?
        .iter()
        .find_map(|log| match log {
            Log::Data(data_log) => data_log.data().ok(),
            _ => None,
        })
        .ok_or(anyhow!("Couldn't find data log."))?;
    PumpfunAmmLog::unpack(data.as_slice()).map_err(|x| anyhow!(x))
}
