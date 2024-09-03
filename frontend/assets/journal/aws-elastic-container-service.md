# Elastic Container Service (ECS)

- [ClI Tools](#cli-tools)

## UI Walkthrough

We will walkthrough the steps of creating and ECS cluster, task, and service. After that, we will deploy a simple web application using the Elastic Container Registry.

### Create an ECS Cluster

1. Navigate to the ECS console and create an ECS cluster, name it `my-app`.

2. For the infastructure provider, select `Amazon Fargate (serverless)`.

3. Click **Create**.

There will be a message that the cluster has been created.

### Create an ECS Task Definition

1. Go to Elastic Container Service product page

2. Select Task Definition from the left hand menu, and click **Create Task Definition**

3. Configure Task and Container Definitions

   - Give the task a name
   - For infrastructure provider, select `Amazon Fargate (serverless)`
   - Configure the task size, 1vCPU and 3GB memory is standard

4. Setup the container for this Task

   - Enter a name for your container.
   - Image URL: Enter your ECR image URL (e.g., `<aws_account_id>.dkr.ecr.<region>.amazonaws.com/my-app:latest`).
   - Memory Limits: Set the memory limits.
   - Port Mappings: Map the container port (e.g., 80) to a host port.

5. Add Storage and Network Configuration (if needed):

#### With JSON

1. Open the Task Definitions Page:
   - Click on "Task Definitions" in the left-hand menu.
   - Click the "Create new Task Definition" button.
   - Select "JSON" as the format.

**Example:**

```json
{
  "family": "my-task",
  "networkMode": "awsvpc",
  "containerDefinitions": [
    {
      "name": "my-container",
      "image": "<aws_account_id>.dkr.ecr.<region>.amazonaws.com/my-app:latest",
      "essential": true,
      "memory": 512,
      "cpu": 256,
      "portMappings": [
        {
          "containerPort": 80,
          "hostPort": 80,
          "protocol": "tcp"
        }
      ]
    }
  ],
  "requiresCompatibilities": ["FARGATE"],
  "cpu": "256",
  "memory": "512",
  "executionRoleArn": "arn:aws:iam::<aws_account_id>:role/ecsTaskExecutionRole"
}
```

### Create a Service

1. Select the cluster you created earlier.

2. Click the "Create" button in the "Services" tab.

3. Configure Service Settings:

   - Task Definition: Select the task definition you created earlier.
   - Service name: Enter a name for your service.
   - Number of tasks: Specify the number of tasks to run.
   - Select Launch: Fargate Spot is the cheapest.
   - Network Configuration: For Fargate, choose the VPC and subnets, and select a security group.

4. If you want to use a load balancer, configure it under the "Load balancing" section.

### Verify and Access Your Service

1. Check the Service Status:

   - In the ECS Console, go to your cluster and check the "Services" tab.
   - Ensure the desired number of tasks are running.

2. Access Your Application:
   - If you configured a load balancer, use the load balancer's DNS name to access your application.
   - If not, use the public IP addresses of the running tasks (available in the "Tasks" tab under your service).

By following these steps, you can deploy a Docker image from Amazon ECR to Amazon ECS using the AWS Management Console.

## CLI Tools

Create an ECS cluster

```shell
aws ecs create-cluster --cluster-name my-cluster
```

Create an ECS Task Definition

```shell
aws ecs create-task-definition --cli-input-json file://task-definition.json
```

Create an ECS Service

```shell
aws ecs create-service --cluster my-cluster --service-name my-service --task-definition my-task-definition --desired-count 1
```

Create an ECS Task

```shell
aws ecs run-task --cluster my-cluster --task-definition my-task-definition
```

Delete an ECS Task

```shell
aws ecs delete-task --cluster my-cluster --task my-task
```

Get an ECS Task

```shell
aws ecs describe-tasks --cluster my-cluster --tasks my-task
```

Get an ECS Service

```shell
aws ecs describe-services --cluster my-cluster --services my-service
```

Get an ECS Task Definition

```shell
aws ecs describe-task-definition --task-definition my-task-definition
```

Get an ECS Cluster

```shell
aws ecs describe-clusters --cluster my-cluster
```
