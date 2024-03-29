CREATE TABLE IF NOT EXISTS microblocks (
    block BIGINT NOT NULL,
    offset_in_block BIGINT,
    shard_id BIGINT,
    header_version BIGINT NOT NULL,
    header_committee_hash bytea,
    header_prev_hash bytea,
    gas_limit BIGINT NOT NULL,
    rewards numeric(76, 38),
    --matching bq bignumeric
    prev_hash bytea,
    tx_root_hash bytea,
    miner_pubkey bytea,
    miner_addr_zil varchar(40),
    miner_addr_eth varchar(40),
    ds_block_num BIGINT,
    state_delta_hash bytea,
    tran_receipt_hash bytea,
    block_shard_id BIGINT,
    gas_used BIGINT NOT NULL,
    epoch_num BIGINT,
    num_txs BIGINT NOT NULL,
    blockhash bytea,
    --blockhash is unique
    timestamp BIGINT NOT NULL,
    cs1 bytea,
    b1 varchar(10),
    cs2 bytea,
    b2 varchar(10),
    imported_from text,
    --zq or eth
    eth_parent_hash bytea,
    eth_uncles_hash bytea,
    eth_state_root bytea,
    eth_extra_data bytea,
    eth_logs_bloom bytea,
    eth_difficulty BIGINT,
    eth_total_difficulty BIGINT,
    eth_nonce bytea,
    eth_base_fee_per_gas BIGINT,
    eth_withdrawals_root bytea,
    eth_withdrawals jsonb
) PARTITION BY RANGE (block);