# AWS CLI Auth Guide (Mac)

## Install AWS CLI

Using homebrew

```bash
brew install awscli
```

```bash
aws --version
```

## Configure Auth

If you run **aws configure** you will get this output

```zsh
AWS Access Key ID [None]:
AWS Secret Access Key [None]:
Default region name [None]:
Default output format [None]:
```

We need to set up an IAM user first in order to fill these out. So head over to the **IAM Service** in your Amazon console.

1. In the search bar at the top of the console, type "IAM" and select the IAM (Identity and Access Management) service from the drop-down list.

2. Create a **User Group** with a name of your choosing. We will need to attach policies to this group.

3. The bare minimum required for the service you want to setup is recommended. For example, deploying an ECS and ECR service.

4. Now we need to create an IAM user with the name of your choosing. When setting permissions, put the user in the same group we created in step 2. Save and continue.

5. Select the user you just created, if it is not already in the dashboard. There you should see a button called **Create Access Key**. Click it and save the credentials in a secure location.

6. Now you can run `aws configure` and use those credentials.

## Run Commands

Now that the user is all set, we can authenticate to AWS. For exmaple, we can get an auth token for our ECR repo.

```zsh
aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin 578187851696.dkr.ecr.us-east-1.amazonaws.com
```
