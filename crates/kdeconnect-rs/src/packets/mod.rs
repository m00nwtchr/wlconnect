use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

mod battery;
mod clipboard;
mod connectivity_report;
mod findmyphone;
mod identity;
mod mousepad;
mod mpris;
mod pair;
mod ping;
mod presenter;
mod runcommand;
mod share;
mod systemvolume;
mod telephony;

pub use battery::*;
pub use clipboard::*;
pub use connectivity_report::*;
pub use findmyphone::FindMyPhone;
pub use identity::*;
pub use mousepad::*;
pub use mpris::*;
pub use pair::*;
pub use ping::Ping;
pub use presenter::Presenter;
pub use runcommand::*;
pub use share::*;
pub use systemvolume::*;
pub use telephony::*;

pub const PROTOCOL_VERSION: usize = 7;

pub const ALL_CAPABILITIES: &[&str] = &[
	Ping::TYPE,
	Battery::TYPE,
	BatteryRequest::TYPE,
	Clipboard::TYPE,
	ClipboardConnect::TYPE,
	FindMyPhone::TYPE,
	ConnectivityReport::TYPE,
	ConnectivityReportRequest::TYPE,
	Presenter::TYPE,
	SystemVolume::TYPE,
	SystemVolumeRequest::TYPE,
	ShareRequest::TYPE,
	Mpris::TYPE,
	MprisRequest::TYPE,
	MousepadRequest::TYPE,
	MousepadEcho::TYPE,
	MousepadKeyboardState::TYPE,
	RunCommand::TYPE,
	RunCommandRequest::TYPE,
];

#[macro_export]
macro_rules! derive_type {
	($struct:ty, $type:literal) => {
		impl crate::packets::PacketType for $struct {
			fn get_type_self(&self) -> &'static str {
				$type
			}
		}
		impl $struct {
			pub const TYPE: &'static str = $type;
		}
	};
}

pub trait PacketType {
	fn get_type_self(&self) -> &'static str;
}

struct DeserializeIDVisitor;

impl serde::de::Visitor<'_> for DeserializeIDVisitor {
	type Value = u128;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		formatter.write_str("an u128 or a string")
	}

	fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(u128::from(v))
	}

	fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(v)
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		FromStr::from_str(v).map_err(serde::de::Error::custom)
	}
}

fn deserialize_id<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
	D: Deserializer<'de>,
{
	deserializer.deserialize_any(DeserializeIDVisitor)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Packet {
	// kdeconnect-kde set this to a string but it's supposed to be an int... :(
	// kdeconnect-android follows the protocol!! so we crash!!
	// so we coerce to a u128
	#[serde(deserialize_with = "deserialize_id")]
	pub id: u128,
	#[serde(rename = "type")]
	pub packet_type: String,
	pub body: Value,

	#[serde(flatten, skip_serializing_if = "Option::is_none")]
	pub payload: Option<PacketPayload>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PacketPayload {
	#[serde(rename = "payloadSize")]
	pub size: i64,
	#[serde(rename = "payloadTransferInfo")]
	pub transfer_info: PacketPayloadTransferInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PacketPayloadTransferInfo {
	pub port: u16,
}

// to_value should never fail, as Serialize will always be successful and packets should never
// contain non-string keys anyway
#[macro_export]
macro_rules! make_packet {
	($packet:ident) => {
		Packet {
			id: $crate::util::get_time_ms(),
			packet_type: $packet.get_type_self().to_string(),
			body: serde_json::value::to_value($packet).expect("packet was invalid"),
			payload: None,
		}
	};
}

#[macro_export]
macro_rules! make_packet_payload {
	($packet:ident, $payload_size:expr, $payload_port:expr) => {
		crate::packets::Packet {
			id: $crate::util::get_time_ms(),
			packet_type: $packet.get_type_self().to_string(),
			body: serde_json::value::to_value($packet).expect("packet was invalid"),
			payload: Some(crate::packets::PacketPayload {
				size: $payload_size,
				transfer_info: crate::packets::PacketPayloadTransferInfo {
					port: $payload_port,
				},
			}),
		}
	};
}

#[macro_export]
macro_rules! make_packet_str {
	($packet:ident) => {
		serde_json::to_string(&make_packet!($packet)).map(|x| x + "\n")
	};
}

#[macro_export]
macro_rules! make_packet_str_payload {
	($packet:ident, $payload_size:expr, $payload_port:expr) => {
		serde_json::to_string(&make_packet_payload!($packet, $payload_size, $payload_port))
			.map(|x| x + "\n")
	};
}
