# Private API Endpoint

Private API Endpoint is a way to provide exclusive API access to some critical users such as block explorers and exchanges. This doc describes how this can be done through ingress, service and pod labels in Kubernetes.

In this doc, mainnet is being used as the context for illustration. Therefore, the domain name `aws.zilliqa.com` is used throughout the doc.

- [Background](#background)
  - [Lookups](#lookups)
  - [Services](#services)
- [Steps](#steps)
  - [Create manifest file](#create-manifest-file)
  - [Label the lookup](#label-the-lookup)
  - [Open the lookup](#open-the-lookup)

## Background

### Lookups

The mainnet has 5 lookups, 15 level-2 lookups and up to 10 new lookups (aka seed nodes) for API services.

- 5 lookups are critical and not only doing transaction forwarding but also incremental DB uploading in some of them. Their API service are not opened.
- 15 level-2 lookups are intended for official API from Zilliqa (api.zilliqa.com), out of which 5 are constantly opened and serving traffic.
- 10 new lookups are reserved for exclusive API access for partners.

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

## Steps

Let's go through the steps for setting up a private API endpoint for a user `customer1`.

### Create manifest file

In the testnet `manifest` folder, create a file `customer1.yaml` with the following content:

```yaml
apiVersion: v1
kind: Service
metadata:
  name: mainnet-changi-api-customer1        # 1. service name
  labels:
    testnet: mainnet-changi
spec:
  type: ClusterIP
  ports:
  - port: 80
    targetPort: 4201
    name: zilliqa-api
  selector:
    testnet: mainnet-changi
    app: zilliqa
    type: newlookup                         # 2. lookup pod type
    customer: customer1                     # 3. customer label
    jsonrpc: opened
---
apiVersion: extensions/v1beta1
kind: Ingress
metadata:
  name: mainnet-changi-api-customer1        # 4. ingress name
  labels:
    testnet: mainnet-changi
    external: "true"
  annotations:
    kubernetes.io/ingress.class: ingress-aws-zilliqa-com
    ingress.kubernetes.io/force-ssl-redirect: "true"
spec:
  rules:
  - host: f78a53621d.aws.zilliqa.com        # 5. random domain name ended with aws.zilliqa.com
    http:
      paths:
      - backend:
          serviceName: mainnet-changi-api-customer1     # 6. service name
          servicePort: 80
```

Customize the numbered fields with your preference. Do note the following requirements:

| No. | Item               | Requirement                                                                                               |
|-----|--------------------|-----------------------------------------------------------------------------------------------------------|
| 1   | service name       | A unique service name in the cluster, recommended to indicate the customer name for easier management.    |
| 2   | lookup type label  | A string that is one of `lookup`, `level2lookup` and `newlookup`. For private endpoints, use `newlookup`. |
| 2   | customer label     | A unique customer name as label selector. The label value will be used later.                             |
| 3   | ingress name       | A unique ingress name in the cluster, recommended to indicate the customer name for easier management.    |
| 4   | random domain name | A long random domain name with at least 10 characters, ended with `aws.zilliqa.com`.                      |
| 5   | service name       | A string same as the service name in field #1.                                                            |

Create the resources declared in the manifest through

```console
$ ./testnet.sh create customer1
Creating customer1 ...
service "mainnet-changi-api-customer1" created
ingress.extensions "mainnet-changi-api-customer1" created
```

Some other operations you might find useful:

- `./testnet.sh replace customer1`: Update the cluster after your local change in the manifest `manifest/customer1.yaml`.
- `./testnet.sh delete customer1`: Remove the resources declared in the manifest `manifest/customer1.yaml`.

### Label the lookup

As you may have seen, the labels `type` and `customer` are used to identify the pods connected to different services. Also, the label `jsonrpc` is used to open or close the pods. By default, all the lookups already have `type` and `jsonrpc` pods. We only need to add new label `customer` while we are labelling the lookup pods. Since we have decided to use `newlookup` for private API endpoints, we simply label the `newlookup` pods with `customer`.

First, let's use `kubectl get pods --show-labels` to check the existing labels.

```console
$ kubectl get pods --show-labels -l type=newlookup
mainnet-changi-newlookup-0   1/1       Running   0          13d       app=zilliqa,controller-revision-hash=mainnet-changi-newlookup-5cfddc5bc5,jsonrpc=opened,statefulset.kubernetes.io/pod-name=mainnet-changi-newlookup-0,testnet=mainnet-changi,type=newlookup
mainnet-changi-newlookup-1   1/1       Running   0          13d       app=zilliqa,controller-revision-hash=mainnet-changi-newlookup-5cfddc5bc5,jsonrpc=opened,statefulset.kubernetes.io/pod-name=mainnet-changi-newlookup-1,testnet=mainnet-changi,type=newlookup
mainnet-changi-newlookup-2   1/1       Running   0          13d       app=zilliqa,controller-revision-hash=mainnet-changi-newlookup-5cfddc5bc5,jsonrpc=opened,statefulset.kubernetes.io/pod-name=mainnet-changi-newlookup-2,testnet=mainnet-changi,type=newlookup
mainnet-changi-newlookup-3   1/1       Running   0          13d       app=zilliqa,controller-revision-hash=mainnet-changi-newlookup-5cfddc5bc5,jsonrpc=opened,statefulset.kubernetes.io/pod-name=mainnet-changi-newlookup-3,testnet=mainnet-changi,type=newlookup
mainnet-changi-newlookup-4   1/1       Running   0          13d       app=zilliqa,controller-revision-hash=mainnet-changi-newlookup-5cfddc5bc5,jsonrpc=opened,statefulset.kubernetes.io/pod-name=mainnet-changi-newlookup-4,testnet=mainnet-changi,type=newlookup
mainnet-changi-newlookup-5   1/1       Running   0          13d       app=zilliqa,controller-revision-hash=mainnet-changi-newlookup-5cfddc5bc5,jsonrpc=opened,statefulset.kubernetes.io/pod-name=mainnet-changi-newlookup-5,testnet=mainnet-changi,type=newlookup
mainnet-changi-newlookup-6   1/1       Running   0          13d       app=zilliqa,controller-revision-hash=mainnet-changi-newlookup-5cfddc5bc5,jsonrpc=opened,statefulset.kubernetes.io/pod-name=mainnet-changi-newlookup-6,testnet=mainnet-changi,type=newlookup
mainnet-changi-newlookup-7   1/1       Running   0          13d       app=zilliqa,controller-revision-hash=mainnet-changi-newlookup-5cfddc5bc5,jsonrpc=opened,statefulset.kubernetes.io/pod-name=mainnet-changi-newlookup-7,testnet=mainnet-changi,type=newlookup
mainnet-changi-newlookup-8   1/1       Running   0          13d       app=zilliqa,controller-revision-hash=mainnet-changi-newlookup-5cfddc5bc5,jsonrpc=opened,statefulset.kubernetes.io/pod-name=mainnet-changi-newlookup-8,testnet=mainnet-changi,type=newlookup
mainnet-changi-newlookup-9   1/1       Running   0          13d       app=zilliqa,controller-revision-hash=mainnet-changi-newlookup-5cfddc5bc5,jsonrpc=opened,statefulset.kubernetes.io/pod-name=mainnet-changi-newlookup-9,testnet=mainnet-changi,type=newlookup
```

Then, we select the pods without existing `customer` labels, e.g., `mainnet-changi-newlookup-0` and `mainnet-changi-newlookup-1` and label them.

```console
$ kubectl label pods mainnet-changi-newlookup-0 mainnet-changi-newlookup-1 customer=customer1
pod "mainnet-changi-newlookup-0" labeled
pod "mainnet-changi-newlookup-1" labeled
```

Now if you run `kubectl get pods --show-labels` again you will see the new labels on the pods we selected.

```console
$ kubectl get pods --show-labels -l type=newlookup
NAME                         READY     STATUS    RESTARTS   AGE       LABELS
mainnet-changi-newlookup-0   1/1       Running   0          13d       app=zilliqa,controller-revision-hash=mainnet-changi-newlookup-5cfddc5bc5,customer=binance,jsonrpc=opened,statefulset.kubernetes.io/pod-name=mainnet-changi-newlookup-0,testnet=mainnet-changi,type=newlookup
mainnet-changi-newlookup-1   1/1       Running   0          13d       app=zilliqa,controller-revision-hash=mainnet-changi-newlookup-5cfddc5bc5,customer=binance,jsonrpc=opened,statefulset.kubernetes.io/pod-name=mainnet-changi-newlookup-1,testnet=mainnet-changi,type=newlookup
...
```

### Open the lookup

If you selected lookups are not opened yet.

```console
$ ./testnet.sh jsonrpc newlookup status
mainnet-changi-newlookup-0 closed # 0 and 1 are closed
mainnet-changi-newlookup-1 closed
mainnet-changi-newlookup-2 opened
mainnet-changi-newlookup-3 opened
mainnet-changi-newlookup-4 opened
mainnet-changi-newlookup-5 opened
mainnet-changi-newlookup-6 opened
mainnet-changi-newlookup-7 opened
mainnet-changi-newlookup-8 opened
mainnet-changi-newlookup-9 opened
```

Just open it.

```console
$ ./testnet.sh jsonrpc newlookup open 0 1
pod "mainnet-changi-newlookup-0" labeled
pod "mainnet-changi-newlookup-1" labeled
```
