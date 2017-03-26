extern crate docopt;
use docopt::Docopt;

use std::default::Default;

extern crate rustc_serialize;
extern crate rusoto;
use rusoto::ProfileProvider;
use rusoto::sqs::{SendMessageRequest, SqsClient};
use rusoto::default_tls_client;

mod aws;
use aws::*;

const USAGE: &'static str = "
Usage:
    sqs-sender
    sqs-sender [--profile=<profile>] [--region=<region>] <queue-url> <message>
    sqs-sender --help

Options:
    --profile=<profile>  Use a specific profile from your credential file. [default: default]
    --region=<region>    Specify a region. [default: ap-northeast-1]
";

#[derive(Debug, RustcDecodable)]
pub struct Args {
    flag_profile: String,
    flag_region: String,
    arg_queue_url: String,
    arg_message: String,
}

fn parse_args() -> Args {
    Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit())
}

fn main() {
    let args: Args = parse_args();
    let config: AwsConfig = AwsConfig::new(&args.flag_profile, &args.flag_region);
    let provider: ProfileProvider = find_provider(&config).unwrap();
    let region = find_region(&config).unwrap();
    let client = SqsClient::new(default_tls_client().unwrap(), provider, region);
    let mut request = SendMessageRequest::default();
    request.message_body = args.arg_message;
    request.queue_url = args.arg_queue_url;

    match client.send_message(&request) {
        Ok(results) => {
            println!("{:?}", results);
        }
        Err(error) => println!("{:?}", error),
    }
}
