resource "aws_s3_bucket" "deploy-bucket" {
  bucket = "${var.bucket_name}"
}

resource "aws_s3_bucket_policy" "allow_access" {
  bucket = aws_s3_bucket.deploy-bucket.id
  policy = jsonencode({
    "Version" : "2012-10-17",
    "Statement" : [
      {
        "Sid" : "PublicReadGetObject",
        "Effect" : "Allow",
        "Principal" : "*",
        "Action" : "s3:GetObject",
        "Resource" : "${aws_s3_bucket.deploy-bucket.arn}/*"
      }
    ]
  })
}

resource "aws_s3_bucket_website_configuration" "website-config" {
  bucket = aws_s3_bucket.deploy-bucket.id
  index_document {
    suffix = "index.html"
  }
}
