extern crate docopt;
use docopt::Docopt;

use std::default::Default;

extern crate rustc_serialize;
extern crate rusoto;
use rusoto::{ProfileProvider, Region};
use rusoto::sqs::{SendMessageRequest, SqsClient};
use rusoto::default_tls_client;

use std::error::Error;
use std::str::FromStr;

const USAGE: &'static str = "
Usage:
    sqs-sender
    sqs-sender [--profile=<profile>] [--region=<region>] <queue-url> <message>
    sqs-sender --help

Options:
    --profile=<profile>  Use a specific profile from your credential file.
    --region=<region>    Specify a region.
";

#[derive(Debug, RustcDecodable)]
pub struct Args {
    flag_profile: Option<String>,
    flag_region: Option<String>,
    arg_queue_url: String,
    arg_message: String,
}

fn parse_args() -> Args {
    Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit())
}

fn find_provider(args: &Args) -> Result<ProfileProvider, String> {
    match ProfileProvider::new() {
        Ok(mut provider) => {
            if let Some(ref name) = args.flag_profile {
                provider.set_profile(name.to_string());
            }
            Ok(provider)
        }
        Err(_) => Err(String::from("Failed to find provider")),
    }
}

fn find_region(args: &Args) -> Result<Region, String> {
    match args.flag_region {
        Some(ref value) => Region::from_str(value).or_else(|e| Err(String::from(e.description()))),
        None => Ok(Region::ApNortheast1),
    }
}


fn main() {
    let args: Args = parse_args();

    let provider: ProfileProvider = find_provider(&args).unwrap();
    let region = find_region(&args).unwrap();
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
