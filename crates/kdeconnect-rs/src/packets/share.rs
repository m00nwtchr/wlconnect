use serde::{Deserialize, Serialize};

use crate::derive_type;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum ShareRequest {
	File(ShareRequestFile),
	Text { text: String },
	Url { url: String },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShareRequestUpdate {
	pub number_of_files: Option<i32>,
	pub total_payload_size: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ShareRequestFile {
	pub filename: String,
	#[serde(rename = "creationTime")]
	pub creation_time: Option<u128>,
	#[serde(rename = "lastModified")]
	pub last_modified: Option<u128>,
	pub open: Option<bool>,
	#[serde(rename = "numberOfFiles")]
	pub number_of_files: Option<i32>,
	#[serde(rename = "totalPayloadSize")]
	pub total_payload_size: Option<i64>,
}

derive_type!(ShareRequest, "kdeconnect.share.request");
derive_type!(ShareRequestUpdate, "kdeconnect.share.request.update");
