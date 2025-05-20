use anyhow::{anyhow, Error};

use pumpswap::instructions_cpi::BuyCpiInstruction;
use pumpswap::instructions_cpi::CreatePoolCpiInstruction;
use pumpswap::instructions_cpi::DepositCpiInstruction;
use pumpswap::instructions_cpi::SellCpiInstruction;
use pumpswap::instructions_cpi::WithdrawCpiInstruction;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;

pub mod pumpswap;
use pumpswap::constants::PUMPSWAP_PROGRAM_ID;
use pumpswap::instruction::PumpswapInstruction;
use pumpswap::instructions_cpi::PumpswapCpiInstruction;

use substreams_solana_utils as utils;
use utils::instruction::{
    get_structured_instructions, StructuredInstruction, StructuredInstructions,
};
use utils::transaction::{get_context, TransactionContext};

pub mod pb;
use pb::pumpswap::pumpswap_event::Event;
use pb::pumpswap::*;

#[substreams::handlers::map]
fn pumpswap_events(block: Block) -> Result<PumpswapBlockEvents, Error> {
    let transactions = parse_block(&block);
    Ok(PumpswapBlockEvents { transactions })
}

pub fn parse_block(block: &Block) -> Vec<PumpswapTransactionEvents> {
    let mut block_events: Vec<PumpswapTransactionEvents> = Vec::new();
    for transaction in block.transactions.iter() {
        if let Ok(events) = parse_transaction(transaction) {
            if !events.is_empty() {
                block_events.push(PumpswapTransactionEvents {
                    signature: utils::transaction::get_signature(&transaction),
                    events,
                });
            }
        }
    }
    block_events
}

pub fn parse_transaction(transaction: &ConfirmedTransaction) -> Result<Vec<PumpswapEvent>, Error> {
    if let Some(_) = transaction.meta.as_ref().unwrap().err {
        return Ok(Vec::new());
    }

    let mut events: Vec<PumpswapEvent> = Vec::new();

    let mut context = get_context(transaction)?;
    let instructions = get_structured_instructions(transaction)?;
    for instruction in instructions.flattened().iter() {
        context.update_balance(&instruction.instruction);

        if instruction.program_id() != PUMPSWAP_PROGRAM_ID {
            continue;
        }

        // substreams::log::println(format!("txn: {:?}", transaction.id()));

        match parse_instruction(&instruction, &context) {
            Ok(Some(event)) => events.push(PumpswapEvent { event: Some(event) }),
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
    if instruction.program_id() != PUMPSWAP_PROGRAM_ID {
        return Err(anyhow!("Not a Pumpfun Amm instruction."));
    }

    let unpacked = PumpswapInstruction::unpack(instruction.data()).map_err(|x| anyhow!(x))?;
    match unpacked {
        PumpswapInstruction::CreatePool(create) => Ok(Some(Event::CreatePool(
            _parse_create_pool_instruction(instruction, context, create)?,
        ))),
        PumpswapInstruction::Buy(buy) => Ok(Some(Event::Swap(_parse_buy_instruction(
            instruction,
            context,
            buy,
        )?))),
        PumpswapInstruction::Sell(sell) => Ok(Some(Event::Swap(_parse_sell_instruction(
            instruction,
            context,
            sell,
        )?))),
        PumpswapInstruction::Deposit => Ok(Some(Event::Liquidity(_parse_deposit_instruction(
            instruction,
            context,
        )?))),
        PumpswapInstruction::Withdraw => Ok(Some(Event::Liquidity(_parse_withdraw_instruction(
            instruction,
            context,
        )?))),
        _ => Ok(None),
    }
}

fn _parse_create_pool_instruction(
    instruction: &StructuredInstruction,
    _context: &TransactionContext,
    _create: pumpswap::instruction::CreatePoolInstruction,
) -> Result<CreatePoolEvent, Error> {
    let pool_event: CreatePoolCpiInstruction = instruction
        .inner_instructions()
        .iter()
        .find_map(
            |inner_ix| match PumpswapCpiInstruction::unpack(inner_ix.data()).unwrap() {
                PumpswapCpiInstruction::CreatePoolCpi(pool_event) => Some(pool_event),
                _ => None,
            },
        )
        .unwrap();

    let pool = pool_event.pool.to_string();
    let creator = pool_event.creator.to_string();
    let coin_creator = "11111111111111111111111111111111".to_string();
    let base_mint = pool_event.base_mint.to_string();
    let quote_mint = pool_event.quote_mint.to_string();
    let base_mint_decimals = pool_event.base_mint_decimals as u32;
    let quote_mint_decimals = pool_event.quote_mint_decimals as u32;
    let base_amount_in = Some(pool_event.base_amount_in);
    let quote_amount_in = Some(pool_event.quote_amount_in);
    let pool_base_amount = Some(pool_event.base_amount_in);
    let pool_quote_amount = Some(pool_event.pool_quote_amount);
    let timestamp = pool_event.timestamp;

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
    _context: &TransactionContext,
    _buy: pumpswap::instruction::BuyInstruction,
) -> Result<SwapEvent, Error> {
    let pool = instruction.accounts()[0].to_string();
    let mint = instruction.accounts()[3].to_string();
    let user = instruction.accounts()[1].to_string();
    let bonding_curve = "".to_string();

    let trade: BuyCpiInstruction = instruction
        .inner_instructions()
        .iter()
        .find_map(
            |inner_ix| match PumpswapCpiInstruction::unpack(inner_ix.data()).unwrap() {
                PumpswapCpiInstruction::BuyCpi(trade) => Some(trade),
                _ => None,
            },
        )
        .unwrap();

    let sol_amount: Option<u64> = Some(trade.quote_amount_in);
    let token_amount: u64 = trade.base_amount_out;

    let virtual_sol_reserves: Option<u64> = Some(0);
    let virtual_token_reserves: Option<u64> = Some(0);
    let real_sol_reserves: Option<u64> = Some(trade.pool_quote_token_reserves);
    let real_token_reserves: Option<u64> = Some(trade.pool_base_token_reserves);
    let protocol_fee: Option<u64> = Some(trade.protocol_fee);
    let coin_creator_fee: Option<u64> = Some(0);
    let timestamp: i64 = trade.timestamp;
    let user_token_pre_balance: Option<u64> = Some(0);
    let direction = "token".to_string();
    let is_buy = true;
    let complete = "".to_string();

    Ok(SwapEvent {
        user,
        mint,
        bonding_curve,
        pool,
        sol_amount,
        token_amount,
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
        complete,
    })
}

fn _parse_sell_instruction<'a>(
    instruction: &StructuredInstruction<'a>,
    _context: &TransactionContext,
    _sell: pumpswap::instruction::SellInstruction,
) -> Result<SwapEvent, Error> {
    let pool = instruction.accounts()[0].to_string();
    let mint = instruction.accounts()[3].to_string(); // pool
    let user = instruction.accounts()[1].to_string();
    let bonding_curve = "".to_string();

    let trade: SellCpiInstruction = instruction
        .inner_instructions()
        .iter()
        .find_map(
            |inner_ix| match PumpswapCpiInstruction::unpack(inner_ix.data()).unwrap() {
                PumpswapCpiInstruction::SellCpi(trade) => Some(trade),
                _ => None,
            },
        )
        .unwrap();

    let sol_amount: Option<u64> = Some(trade.quote_amount_out);
    let token_amount: u64 = trade.base_amount_in;

    let virtual_sol_reserves: Option<u64> = Some(0);
    let virtual_token_reserves: Option<u64> = Some(0);
    let real_sol_reserves: Option<u64> = Some(trade.pool_quote_token_reserves);
    let real_token_reserves: Option<u64> = Some(trade.pool_base_token_reserves);
    let protocol_fee: Option<u64> = Some(trade.protocol_fee);
    let coin_creator_fee: Option<u64> = Some(0);
    let timestamp: i64 = trade.timestamp;
    let user_token_pre_balance: Option<u64> = Some(0);
    let direction = "sol".to_string();
    let is_buy = false;
    let complete = "".to_string();

    Ok(SwapEvent {
        user,
        mint,
        bonding_curve,
        pool,
        sol_amount,
        token_amount,
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
        complete,
    })
}

fn _parse_deposit_instruction(
    instruction: &StructuredInstruction,
    _context: &TransactionContext,
) -> Result<LiquidityEvent, Error> {
    let pool = instruction.accounts()[0].to_string();
    let user = instruction.accounts()[2].to_string();

    let liquidity: DepositCpiInstruction = instruction
        .inner_instructions()
        .iter()
        .find_map(
            |inner_ix| match PumpswapCpiInstruction::unpack(inner_ix.data()).unwrap() {
                PumpswapCpiInstruction::DepositCpi(liquidity) => Some(liquidity),
                _ => None,
            },
        )
        .unwrap();

    let pool_base_token_reserves: Option<u64> = Some(liquidity.pool_base_token_reserves);
    let pool_quote_token_reserves: Option<u64> = Some(liquidity.pool_quote_token_reserves);
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

    let liquidity: WithdrawCpiInstruction = instruction
        .inner_instructions()
        .iter()
        .find_map(
            |inner_ix| match PumpswapCpiInstruction::unpack(inner_ix.data()).unwrap() {
                PumpswapCpiInstruction::WithdrawCpi(liquidity) => Some(liquidity),
                _ => None,
            },
        )
        .unwrap();

    let pool_base_token_reserves: Option<u64> = Some(liquidity.pool_base_token_reserves);
    let pool_quote_token_reserves: Option<u64> = Some(liquidity.pool_quote_token_reserves);
    let is_add = false;

    Ok(LiquidityEvent {
        pool,
        user,
        is_add,
        pool_base_token_reserves,
        pool_quote_token_reserves,
    })
}

// fn parse_buy_cpi_instruction(
//     instruction: &StructuredInstruction,
// ) -> Result<BuyCpiInstruction, anyhow::Error> {
//     let buy_instruction = instruction
//         .inner_instructions()
//         .iter()
//         .find_map(|inner_ix| {
//             // Optional: Check data length to avoid invalid deserialization
//             if inner_ix.data().len() < 8 {
//                 return None;
//             }
//             PumpswapCpiInstruction::unpack(inner_ix.data()).ok()
//         })
//         .ok_or(anyhow!("Couldn't find data BuyCpiInstruction."))?;

//     match buy_instruction {
//         PumpswapCpiInstruction::BuyCpi(trade) => Ok(trade),
//         _ => Err(anyhow!(
//             "Found instruction but it was not a BuyCpiInstruction"
//         )),
//     }
// }
