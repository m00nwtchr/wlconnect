use serde::{Deserialize, Serialize};

use crate::derive_type;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Pair {
	pub pair: bool,
}

derive_type!(Pair, "kdeconnect.pair");
