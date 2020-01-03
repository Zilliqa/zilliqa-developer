# Private API Endpoint

Private API Endpoint is a way to provide exclusive API access to some critical users such as block explorers and exchanges. This doc describes how this can be done through ingress, service and pod labels in Kubernetes.

In this doc, mainnet is being used as the context for illustration. Therefore, the domain name `aws.zilliqa.com` is used throughout the doc.

- [Background](#background)
  - [Lookups](#lookups)
  - [Services](#services)
- [Creating New Private API Endpoint](#creating-new-private-api-endpoint)
- [Migrating Private API Endpoints](#migrating-private-api-endpoints)

## Background

### Lookups

The mainnet has 5 lookups, 15 level-2 lookups and up to 5 new lookups (aka seed nodes) for API services.

- 5 lookups are critical and not only doing transaction forwarding but also incremental DB uploading in some of them. Their API service are not opened.
- 15 level-2 lookups are intended for official API from Zilliqa (api.zilliqa.com), out of which 5 are constantly opened and serving traffic.
- 5 new lookups are reserved for exclusive API access for partners.

Level-2 lookups and new lookups are mostly identical and can be used interchangeably.

### Services

Technically, all the pods have the API port opened and the port is accessible internally through cluster IP. To enable the API for public access, one only needs to create services and select the pods using label selectors to serve the traffic.
To disable it, you can either delete the services or just change the label selector so that none of the lookups are selected.

For example, our public API `api.zilliqa.com` is backed by a service with the following label selectors

```yaml
spec:
  selector:
    testnet: mainnet-changi
    app: zilliqa
    type: level2lookup
    jsonrpc: opened
```

This will pick pods from level-2 lookup who has the value `opened` for the label `jsonrpc`.

## Creating New Private API Endpoint

Let's go through the steps for setting up private API endpoints for customers `foo` and `bar`.

During bootstrap, add `--private-api` option to set the name and ID for the customers.

```console
$ ./bootstrap.py mainnet-changi --private-api foo:abcde00001 --private-api bar:abcde00002
...(omitted)...
/testnet/mainnet-changi/manifest/foo.yaml (private api endpoint)
/testnet/mainnet-changi/manifest/bar.yaml (private api endpoint)
...(omitted)...
```

> **NOTE**: The value of `--private-api` is of the form `CUSTOMER:ID` where `ID` will be used in the url for the private API endpoint. For example, `foo:abcde00001` results in `abcde00001.example.com`.

After the network is up and newlookup is ready, create the resources for the customers.

```console
$ ./testnet.sh create foo bar
Creating foo bar ...
service "mainnet-changi-api-foo" created
ingress.extensions "mainnet-changi-api-foo" created
service "mainnet-changi-api-bar" created
ingress.extensions "mainnet-changi-api-bar" created
```

You should be able to see the new URLs soon with `./testnet.sh url`

```console
$ ./testnet.sh url
NAME                     HOSTS                                ADDRESS                           PORT    AGE
...(omitted)...
mainnet-changi-api-foo   abcde00001.mainnet.aws.zilliqa.com   xyz.us-west-2.elb.amazonaws.com   80      6d18h
mainnet-changi-api-bar   abcde00002.mainnet.aws.zilliqa.com   xyz.us-west-2.elb.amazonaws.com   80      6d18h
...(omitted)...
```

## Migrating Private API Endpoints

The migration happens when a new mainnet `mainnet-new-born` will replace the old one `mainnet-ancient`.

Assume we have the following private API endpoints to migrate.

| Customer | ID           | URL                                  |
|----------|--------------|--------------------------------------|
| `apple`  | `aaaaa00001` | `aaaaa00001.mainnet.aws.zilliqa.com` |
| `banana` | `bbbbb00001` | `bbbbb00001.mainnet.aws.zilliqa.com` |

First, when bootstrapping `mainnet-new-born`, following the same steps in [Creating New Private API Endpoint](#creating-new-private-api-endpoint) with the correct `--private-api` parameters. In this migration example, use `--private-api apple:aaaaa00001 --private-api banana:bbbbb00001`.

Now you should have two running mainnets, both with private API endpoints created.

To start directing the traffic to the new ones, delete the resources in old mainnet:

```console
$ ./testnet.sh delete apple banana
Deleting apple banana ...
service "mainnet-ancient-api-apple" deleted
ingress.extensions "mainnet-ancient-api-apple" deleted
service "mainnet-ancient-api-banana" deleted
ingress.extensions "mainnet-ancient-api-banana" deleted
```

Then in the new network, delete the DNS records so that new records will be created automatically. Before you confirm the deletion, please carefully check the records to be deleted from the temporary file (e.g., simple run `cat /tmp/route53-delete-record.v7UvCW` in another terminal).

```console
$ ./testnet.sh release-private-api
found hosted zone with ID: 'Z2M2***XHDH'
Confirm the deletion described in file '/tmp/route53-delete-record.v7UvCW'? [y/N]: y
{
    "ChangeInfo": {
        "Status": "PENDING",
        "Comment": "Deleting",
        "SubmittedAt": "2019-11-01T05:41:27.823Z",
        "Id": "/change/C7HCTP14J5T6R"
    }
}

Hints:

  To check if the records are created in new cluster, run 'kubectl --context new-born.cluster.z7a.xyz --namespace=kube-system logs  -l "app.kubernetes.io/name=external-dns"'
```

As the hint suggested, you can check the log of `external-dns` to see if the new records are created:

```console
$ kubectl --namespace=kube-system logs -l "app.kubernetes.io/name=external-dns" | grep CREATE
time="2019-11-01T05:41:33Z" level=info msg="Desired change: CREATE aaaaa00001.mainnet.aws.zilliqa.com A"
time="2019-11-01T05:41:33Z" level=info msg="Desired change: CREATE bbbbb00001.mainnet.aws.zilliqa.com A"
time="2019-11-01T05:41:33Z" level=info msg="Desired change: CREATE aaaaa00001.mainnet.aws.zilliqa.com TXT"
time="2019-11-01T05:41:33Z" level=info msg="Desired change: CREATE bbbbb00001.mainnet.aws.zilliqa.com TXT"
```
