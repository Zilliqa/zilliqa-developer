
# Create AWS Marketplace Product

1. Launch an aws instance with OS ubuntu 16.04, using ssh to connect to the instance.
1. Install Docker CE for Ubuntu 16.04 on the instance by following instructions [here](https://docs.docker.com/install/linux/docker-ce/ubuntu/).
1. Make a new directory `join_mainnet` and change directory to it.

      ```shell
      mkdir join_mainnet && cd join_mainnet
      ```

1. Get the joining configuration files.

      ```shell
      wget https://mainnet-join.zilliqa.com/configuration.tar.gz
      tar zxvf configuration.tar.gz
      ```

1. Remove all the authorized keys on the instance. [reference](https://aws.amazon.com/articles/how-to-share-and-use-public-amis-in-a-secure-manner/)

      ```shell
      sudo find / -name "authorized_keys" -print -exec rm -f {} \;
      ```

1. Clean the command line history.

      ```shell
      sudo find /root/.*history /home/*/.*history -exec rm -f {} \;
      history -c
      ```

1. Use a new console try to use ssh connect to the instace, make sure now you
 cannot connect any more. (Because all the access keys are cleared)
1. Login to the aws EC2 management console, create an AMI (Amazon Machine Image)
 from this instance.
1. Copy the image to US East (N. Virginia).
1. Go to the AWS marketplace management portal, open AMI tab, share the image
 just copied. Change the OS user name to ubuntu (This step is important,
 otherwise the aws scan cannot start).
`Note: Step 9 and 10 may have some aws permission issues, if encountered, then
 need administrator help to do.`
1. Scan the image, it takes about half an hour, wait until the result is successful.
1. Go to the listing, in "add a new listing" select "Free AMI", and then fill in
 the details of the image, in "Enter your AMI", please input the AMI just shared.
