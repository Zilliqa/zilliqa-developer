# DNS for Public Testnet

Creating or updating the DNS records is one critical step to turn the testnet into production state.
This doc show what are the steps to coordinate the change on testnet and your service provider.

> **Do note that any change in the ingress or service provider will IMMEDIATELY TAKE EFFECT and will have impact on your production system. Please read through the entire doc and exercise first before operating in production.**

<!-- TOC orderedList:true -->

1. [Listing down the DNS CNAME mapping](#listing-down-the-dns-cname-mapping)
2. [Updating the testnet ingress](#updating-the-testnet-ingress)
    1. [Checking the existing ingress rules](#checking-the-existing-ingress-rules)
    2. [Updating the ingress rules](#updating-the-ingress-rules)
3. [Updating DNS records in the provider](#updating-dns-records-in-the-provider)
4. [Verifying new DNS records](#verifying-new-dns-records)

<!-- /TOC -->

## Listing down the DNS CNAME mapping

First, understand what change you will be making and list them down in a table.

| No. | Private URL                         | Public URL                 |
|-----|-------------------------------------|----------------------------|
| 1   | `community441-api.aws.z7a.xyz`      | `dev-api.zilliqa.com`      |
| 2   | `community441-explorer.aws.z7a.xyz` | `dev-explorer.zilliqa.com` |

## Updating the testnet ingress

### Checking the existing ingress rules

Let's take a look at the default ingress by running `./testnet.sh url` or `kubectl get ingress`.

```console
$ ./testnet.sh url
NAME                    HOSTS                                                    ADDRESS                                                                   PORTS     AGE
community441-api        community441-api.aws.z7a.xyz                             aebb03c79250e11e9a41506310b6001d-1740577342.us-west-2.elb.amazonaws.com   80        35m
community441-explorer   community441-explorer.aws.z7a.xyz,explorer.zilliqa.com   aebb03c79250e11e9a41506310b6001d-1740577342.us-west-2.elb.amazonaws.com   80        35m
community441-l2api      community441-l2api.aws.z7a.xyz,api.zilliqa.com           aebb03c79250e11e9a41506310b6001d-1740577342.us-west-2.elb.amazonaws.com   80        35m
community441-newapi     community441-newapi.aws.z7a.xyz                          aebb03c79250e11e9a41506310b6001d-1740577342.us-west-2.elb.amazonaws.com   80        35m
community441-origin     community441-origin.aws.z7a.xyz                          aebb03c79250e11e9a41506310b6001d-1740577342.us-west-2.elb.amazonaws.com   80        35m
```

Run `./testnet.sh url edit community441-api` and check the `.spec.rules` section in the opened editor.

```yaml
spec:
  rules:
  - host: community441-api.aws.z7a.xyz  # The private DNS name
    http:
      paths:
      - backend:
          serviceName: community441-api # The service name
          servicePort: 80
```

There is one rule currently, which establishes a mapping from the private DNS name to the service name.

### Updating the ingress rules

Now, we add a new rule for `dev-api.zilliqa.com` in the editor.

```yaml
spec:
  rules:
  - host: community441-api.aws.z7a.xyz  # The private DNS name
    http:
      paths:
      - backend:
          serviceName: community441-api # The service name
          servicePort: 80
  # This is the new rule we inserted. Make sure the indentation is correct
  - host: dev-api.zilliqa.com  # The public DNS name
    http:
      paths:
      - backend:
          serviceName: community441-api # The service name
          servicePort: 80
```

Save the change and exit the editor. You will see a message indicating the change.

```console
$ ./testnet.sh url edit community441-api
ingress.extensions "community441-api" edited
```

You will also see the new entry in the `./testnet.sh url`.

```fundamental
NAME                    HOSTS                                                        ADDRESS                                                                   PORTS     AGE
community441-api        community441-api.aws.z7a.xyz,dev-api.zilliqa.com             aebb03c79250e11e9a41506310b6001d-1740577342.us-west-2.elb.amazonaws.com   80        1h
```

Do the same change for other mappings you [listed before](#listing-down-the-dns-cname-mapping).

> **Important Note**
>
> If you are doing green-blue deployment (i.e., upgrade or recover in a new testnet), you need to undo the mapping in the testnet to be retired first. Any removal of the ingress rules will take effect instantly so you may want to do the removing and inserting in one transaction through `kubectl edit ingress`.

## Updating DNS records in the provider

Go to your DNS provider and create new records or update the existing one about the CNAME mapping.

For example, cname `dev-explorer.zilliqa.com` to `community441-explorer.aws.z7a.xyz`.

## Verifying new DNS records

Use any machine and try to access the updated DNS. The caching on browser might affect your testing so do take care of that.
