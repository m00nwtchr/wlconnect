use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::derive_type;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
	pub device_id: String,
	pub device_name: String,
	pub device_type: DeviceType,
	pub incoming_capabilities: Vec<String>,
	pub outgoing_capabilities: Vec<String>,
	pub protocol_version: usize,
	pub tcp_port: Option<u16>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "lowercase")]
pub enum DeviceType {
	Desktop,
	Laptop,
	Phone,
	Tablet,
	Tv,
}

impl Display for DeviceType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Desktop => write!(f, "desktop"),
			Self::Laptop => write!(f, "laptop"),
			Self::Phone => write!(f, "phone"),
			Self::Tablet => write!(f, "tablet"),
			Self::Tv => write!(f, "tv"),
		}
	}
}

derive_type!(Identity, "kdeconnect.identity");
