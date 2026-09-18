use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum RecordType {
    A,
    AAAA,
    CNAME,
    TXT,
    SRV,
    NS,
    PTR,
    MX,
    NAPTR
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProviderSpecificProperty {
    pub name: String,
	pub value: String,
}

pub type TTL = i64;
pub type ProviderSpecific = Vec<ProviderSpecificProperty>;
pub type Targets = Vec<String>;
pub type Labels = HashMap<String,String>;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
	// The hostname of the DNS record
	pub dns_name: String,
	// The targets the DNS record points to
    pub targets: Targets,
	// RecordType type of record, e.g. CNAME, A, AAAA, SRV, TXT etc
	pub record_type: RecordType,
	// Identifier to distinguish multiple records with the same name and type (e.g. Route53 records with routing policies other than 'simple')
	pub set_identifier: Option<String>,
	// TTL for the record
	pub record_t_t_l: Option<TTL>,
	// Labels stores labels defined for the Endpoint
	pub labels: Option<Labels>,
	// ProviderSpecific stores provider specific config
	pub provider_specific: Option<ProviderSpecific>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Changes {
	// Records that need to be created
	pub create: Option<Records>,
	// Records that need to be updated (current data)
	pub update_old: Option<Records>,
	// Records that need to be updated (desired data)
	pub update_new: Option<Records>,
	// Records that need to be deleted
	pub delete: Option<Records>,
}

pub type Records = Vec<Endpoint>;

