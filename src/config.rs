use once_cell::sync::Lazy;
use clap::Parser;
use serde::{Deserialize, Serialize};

pub static CONFIG: Lazy<Config> = Lazy::new(|| {Config::parse()});

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(
        long,
        default_value_t = false)]
    pub dry_run: bool,

    #[arg(
        long, short,
        default_value_t = false)]
    pub debug: bool,

    #[arg(
        long,
        value_name = "HOST_CM_NAME",
        env = "HOST_CM_NAME",
        default_value_t = String::from("external-mdns"))]
    pub host_configmap_name: String,
    
    #[arg(
        long,
        value_name = "HOST_CM_NAMESPACE",
        env = "HOST_CM_NAMESPACE")]
    pub host_configmap_namespace: Option<String>,

    #[arg(
        long,
        value_name = "HOST_CM_KEY",
        env = "HOST_CM_KEY",
        default_value_t = String::from("hosts"))]
    pub host_configmap_key: String,
   
    #[arg(
        long,
        value_name = "LISTEN_ADDR",
        env = "LISTEN_ADDR",
        default_value_t = String::from("127.0.0.1:8888"))]
    pub listen_addr: String,

    #[arg(
        long,
        value_name = "HEALTH_LISTEN_ADDR",
        env = "HEALTH_LISTEN_ADDR",
        default_value_t = String::from("0.0.0.0:8080"))]
    pub health_listen_addr: String,

    #[command(flatten)]
    pub domain_filter: DomainFilter,
}

#[derive(Serialize, Deserialize, Parser, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct DomainFilter {
    // Filters define what domains to match
    #[arg(
        long,
        value_delimiter = ',',
        value_name = "DOMAINS_FILTER",
        env = "DOMAINS_FILTER",
        default_value = ".local")]
    pub filters: Vec<String>,
    
    // exclude define what domains not to match
    #[arg(
        long,
        value_delimiter = ',',
        value_name = "DOMAINS_EXCLUDE",
        env = "DOMAINS_EXCLUDE",
        default_value = "")]
    pub exclude: Vec<String>,
    
    // regex defines a regular expression to match the domains
    #[arg(
        long,
        value_name = "DOMAINS_REGEX",
        env = "DOMAINS_REGEX",
        default_value = "")]
    pub regex: String,

    // regexExclusion defines a regular expression to exclude the domains matched
    #[arg(
        long,
        value_name = "DOMAINS_REGEX_EXCUDE",
        env = "DOMAINS_REGEX_EXCUDE",
        default_value = "")]
    pub regex_exclusion: String,
}
