resource "aws_s3_bucket" "prd-bucket" {
  bucket = "learn-common-words.net"
}

resource "aws_s3_bucket_policy" "allow_access-prd" {
  bucket = aws_s3_bucket.prd-bucket.id
  policy = jsonencode({
    "Version" : "2012-10-17",
    "Statement" : [
      {
        "Sid" : "PublicReadGetObject",
        "Effect" : "Allow",
        "Principal" : "*",
        "Action" : "s3:GetObject",
        "Resource" : "${aws_s3_bucket.prd-bucket.arn}/*"
      }
    ]
  })
}

resource "aws_s3_bucket_website_configuration" "website-config-prd" {
  bucket = aws_s3_bucket.prd-bucket.id
  index_document {
    suffix = "index.html"
  }
}


resource "aws_s3_bucket" "dev-bucket" {
  bucket = "dev.learn-common-words.net"
}

resource "aws_s3_bucket_policy" "allow_access-dev" {
  bucket = aws_s3_bucket.dev-bucket.id
  policy = jsonencode({
    "Version" : "2012-10-17",
    "Statement" : [
      {
        "Sid" : "PublicReadGetObject",
        "Effect" : "Allow",
        "Principal" : "*",
        "Action" : "s3:GetObject",
        "Resource" : "${aws_s3_bucket.dev-bucket.arn}/*"
      }
    ]
  })
}

resource "aws_s3_bucket_website_configuration" "website-config-dev" {
  bucket = aws_s3_bucket.dev-bucket.id
  index_document {
    suffix = "index.html"
  }
}
