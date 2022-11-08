# Local Storage

## Lookup Nodes

1. Lookups are full nodes hence they store all the data. Transactions, blocks, microblocks, state, and state deltas (of previous 10 DS epochs) are stored by lookups.
1. Lookup persistence (specifically `lookup-0`) is also uploaded to AWS S3 for synchronization of nodes.

## DS and Shard Nodes

1. These nodes store DS Blocks, Tx Blocks, current state.
1. The shard nodes also store processed txns of that DS epoch in temporary storage. These are uploaded to S3 for backup.
1. The DS nodes also store all the microblocks as they receive them from the shard nodes.