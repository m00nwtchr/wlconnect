use serde::{Deserialize, Serialize};

use crate::derive_type;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct FindMyPhone {}

derive_type!(FindMyPhone, "kdeconnect.findmyphone.request");
