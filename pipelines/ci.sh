set -Eeo pipefail

##### Deploy the terraform #####
cd terraform

echo "[INFO] Installing terraform using Yum"
yum install -y yum-utils
yum-config-manager --add-repo https://rpm.releases.hashicorp.com/AmazonLinux/hashicorp.repo
yum -y install terraform

echo "[INFO] Initialiasing terraform in ${BUILD_ENV} environment"
terraform init -reconfigure -backend-config="./environments/${BUILD_ENV}/backend.conf"

echo "[INFO] Validating terraform code"
terraform validate

echo "[INFO] Deploying the infrastructure in ${BUILD_ENV} environment"
terraform apply -auto-approve -var-file="./environments/${BUILD_ENV}/terraform.tfvars"

cd ..


##### Deploy the frontend #####
echo "[INFO] Building frontend"
make build

echo "[INFO] Writing frontend files to S3"
aws s3 cp dist $DEPLOY_BUCKET_NAME --recursive

echo "[INFO] Invalidating Cloudfront cache"
aws cloudfront create-invalidation --distribution-id $CF_DISTRIBUTION_ID --paths '/*'
