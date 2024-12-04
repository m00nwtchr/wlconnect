use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::derive_type;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Battery {
	#[serde(rename = "currentCharge")]
	pub charge: i32,
	#[serde(rename = "isCharging")]
	pub is_charging: bool,
	#[serde(
		rename = "thresholdEvent",
		serialize_with = "serialize_threshold",
		deserialize_with = "deserialize_threshold"
	)]
	pub under_threshold: bool,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct BatteryRequest {
	pub request: bool,
}

derive_type!(Battery, "kdeconnect.battery");
derive_type!(BatteryRequest, "kdeconnect.battery.request");

fn serialize_threshold<S>(x: &bool, s: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	s.serialize_i32(if *x { 1 } else { 0 })
}

fn deserialize_threshold<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
	D: Deserializer<'de>,
{
	let buf = i32::deserialize(deserializer)?;

	match buf {
		0 => Ok(false),
		1 => Ok(true),
		_ => Err(serde::de::Error::invalid_value(
			serde::de::Unexpected::Signed(buf.into()),
			&"0 or 1",
		)),
	}
}
