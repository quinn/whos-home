resource "aws_sns_topic" "whos-home" {
	name = "whos-home.fifo"
	fifo_topic = true
	content_based_deduplication = true
}

resource "aws_sqs_queue" "whos-home-queue-quinn" {
  name = "whos-home-quinn.fifo"
	fifo_queue = true
	content_based_deduplication = true
}

resource "aws_sns_topic_subscription" "whos-home-topic-subscription-quinn" {
	topic_arn = aws_sns_topic.whos-home.arn
	protocol  = "sqs"
	endpoint = aws_sqs_queue.whos-home-queue-quinn.arn
}

# TODO: This exists already in AWS, might need to delete it and have tf re-create it
resource "aws_iam_role" "whos-home-lambda" {}

# TODO: Can this just connect the function to the role? Probably not..
resource "aws_lambda_function" "whos-home-arrival-handler" {
	# filename = "arrivals_handler/function.zip"
	# function_name = "whos-home-arrival-handler"
	role = aws_iam_role.whos-home-lambda.arn
}
