create table if not exists ledger (
    id text not null,
    block bigint not null,
    account text not null,
    amount numeric not null,
    unique(id, block, account, amount)
);