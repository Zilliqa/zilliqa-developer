# Persistence Data Transformer (PDT)

PDT allows you to understand and ask questions about Zilliqa 1 persistence data
by transforming it into one of BigQuery or PostgreSQL.

## Quickstart

You'll first want to download and unpack the persistence archives. Run:

```bash
cargo run -- --download-dir=<your_download_dir> --unpack-dir=<your_unpacking_dir> download
```

Be aware that quite a bit of free disk space is needed.

If you'd like to use BigQuery, you'll need to set up your service account key
file. Then run

```bash
cargo run -- --download-dir=<your_download_dir> --unpack-dir=<your_unpacking_dir> bqmulti --dataset-id=<your_id> --nr-threads=<n> --service-account-key-file=<your_key_file> --batch-blocks=<n>
```

For Postgres, make sure your database is created and run

```bash
cargo run -- --download-dir=<your_download_dir> --unpack-dir=<your_unpacking_dir> --postgres-url=<your_postgres_url> psqlmulti --nr-threads=<n> --batch-blocks=<n> --partition-size=10000
```

All our import operations are multithreaded. Each thread will create a duplicate
of the unpacked archive, so ensure you have enough space for the number of
threads you've selected. We import blocks and transactions in batches, so if PDT
stops running your progress will be saved and will restart gracefully. Larger
batches means less network roundtrips but less safety. Bear in mind that
BigQuery has size and number limits on row insertion requests, so it can't
insert more than ~500 rows at a time anyways.

Our Postgres client partitions microblocks and transactions by block number in
partitions of the size you set, for BigQuery you can ask it to do the same.

## Determining whether a transaction is EVM or not

This is done (`AccountStore.cpp`) by a truly horrid set of heuristics:

- If data is not empty and the destination is not the null address and code is empty, it's a contract call.
- If code is not empty and it's to the null address, it's a contract creation.
- If data is empty and it's not to the null address, and code is also empty, it's NON_CONTRACT
- Otherwise error.

Now:

- If this is a contract creation, and the code begins 'E' 'V' 'M' (hopefully no legal scilla ever starts this way!) it is an EVM transaction.
- Otherwise, look at the contents of the to address, if it exists.
- If the to address has code which is EVM, this transaction is EVM, otherwise it isn't.

## Envrionment variables — for development only

The url used to connect to the Postgres database is **always passed in through the
command line**. However, sqlx's compile-time query-checking is much more ergonomic
with the `DATABASE_URL` environment variable set in a `.env`.

This isn't required — the compile-time query checks are committed to `.sqlx`, so
development works perfectly fine without an active database. If you have to
update any of the queries, though, you'll need to rerun the checks, which
requires a database with the correct schemas and tables etc. You'll then have to run
`cargo sqlx prepare --database-url <url> --workspace`, which will check all the
queries against that database url.

## Permissions needed in bq

- Data Owner

## Validating queries

```sql
-- Check that there are no duplicates.
SELECT id FROM `<your-dataset-id>` GROUP BY id HAVING COUNT(1) > 1 LIMIT 1000
```

## Views

```sql
SELECT DIV(block,10000) AS blk, COUNT(DISTINCT(from_addr)) AS addr, COUNT(*) AS txns, SUM(amount) AS amount FROM `<your-dataset-id>` GROUP BY DIV(block, 10000)
```
