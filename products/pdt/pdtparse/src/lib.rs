mod events;
mod ledger;

use crate::events::*;
use anyhow::{anyhow, Context, Result};
use futures::{
    stream::{Stream, StreamExt},
    TryStreamExt,
};
use ledger::LedgerInsertable;
use serde::{Deserialize, Serialize};
use serde_json::from_value;
use sqlx::{postgres::PgPoolOptions, query, query_as, types::JsonValue, PgPool};
use tokio::task::JoinSet;

#[derive(sqlx::FromRow, Debug)]
struct Event {
    txn_id: String,
    block: i64,
    params: Vec<EventParam>,
}

impl Event {
    fn find_param(&self, param: &str) -> Result<String> {
        self.params
            .iter()
            .find_map(|x| {
                if x.vname == param {
                    Some(x.value.clone())
                } else {
                    None
                }
            })
            .ok_or(anyhow!("could not find event parameter {param}"))
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct EventParam {
    #[serde(rename = "type")]
    param_type: String,
    value: String,
    vname: String,
}

// sqlx doesn't recognize that none of these columns will ever be null,
// so we need a struct to unwrap the options.
#[derive(Debug, Clone)]
struct RawEvent {
    id: Option<String>,
    block: Option<i64>,
    params: Option<JsonValue>,
    event: Option<String>,
}

fn get_raw_events<'a>(
    events: &'a Vec<String>,
    start_point: Option<(i64, String)>,
    client: &'a PgPool,
) -> impl Stream<Item = Result<RawEvent, sqlx::Error>> + 'a {
    match start_point {
        Some((start_block, max_id)) => {
            println!(
                "restarting from previous run, from.. {:?}",
                (start_block, &max_id)
            );
            query_as!(RawEvent,
                "select id, block, params, event from txn_with_events
                where event = ANY($1) and (block > $2 or (block = $2 and id > $3)) order by block asc, id asc", &events[..], start_block, &max_id
            )
            .fetch(client)
        }
        None => {
            println!("starting from the beginning!");
            query_as!(
                RawEvent,
                "select id, block, params, event from txn_with_events
                where event = ANY($1) order by block asc, id asc",
                &events[..]
            )
            .fetch(client)
        }
    }
}

pub async fn parse_zrc2(db_url: &str) -> Result<()> {
    const NUM_THREADS: usize = 50;
    let pool = PgPoolOptions::new()
        .max_connections(NUM_THREADS as u32)
        .connect(db_url)
        .await?;
    println!("finding where we left off..");
    let start_point = query!(
        "select max(id) as max_id,
            max(block)
        from ledger
        where block =(
                select max(block)
                from ledger
            )"
    )
    .fetch_one(&pool)
    .await?;
    let start_point = match start_point.max {
        None => None,
        Some(i) => Some((i - NUM_THREADS as i64, start_point.max_id.unwrap())),
    };
    let events = vec![
        "Minted".to_string(),
        "Burnt".to_string(),
        "TransferSuccess".to_string(),
        "TransferFromSuccess".to_string(),
        "OperatorSendSuccess".to_string(),
    ];
    let raw_events = get_raw_events(&events, start_point, &pool);
    let mut parsed_events = raw_events
        .map_err(|sqlx_error| anyhow!("{}", sqlx_error))
        .map_ok(|x| async {
            let y = x.clone();
            let params_json = x.params.ok_or(anyhow!("no event parameters found!"))?;
            let params: Vec<EventParam> = from_value(params_json)?;
            let block = x
                .block
                .expect("block should be not null on all transactions");
            let txn_id =
                x.id.expect("transaction id should be not null on all transactions");
            let event = x
                .event
                .expect("event should be not null on all transactions");

            println!("{event} found at block {}", block);
            let e = Event {
                txn_id,
                block,
                params,
            };
            match event.as_str() {
                "Minted" => MintedEvent::from_event(e).map(|x| ZRC2Event::Mint(x)),
                "Burnt" => BurntEvent::from_event(e).map(|x| ZRC2Event::Burn(x)),
                "TransferSuccess" => {
                    TransferSuccessEvent::from_event(e).map(|x| ZRC2Event::Transfer(x))
                }
                "TransferFromSuccess" => {
                    TransferFromSuccess::from_event(e).map(|x| ZRC2Event::TransferFrom(x))
                }
                "OperatorSendSuccess" => {
                    OperatorSendSuccess::from_event(e).map(|x| ZRC2Event::OperatorSend(x))
                }
                _ => panic!("query returned an invalid event!"),
            }
            .with_context(|| format!("failed to parse event params {:#?} into event {event}", y))
        })
        .try_buffered(NUM_THREADS);
    let mut jobs = JoinSet::new();
    while let Some(event) = parsed_events.next().await {
        let my_pool = pool.clone();
        jobs.spawn(async move {
            match event {
                Ok(event) => {
                    match insert_into_ledger(&event, &my_pool).await {
                        Ok(_) => println!("inserted"),
                        Err(err) => {
                            eprintln!("unable to insert record with error {:?} {:#?}", err, event);
                        }
                    };
                }
                Err(err) => {
                    eprintln!("{:?}", err)
                }
            }
        });
    }
    println!("waiting for jobs to complete..");
    while let Some(_) = jobs.join_next().await {
        continue;
    }
    Ok(())
}
