# Certificate Manager

To serve your container over HTTPS, you'll need to use an Application Load Balancer (ALB) with an SSL/TLS certificate from AWS Certificate Manager (ACM). Here's a step-by-step guide:

## Prerequisites

1. **AWS Account**: Ensure you have an AWS account.
2. **Docker Image in ECR**: Ensure your Docker image is already pushed to Amazon ECR.
3. **ECS Cluster**: Ensure you have an ECS cluster created.

## Step-by-Step Guide

### 1. Obtain an SSL/TLS Certificate

1. **Open the AWS Certificate Manager (ACM) Console**:

   - Navigate to the [ACM Console](https://console.aws.amazon.com/acm/home).

2. **Request a Certificate**:

   - Click on "Request a certificate".
   - Choose "Request a public certificate" and click "Request a certificate".
   - Enter your domain name (e.g., `example.com` and `www.example.com`).
   - Choose a validation method (DNS validation is recommended).
   - Follow the instructions to validate your domain ownership.

3. **Complete the Validation Process**:
   - For DNS validation, add the provided CNAME record to your DNS provider.
   - Once validated, the certificate status will change to "Issued".

### 2. Create an Application Load Balancer (ALB)

1. **Open the EC2 Console**:

   - Navigate to the [EC2 Console](https://console.aws.amazon.com/ec2/).

2. **Create a Load Balancer**:

   - Click on "Load Balancers" in the left-hand menu.
   - Click the "Create Load Balancer" button.
   - Choose "Application Load Balancer" and click "Create".

3. **Configure the Load Balancer**:

   - **Name**: Enter a name for your load balancer.
   - **Scheme**: Choose "Internet-facing".
   - **IP address type**: Choose "ipv4".
   - **Listeners**: Ensure there is a listener on port 80 (HTTP) and click "Add listener" to add a listener on port 443 (HTTPS).
   - **Availability Zones**: Select your VPC and at least two subnets in different Availability Zones.

4. **Configure Security Settings**:

   - **Security groups**: Create or select a security group that allows inbound traffic on ports 80 and 443.

5. **Configure Listeners and Routing**:

   - **HTTP Listener**: Set up a rule to redirect HTTP (port 80) traffic to HTTPS (port 443).
   - **HTTPS Listener**: Choose "Forward to" and select your target group (you'll create this in the next step).
   - **SSL Certificate**: Select "Choose an existing certificate from ACM" and choose the certificate you requested earlier.

6. **Register Targets**:

   - Create a target group for your ECS service. Ensure the target type is "IP" if using Fargate.
   - Configure health checks as needed.

7. **Review and Create**:
   - Review your settings and click "Create load balancer".

### 3. Update ECS Service to Use ALB

1. **Open the ECS Console**:

   - Navigate to the [ECS Console](https://console.aws.amazon.com/ecs/).

2. **Update Your Service**:
   - Go to your cluster and select your service.
   - Click "Update".
   - In the "Load balancing" section, choose "Application Load Balancer".
   - Select the ALB you created.
   - Choose the HTTPS listener and the target group you created.
   - Update the service.

### 4. Configure Health Checks

1. **Update Target Group Health Checks**:
   - Go back to the EC2 console.
   - Select "Target Groups" in the left-hand menu.
   - Select your target group and configure the health checks as needed (e.g., HTTP path `/health`).

## Verify the Setup

1. **Check the ALB**:

   - Ensure the ALB is active and that it has successfully registered your ECS service tasks.

2. **Access Your Application**:
   - Use the DNS name of your ALB to access your application over HTTPS (e.g., `https://your-alb-dns-name`).

By following these steps, you can serve your ECS service over HTTPS using an Application Load Balancer and an SSL/TLS certificate from AWS Certificate Manager.
