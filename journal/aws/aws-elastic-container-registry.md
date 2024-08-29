# Elastic Container Registry

- [Create an ECR Repository](#create-an-ecr-repository)
- [Push an Image to an ECR Repository](#push-an-image-to-an-ecr-repository)
- [Pull an Image from an ECR Repository](#pull-an-image-from-an-ecr-repository)
- [Delete an ECR Repository](#delete-an-ecr-repository)

## Create an ECR Repository

```shell
aws ecr create-repository --repository-name <repository_name>
```

## Push an Image to an ECR Repository

```shell
aws ecr get-login-password --region <region> | docker login --username AWS --password-stdin <account_id>.dkr.ecr.<region>.amazonaws.com
docker build -t <image_name> .
docker tag <image_name>:<image_tag> <account_id>.dkr.ecr.<region>.amazonaws.com/<repository_name>:<image_tag>
docker push <account_id>.dkr.ecr.<region>.amazonaws.com/<repository_name>:<image_tag>
```

### Push Example

```shell
aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin 578187851696.dkr.ecr.us-east-1.amazonaws.com

docker build -t thrust-api .

docker tag thrust-api:latest 578187851696.dkr.ecr.us-east-1.amazonaws.com/thrust-api:latest

docker push 578187851696.dkr.ecr.us-east-1.amazonaws.com/thrust-api:latest
```

## Pull an Image from an ECR Repository

```shell
aws ecr get-login-password --region <region> | docker login --username AWS --password-stdin <account_id>.dkr.ecr.<region>.amazonaws.com
docker pull <account_id>.dkr.ecr.<region>.amazonaws.com/<repository_name>:<image_tag>
```

### Pull Example

```shell
aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin 578187851696.dkr.ecr.us-east-1.amazonaws.com

docker pull 578187851696.dkr.ecr.us-east-1.amazonaws.com/portfolio:latest
```

## Delete an ECR Repository

```shell
aws ecr delete-repository --repository-name <repository_name>
```
