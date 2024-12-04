use serde::{Deserialize, Serialize};

use crate::derive_type;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[serde(untagged)]
pub enum Presenter {
	Move { dx: f32, dy: f32 },
	Stop { stop: bool },
}

derive_type!(Presenter, "kdeconnect.presenter");
