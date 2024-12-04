use serde::{Deserialize, Serialize};

use crate::derive_type;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RunCommand {
	#[serde(rename = "commandList")]
	pub command_list: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RunCommandItem {
	pub name: String,
	pub command: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RunCommandRequest {
	pub key: Option<String>,
	#[serde(rename = "requestCommandList")]
	pub request_command_list: Option<bool>,
}

derive_type!(RunCommand, "kdeconnect.runcommand");
derive_type!(RunCommandRequest, "kdeconnect.runcommand.request");
