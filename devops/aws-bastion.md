# AWS Bastion Management

On AWS, we use Cloud9 as a tool to manage bastion host. This has several benefits in terms of security, cost and maintenance effort:

- **Permissions built into bastion**. The access permissions to project resources are built into the bastion host using IAM instance profile. Users who log in to bastion don not carry any permissions.

- **Cost-saving upon hibernation**. When the bastion is not used by any users or does not have any activites, it will be stopped to save cost. It will resume running when anyone logs in.

- **Shareable environment**. The bastion can be shared, which is the way we used to grant access to human users.

Also, the use of Cloud9 is integrated closely with AWS SSO login, we will see that shortly.

This document is organized as follows:
<!-- TOC -->

- [Creation of bastion](#creation-of-bastion)

<!-- /TOC -->

## Creation of bastion

The creation of the bastion is done through AWS management console. Make sure you log in the correct account with the necessary permissions.

Open **AWS Cloud9** and select **Create environment**.

In Step 1 **Name environment**, fill in the **Name** and **Description** properly.

In Step 2 **Configure settings**, configure the options as follows and only change if you understand the consequence

- **Environment type**: select the default `Create a new instance ...`
- **Instance type**: select the default `t2.micro` or selelct powerful ones if you need
- **Platform**: select `Ubuntu Server 18.04 TLS`
- **Cost-saving setting**: select the default `After 30 minutes`
- **Network settings (advanced)**: open the dropdown options and create new vpc
  > **Note**: Always create a new VPC for your bastion host. Do not use the default VPC.
  - In the new tab opened, Step 1, select **VPC with a Single Public Subnet**
  - In Step 2, give a meaningful **VPC name** and **public subnet name** and click **Create** with other default options.

In Step 3 **Review**, check the information again and click **Create environment**.

You will be forwarded to the Cloud9 page and soon your bastion will be ready to use.

<!-- TODO: detailed instructions needed -->








