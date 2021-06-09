// use graphql_client::Error;
// use anyhow::Error;
// use clap::Error;
use serde_derive::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Response<T> {
//     pub data: Option<T>,
//     pub errors: Option<Error>,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Packages {
    pub packages: Vec<Packages>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub description: String,
    pub maintainer: String,
    #[serde(rename = "pgpKey")]
    pub pgp_key: String,
    #[serde(rename = "buildDate")]
    pub build_date: String,
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageByName {
    #[serde(rename = "PackageByName")]
    pub package_by_name: Package,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageByNames {
    #[serde(rename = "PackageByNames")]
    pub package_by_names: Vec<Package>,
}

#[derive(Debug)]
pub enum Operation {
    Add,
    Remove,
    Help,
}
