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

1. Use a new console try to use ssh connect to the instance, make sure now you
   cannot connect any more. (Because all the access keys are removed)
1. Login to the aws EC2 management console, create an AMI (Amazon Machine Image)
   from this instance.
1. Copy the image to US East (N. Virginia).
1. Go to the AWS marketplace management portal, open AMI tab, share the image
   just copied. Change the OS user name to ubuntu (This step is important,
   otherwise the aws scan cannot start).
   `Note: Step 9 and 10 may have some aws permission issues, if encountered, then need administrator help to do.`
1. Scan the image, it takes about half an hour, wait until the result is successful.
1. Go to the "Product -> Server", click "create server product" select "free", and then fill in
   the details of the image, in "Enter your AMI", please input the AMI just shared.
1. Fills in the details of the product, the items with mark \* is mandatory.
1. Submit the form to AWS for review, it may take a few days to get response from AWS team.

# Add New Version for Existing AWS Marketplace Product

1. Following the same steps of 1~11 of last chapter to create a new AMI, record the AMI name.
1. Go the AWS marketplace management portal, go to "Product -> Server", open the current product page, clicked the "Edit" button, open the "Add/Edit Version" tab, Toggle option to "Add a New Version", fill in the product new version details.
1. If the old version cannot use any more, in "Remove Version" tab select remove old version.
1. After all the details are filled in, submit to AWS for review, the update version review is very fast, normally can get response from AWS team in 1~2 days.
