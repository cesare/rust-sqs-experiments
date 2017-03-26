extern crate rustc_serialize;
extern crate rusoto;
use rusoto::{ProfileProvider, Region};

use std::error::Error;
use std::str::FromStr;

pub fn find_provider(profile_name: &str) -> Result<ProfileProvider, String> {
    match ProfileProvider::new() {
        Ok(mut provider) => {
            provider.set_profile(profile_name.clone());
            Ok(provider)
        }
        Err(_) => Err(String::from("Failed to find provider")),
    }
}

pub fn find_region(region_name: &str) -> Result<Region, String> {
    Region::from_str(region_name).or_else(|e| Err(String::from(e.description())))
}
