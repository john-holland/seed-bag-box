# Deployment Guide

## Prerequisites

Before deploying Seed Box Bag Box, ensure you have:

1. **AWS Account** with appropriate permissions
2. **AWS CLI** installed and configured
3. **Rust** (1.75+) installed
4. **Cargo Lambda** installed: `cargo install cargo-lambda`
5. **AWS SAM CLI** installed: `pip install aws-sam-cli`
6. **Zig** installed (for cross-compilation to ARM64)

## Initial Setup

### 1. Install Required Tools

```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install cargo-lambda
cargo install cargo-lambda

# Install AWS SAM CLI
pip install aws-sam-cli

# Verify installations
cargo lambda --version
sam --version
aws --version
```

### 2. Configure AWS Credentials

```bash
aws configure
# Enter your AWS Access Key ID
# Enter your AWS Secret Access Key
# Enter your default region (e.g., us-west-2)
# Enter your default output format (json)
```

### 3. Set Up Secrets in AWS Secrets Manager

```bash
# ShipStation credentials
aws secretsmanager create-secret \
    --name seed-box/shipstation \
    --secret-string '{"api_key":"YOUR_API_KEY","api_secret":"YOUR_API_SECRET"}'

# CrateJoy credentials
aws secretsmanager create-secret \
    --name seed-box/cratejoy \
    --secret-string '{"api_key":"YOUR_CRATEJOY_KEY"}'
```

## Building the Project

### Build All Lambda Functions

```bash
# Build for AWS Lambda (ARM64)
make build

# Or manually:
cargo lambda build --release --arm64
```

### Build Specific Function

```bash
make build-subscription
make build-shipping
make build-inventory
make build-greenhouse
make build-plant-processing
```

## Deployment

### First-Time Deployment (Guided)

```bash
# This will prompt you for configuration
make deploy-guided

# Or manually:
sam build
sam deploy --guided
```

You'll be prompted for:
- **Stack Name**: `seed-box-bag-box` (recommended)
- **AWS Region**: Your preferred region (e.g., `us-west-2`)
- **Parameter Overrides**: None needed initially
- **Confirm changes before deploy**: Y
- **Allow SAM CLI IAM role creation**: Y
- **Save arguments to configuration file**: Y

### Subsequent Deployments

```bash
# Quick deployment using saved config
make deploy

# Or manually:
sam build && sam deploy
```

## Configuration Files

### samconfig.toml (Generated After First Deploy)

```toml
version = 0.1
[default]
[default.deploy]
[default.deploy.parameters]
stack_name = "seed-box-bag-box"
s3_bucket = "aws-sam-cli-managed-default-samclisourcebucket-xxxxx"
s3_prefix = "seed-box-bag-box"
region = "us-west-2"
confirm_changeset = true
capabilities = "CAPABILITY_IAM"
```

## Post-Deployment

### 1. Get API Endpoint

```bash
# Get the API Gateway endpoint URL
aws cloudformation describe-stacks \
    --stack-name seed-box-bag-box \
    --query 'Stacks[0].Outputs[?OutputKey==`ApiEndpoint`].OutputValue' \
    --output text
```

Example output: `https://abc123xyz.execute-api.us-west-2.amazonaws.com/prod/`

### 2. Test the Deployment

```bash
# Health check (if implemented)
curl https://your-api-endpoint/prod/health

# Create a subscription
curl -X POST https://your-api-endpoint/prod/subscriptions \
  -H "Content-Type: application/json" \
  -d '{
    "customerEmail": "test@example.com",
    "customerName": "Test User",
    "tier": "STANDARD",
    "shippingAddress": {
      "street1": "123 Main St",
      "city": "Portland",
      "state": "OR",
      "zip": "97201",
      "country": "US"
    }
  }'
```

### 3. Monitor Logs

```bash
# View logs for specific function
sam logs -n SubscriptionServiceFunction --tail

# Or use AWS CLI
aws logs tail /aws/lambda/seed-box-subscription-service --follow
```

## Updating the Application

### Update Code

```bash
# 1. Make your code changes
# 2. Build
make build

# 3. Deploy
make deploy
```

### Update Infrastructure (template.yaml)

```bash
# After modifying template.yaml
sam build
sam deploy
```

## Rollback

If deployment fails or you need to rollback:

```bash
# Rollback to previous version
aws cloudformation cancel-update-stack --stack-name seed-box-bag-box

# Or delete and redeploy
aws cloudformation delete-stack --stack-name seed-box-bag-box
# Wait for deletion to complete, then redeploy
```

## Environment-Specific Deployments

### Development Environment

```bash
sam deploy \
  --stack-name seed-box-bag-box-dev \
  --parameter-overrides Environment=dev \
  --config-env dev
```

### Production Environment

```bash
sam deploy \
  --stack-name seed-box-bag-box-prod \
  --parameter-overrides Environment=prod \
  --config-env prod
```

## CI/CD Setup (GitHub Actions Example)

Create `.github/workflows/deploy.yml`:

```yaml
name: Deploy to AWS

on:
  push:
    branches:
      - main

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: aarch64-unknown-linux-gnu
      
      - name: Install cargo-lambda
        run: cargo install cargo-lambda
      
      - name: Build Lambda functions
        run: cargo lambda build --release --arm64
      
      - name: Setup SAM CLI
        uses: aws-actions/setup-sam@v2
      
      - name: Configure AWS credentials
        uses: aws-actions/configure-aws-credentials@v2
        with:
          aws-access-key-id: ${{ secrets.AWS_ACCESS_KEY_ID }}
          aws-secret-access-key: ${{ secrets.AWS_SECRET_ACCESS_KEY }}
          aws-region: us-west-2
      
      - name: SAM Build
        run: sam build
      
      - name: SAM Deploy
        run: sam deploy --no-confirm-changeset --no-fail-on-empty-changeset
```

## Cost Monitoring

### Set Up Billing Alerts

```bash
# Create SNS topic for alerts
aws sns create-topic --name seed-box-billing-alerts

# Subscribe your email
aws sns subscribe \
  --topic-arn arn:aws:sns:us-west-2:ACCOUNT_ID:seed-box-billing-alerts \
  --protocol email \
  --notification-endpoint your-email@example.com

# Create billing alarm (requires CloudWatch billing metrics enabled)
aws cloudwatch put-metric-alarm \
  --alarm-name seed-box-monthly-cost \
  --alarm-description "Alert when monthly costs exceed $50" \
  --metric-name EstimatedCharges \
  --namespace AWS/Billing \
  --statistic Maximum \
  --period 21600 \
  --evaluation-periods 1 \
  --threshold 50 \
  --comparison-operator GreaterThanThreshold \
  --alarm-actions arn:aws:sns:us-west-2:ACCOUNT_ID:seed-box-billing-alerts
```

## Troubleshooting

### Lambda Function Errors

```bash
# Check CloudWatch logs
sam logs -n SubscriptionServiceFunction --tail

# Check Lambda configuration
aws lambda get-function --function-name seed-box-subscription-service
```

### DynamoDB Connection Issues

```bash
# Verify tables exist
aws dynamodb list-tables

# Check table status
aws dynamodb describe-table --table-name seed-box-bags
```

### API Gateway 502 Errors

- Check Lambda function timeout (increase if needed)
- Check Lambda memory allocation
- Review CloudWatch logs for function errors

### Build Failures

```bash
# Clean and rebuild
make clean
make build

# Check Rust version
rustc --version  # Should be 1.75+

# Update dependencies
cargo update
```

## Performance Tuning

### Lambda Memory Optimization

Test different memory settings:
- Start with 512MB
- Monitor execution time and memory usage in CloudWatch
- Adjust as needed (more memory = faster CPU)

### DynamoDB Throughput

- Start with PAY_PER_REQUEST mode (on-demand)
- Monitor usage in CloudWatch
- Consider PROVISIONED mode if consistent traffic

## Security Best Practices

1. **IAM Roles**: Use least privilege principle
2. **Secrets**: Store in AWS Secrets Manager, not environment variables
3. **API Gateway**: Add authentication (API keys or Cognito)
4. **VPC**: Consider VPC for database connections (if using RDS)
5. **Encryption**: Enable encryption at rest for DynamoDB

## Cleanup

To remove all resources:

```bash
# Delete CloudFormation stack (removes all resources)
aws cloudformation delete-stack --stack-name seed-box-bag-box

# Verify deletion
aws cloudformation describe-stacks --stack-name seed-box-bag-box
```

**Note**: DynamoDB tables with data may require manual deletion confirmation.

---

**Last Updated**: October 16, 2025

