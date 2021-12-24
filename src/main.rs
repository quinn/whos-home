use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sqs::{Client, Error};
// use serde_json::json;

static QUEUE_URL: &str = "https://sqs.us-east-1.amazonaws.com/824357296248/whos-home-main-sub.fifo";

async fn handle_message(client: &aws_sdk_sqs::Client, message: aws_sdk_sqs::model::Message) -> () {
	println!("{}", message.body.unwrap());

	match client
		.delete_message()
		.queue_url(QUEUE_URL)
		.receipt_handle(message.receipt_handle.unwrap())
		.send()
		.await
	{
		Ok(_) => println!("received message"),
		Err(_) => println!("error deleting received message"),
	}
}

#[tokio::main]
async fn main() {
	let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
	let config = aws_config::from_env().region(region_provider).load().await;
	let client = Client::new(&config);

	loop {
		match client
			.receive_message()
			.wait_time_seconds(5)
			.queue_url(QUEUE_URL)
			.send()
			.await
		{
			Ok(message) => {
				println!("Request completed. checking for messages...");

				for m in message.messages {
					for m2 in m {
						handle_message(&client, m2).await;
					}
				}
			}
			Err(e) => {
				println!("{}", e);
				break;
			}
		};
	}
}
