resource "aws_dynamodb_table" "german-words" {
  name           = "learn-common-words-german-${var.environment}"
  billing_mode   = "PROVISIONED"
  read_capacity  = 5
  write_capacity = 5
  hash_key       = "frequency"

  attribute {
    name = "frequency"
    type = "N"
  }

  tags = {
    Name        = "learn-common-words-german-${var.environment}"
    Description = "DynamoDB table to store german words and their translations for the learn-common-words application"
    Environment = "${var.environment}"
  }
}
