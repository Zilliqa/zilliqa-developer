# Mining with NiceHash

## NiceHash

NiceHash is a hash power broker with integrated marketplace that connects sellers of hashing power (miners) with buyers of hashing power using the sharing economy approach.  

Buyers rent computing (hashing) power through NiceHash's online platform. Sellers provide hashing power by connecting to the NiceHash marketplace with NiceHash's own mining software.

## Register account and mining pool on NiceHash

Register an account on NiceHash website [NiceHash](https://www.nicehash.com)
Deposite some Bitcoin to the address on NiceHash, generate and save the API key and API secret key.  
Register a mining pool on the NiceHash hashpower market place. The address and port will be used in next step.
The pool id can be retrieved using tool [NiceHash Python Example](https://github.com/nicehash/rest-clients-demo/tree/master/python).

## Configure the mining proxy and restart it

Launch an AWS instance to run the mining proxy following instructions at [Zillqa-Mining-Proxy](https://github.com/DurianStallSingapore/Zilliqa-Mining-Proxy). Before start, need to checkout branch `test/nicehash_newapi`. Open the `pool.conf` file, input the organisation_id, api_key, api_secret and pool_id.

```yaml
nicehash:
  enabled: true
  host: "https://api2.nicehash.com"
  organisation_id: "your organization id"
  api_key: "your api key"
  api_secret: "your api secret"
  location: "USA"
  algo: "DAGGERHASHIMOTO"
  pool_id: "your registered pool id on nice hash"
  place_order_block: 95
```

Now can start the the mongo DB service and mining proxy using following command.

```bash
sudo service mongod start
python3.7 start.py
```

## Start Zilliqa shard nodes

Can use marketplace AMI to start some zilliqa nodes, change the `REMOTE_MINE` to true and `MINING_PROXY_URL` to the mining proxy running last step.

```xml
<REMOTE_MINE>true</REMOTE_MINE>
<MINING_PROXY_URL>http://52.220.146.17:4202/api</MINING_PROXY_URL>
```