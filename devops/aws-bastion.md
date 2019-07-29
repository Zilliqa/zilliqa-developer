# AWS Cloud9 Bastion

AWS Cloud9 is a browser-based service that offers you a coding environment from any location. We use it as a tool to manage bastion hosts. This has several benefits in terms of security, cost and maintenance effort:

- **Permissions built into bastion**. The access permissions to other AWS resources are built into the bastion host using IAM instance profile. Users who log in to bastion do not need to carry any permissions.

- **Restricted access**. By default, the bastion only allows access from a logged-in user through AWS Cloud9 console. Direct access from Internet is disallowed in the security group of the Cloud9 EC2 instance.

- **Cost-saving upon hibernation**. When the bastion is not used by any users or does not have any activities, it will be stopped to save cost. It will resume running when anyone logs in.

- **Shareable environment**. The bastion can be shared and thus grant access to human users.

The following topics are covered in this document.

- [Securing your browser](#securing-your-browser)
- [Creation of bastion](#creation-of-bastion)
- [Permissions to Other AWS Services](#permissions-to-other-aws-services)
- [Backup of User Data](#backup-of-user-data)
- [Example Use Cases](#example-use-cases)
  - [Kubernetes Clusters Management](#kubernetes-clusters-management)
  - [Kubernetes Admin/User](#kubernetes-adminuser)
- [References](#references)
- [TODOs](#todos)

## Securing your browser

The Cloud9 service or other AWS services might be accessed through browsers all the time. Therefore, it requires a secure browser setup (of course on a secure OS) to prevent security breaches.

In your browser, most of the threats may arise from the extensions you install and the websites you visit. They may contains malicous code that hijacks your sessions or steals your information. To have at least first line of defence, it's highly recommended to access AWS services **from a browser with no extension** installed. Also, if you can please **don't access untrusted websites** from the browser you used for AWS accessing.

To acheive the separation, you may choose either one of these options:

- Use a separate browser for AWS services. For example, you are using Chrome for daily work but you can use a vanilla (i.e., no extesions) Firefox for AWS services
- Create a separate browser profile with no extensions for AWS services. For example, in Chrome you can have [multiple profiles  (or persons)](https://support.google.com/chrome/answer/2364824?co=GENIE.Platform%3DDesktop&hl=en) for different purpose. This works well if you even want to access multiple AWS accounts at the same time.

## Creation of bastion

The creation of the bastion is done through AWS management console. Make sure you log in the correct account with the necessary permissions.

Open **AWS Cloud9** and select **Create environment**.

In Step 1 **Name environment**, fill in the **Name** and **Description** properly.

In Step 2 **Configure settings**, configure the options as follows and enter **Next Step**.

- **Environment type**: select the default `Create a new instance for environment (EC2)`
- **Instance type**: select the default `t2.micro` or selelct powerful ones if you need
- **Platform**: select `Ubuntu Server 18.04 TLS`
- **Cost-saving setting**: select the default `After 30 minutes`
- **Network settings (advanced)**: use the default
  > **Note**: If your bastion is considered critical and sensitive, please create a new VPC for you bastion host and do not use the default VPC.

In Step 3 **Review**, check the information again and click **Create environment**.

You will be forwarded to the Cloud9 page and soon your bastion will be ready to use.

## Permissions to Other AWS Services

By default, your bastion is able to use [AWS Managed Temporary Credentials](https://docs.aws.amazon.com/cloud9/latest/user-guide/auth-and-access-control.html#auth-and-access-control-temporary-managed-credentials-supported), which automatically allows the bastion to access other AWS services on behalf of the AWS entity (account, IAM user, federated user, etc.) you use to create the bastion.

However, this default permission might be too permissive or too restrictive if

1. you will share the bastion with other users who have less permissions than you.
2. you need the bastion to have more permissions than your current account.

For these situations, you can choose to:

1. [Switch off the AWS Managed Temporary Credentials](https://docs.aws.amazon.com/cloud9/latest/user-guide/auth-and-access-control.html#auth-and-access-control-temporary-managed-credentials-supported)
2. [Create and Use an Instance Profile to Manage Temporary Credentials](https://docs.aws.amazon.com/cloud9/latest/user-guide/credentials.html#credentials-temporary)

## Sharing Created Environments

Steps for the user to be invited:

1. In the AWS applications page, click "Command line or programmatic access" for the correct role (e.g., `Cloud9User`) and account.
2. Retrieve the AWS environment variables and export to a terminal session on your local machine or other Cloud9 bastions.
3. Execute `aws sts get-caller-identity --query 'Arn' --output text` to get the ARN (Amazon Resource Name):

```Shell
{
    "UserId": "<redacted>",
    "Account": "<redcated>",
    "Arn": "<arn-string>"
}
```

Steps for the environment creator:

1. Launch the IDE for your environment
2. Go to AWS Cloud9 > Preferences > AWS Settings > Credentials and disable "AWS managed temporary credentials"
3. Go to "Share" on the top-right corner of the IDE
4. Add the IAM username (i.e., the `arn-string`) of the other user and select RW access
5. After inviting, the environment should now appear in the "Shared with you" section in the other user's Cloud9 management console

## Backup of User Data

Please be aware that the Cloud9 bastion does not provide any backup of the data. It is essentially an EC2 instance managed by Cloud9 with no automatic backups. You will have the same concern of data persistence when you use an EC2 instance. So backup your own data or credentials if they are important.

## Example Use Cases

### Kubernetes Clusters Management

Managing Kubernetes cluster involves having access to other AWS resources which are considered critical and risky if unauthorized access happens. There are a few advantages using Cloud9 bastion for this:

1. The permissions can be safely assigned to the bastion through IAM instance profile and the user account does not need to have them.
2. If the bastion is in the hibernation state, no one can use the permissions.

### Kubernetes Admin/User

If you are an admin or user of the Kubernetes clusters, you can also make use of Cloud9 bastion to:

1. Safely store `kubeconfig` that has admin access to the cluster.
2. Enjoy optimized network latency when accessing clusters from the bastion.

## References

- [AWS Cloud9 User Guide](https://docs.aws.amazon.com/cloud9/latest/user-guide/welcome.html)

## TODOs

- [ ] add guide about sharing bastion with others.
