# !/bin/bash

set -Eeo pipefail


##### Get parameters based on environment. This will be moved to SSM in the next update.
if [ $STAGE == "prd" ]; then
    DEPLOY_BUCKET_NAME="s3://learn-common-words.net"
    TF_BUCKET_PATH="s3://terraform-state-yyq3vrfhye7d/learn-common-words/prd/"
    CF_DISTRIBUTION_ID="E10DDAUK4G06OW"
elif [ $STAGE == "dev" ]; then
    DEPLOY_BUCKET_NAME="s3://dev.learn-common-words.net"
    TF_BUCKET_PATH="s3://terraform-state-yyq3vrfhye7d/learn-common-words/dev/"
    CF_DISTRIBUTION_ID="E1TUWRN0Y8YSU1"
else
    echo "[ERROR] Invalid deployment stage ($STAGE)"
    exit 1
fi


##### Deploy the terraform #####
cd terraform

echo "[INFO] Installing terraform using Yum"
yum install -y yum-utils
yum-config-manager --add-repo https://rpm.releases.hashicorp.com/AmazonLinux/hashicorp.repo
yum -y install terraform

# Copy the tfstate and lock file from s3 - a bit of a hack until I put together something more elegant
aws s3 cp $TF_BUCKET_PATH"terraform.tfstate" "terraform.tfstate"
aws s3 cp $TF_BUCKET_PATH".terraform.lock.hcl" ".terraform.lock.hcl"

echo "[INFO] Removing cached terraform modules"
rm -Rf .terraform/modules

echo "[INFO] Initialiasing terraform in ${BUILD_ENV} environment"
terraform init -reconfigure -backend-config="./environments/${BUILD_ENV}/backend.conf"

echo "[INFO] Validating terraform code"
terraform validate

echo "[INFO] Deploying the infrastructure in ${BUILD_ENV} environment"
terraform apply -auto-approve -var-file="./environments/${BUILD_ENV}/terraform.tfvars"

# Write the tfstate and lock file back to s3
aws s3 cp "terraform.tfstate" $TF_BUCKET_PATH"terraform.tfstate"
aws s3 cp ".terraform.lock.hcl" $TF_BUCKET_PATH".terraform.lock.hcl"

cd ..


##### Deploy the frontend #####
echo "[INFO] Building frontend"
make build

echo "[INFO] Writing frontend files to S3"
aws s3 cp dist $DEPLOY_BUCKET_NAME --recursive

echo "[INFO] Invalidating Cloudfront cache"
aws cloudfront create-invalidation --distribution-id $CF_DISTRIBUTION_ID --paths '/*'
