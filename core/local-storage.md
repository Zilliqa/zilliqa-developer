#LOOKUP NODES

1. Lookups are full nodes hence they store all the data. Transactions, Blocks, microblocks State and state 
deltas (of prev 10 ds epochs) are stored by lookups.
2. Lookup persistences (specifically lookup-0) persistence is also uploaded to s3
for synchorinization of nodes.

#SHARD NODES (including DS)

1. These nodes store DS Blocks, Tx Blocks, current state.
2. The shard nodes also store processed txns of that ds epoch in temporary storage.
These are uploaded to s3 for backup
3. The DS nodes also store all the microblocks as they recieve it from the shard nodes.