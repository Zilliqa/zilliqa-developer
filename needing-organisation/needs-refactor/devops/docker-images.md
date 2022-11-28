# Docker Images

This document describes how to access public and private docker images.

- [Public Images in Docker Cloud](#public-images-in-docker-cloud)
  - [Get your account ready](#get-your-account-ready)
  - [Log in to Docker Cloud](#log-in-to-docker-cloud)
- [Private Images in AWS ECR](#private-images-in-aws-ecr)
  - [Get the AWS credentials ready](#get-the-aws-credentials-ready)
  - [Log in to ECR](#log-in-to-ecr)
  - [Create commit-specific Zilliqa image](#create-commit-specific-zilliqa-image)

## Public Images in Docker Cloud

We use [Docker Cloud](https://cloud.docker.com/) as a public registry to store our images for public usage.

### Get your account ready

In order to push any images to image repositories on Docker Cloud, you will need an account that also joins Zilliqa organization.

### Log in to Docker Cloud

Just run `docker login` and use your Docker Cloud account username and password to login.

```console
$ docker login
Login with your Docker ID to push and pull images from Docker Hub. If you don't have a Docker ID, head over to https://hub.docker.com to create one.
Username: myusername
Password: ****
..
```

After login, you can run commands like `docker push` to conduct write operations. Note that you don't need login when you only need read-only operations such as `docker pull`.

## Private Images in AWS ECR

We use [AWS Elastic Container Registry](https://aws.amazon.com/ecr/) as a private registry to store our images for different applications, such as the Zilliqa, Scilla, etc. To access ECR, you will need to log in through `docker login` command, simliar to [**Public Images in Docker Cloud**](#public-images-in-docker-cloud). With AWS CLI tool, this could be done easily as you can see in [**Log in to ECR**](#log-in-to-ecr).

### Get the AWS credentials ready

Export you AWS credentials in the current environment. Depending on the operation you are doing, you will need different set of credentials. One common way to achieve this is to export these variables in your terminal.

```bash
export AWS_ACCESS_KEY_ID="****************"
export AWS_SECRET_ACCESS_KEY="****************"
export AWS_SESSION_TOKEN="****************"
```

### Log in to ECR

First, make sure you already [get the AWS credentials ready](#get-the-aws-credentials-ready).

Then, run the command to log in the registry.

```bash
eval $(aws ecr get-login --no-include-email --region us-west-2)
```

Now you should be able to use `docker pull` or `docker push` (write permission needed) to interact with ECR.

### Create commit-specific Zilliqa image

First, make sure you already [get the AWS credentials ready](#get-the-aws-credentials-ready).

Makse sure you have the Zilliqa repository cloned.

```bash
git clone https://github.com/Zilliqa/Zilliqa.git
cd Zilliqa
```

Checkout to the commit you want to make image from.

```bash
git checkout f7d27f6
```

Make the image and check if it is successful.

```console
$ ./scripts/make_image.sh
Making images using commit 'f7d27f6c6ae8790c1f2fca7de10ce1cb85c776fb'
Docker version 18.09.2, build 6247962
aws-cli/1.16.100 Python/3.7.3 Darwin/18.6.0 botocore/1.12.90
WARNING! Using --password via the CLI is insecure. Use --password-stdin.
Login Succeeded
cat dev/Dockerfile dev/Dockerfile.k8s | docker build -t zilliqa:f7d27f6 \
        --build-arg COMMIT=f7d27f6 --build-arg EXTRA_CMAKE_ARGS="" -
Sending build context to Docker daemon   5.12kB
Step 1/14 : ARG BASE=zilliqa/scilla:v0.3.0
...
...
...
Successfully built fb91a6b40708
Successfully tagged zilliqa:f7d27f6
The push refers to repository [***********.dkr.ecr.us-west-2.amazonaws.com/zilliqa]
df66e59bc712: Pushed
e2305fce51f4: Pushed
cbba671db3df: Pushed
1bcc5627f77d: Layer already exists
1db4d4b23441: Layer already exists
22388210c94b: Layer already exists
d908c11b1a82: Layer already exists
7ccfaa7554e3: Layer already exists
89ec57aea3bf: Layer already exists
a0c1e01578b7: Layer already exists
f7d27f6: digest: sha256:155bf41d515e97a1878c3ff59d5c2e652e48fec199173e9f8ed3a607994d9bb9 size: 2424
```

> Note: If you are see any permission error, you might need to check if your current credentials have the write permission to ECR. Contact the admin if you don't have the required permissions.
