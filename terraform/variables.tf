variable "region" {
  type        = string
  description = "Name of the AWS region all AWS resources will be provisioned in"
}

variable "environment" {
  type        = string
  description = "Name of the deployment environment"
}

variable "bucket_name" {
  type        = string
  description = "Name of the deployment S3 bucket"
}