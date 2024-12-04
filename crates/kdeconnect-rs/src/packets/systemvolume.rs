use serde::{Deserialize, Serialize};

use crate::derive_type;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum SystemVolume {
	List {
		#[serde(rename = "sinkList")]
		sink_list: Vec<SystemVolumeStream>,
	},
	Update {
		name: String,
		enabled: Option<bool>,
		muted: Option<bool>,
		volume: Option<i32>,
	},
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SystemVolumeStream {
	pub name: String,
	pub description: String,
	pub enabled: Option<bool>,
	pub muted: bool,
	pub max_volume: Option<i32>,
	pub volume: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SystemVolumeRequest {
	// this may happen again
	#[serde(skip_serializing_if = "Option::is_none")]
	pub request_sinks: Option<bool>,
	pub name: Option<String>,
	pub enabled: Option<bool>,
	pub muted: Option<bool>,
	pub volume: Option<i32>,
}

derive_type!(SystemVolume, "kdeconnect.systemvolume");
derive_type!(SystemVolumeRequest, "kdeconnect.systemvolume.request");
