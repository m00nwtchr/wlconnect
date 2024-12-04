use serde::{Deserialize, Serialize};

use crate::derive_type;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Clipboard {
	pub content: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClipboardConnect {
	pub content: String,
	pub timestamp: u128,
}

derive_type!(Clipboard, "kdeconnect.clipboard");
derive_type!(ClipboardConnect, "kdeconnect.clipboard.connect");
