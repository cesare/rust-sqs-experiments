extern crate rustc_serialize;
extern crate rusoto;
use rusoto::{ProfileProvider, Region};

use std::error::Error;
use std::str::FromStr;

pub struct AwsConfig {
    profile_name: String,
    region_name: String,
}

impl AwsConfig {
    pub fn new(profile_name: &str, region_name: &str) -> AwsConfig {
        AwsConfig {
            profile_name: profile_name.to_string(),
            region_name: region_name.to_string(),
        }
    }
}

pub fn find_provider(config: &AwsConfig) -> Result<ProfileProvider, String> {
    match ProfileProvider::new() {
        Ok(mut provider) => {
            provider.set_profile(config.profile_name.clone());
            Ok(provider)
        }
        Err(_) => Err(String::from("Failed to find provider")),
    }
}

pub fn find_region(config: &AwsConfig) -> Result<Region, String> {
    Region::from_str(&config.region_name).or_else(|e| Err(String::from(e.description())))
}
