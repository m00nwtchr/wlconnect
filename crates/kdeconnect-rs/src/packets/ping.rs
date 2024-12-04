use serde::{Deserialize, Serialize};

use crate::derive_type;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ping {
	pub message: Option<String>,
}

derive_type!(Ping, "kdeconnect.ping");
