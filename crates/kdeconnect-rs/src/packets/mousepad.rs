use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::derive_type;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MousepadRequest {
	pub key: Option<String>,
	#[serde(rename = "specialKey")]
	pub special_key: Option<MousepadSpecialKey>,
	pub alt: Option<bool>,
	pub ctrl: Option<bool>,
	pub shift: Option<bool>,

	pub dx: Option<f32>,
	pub dy: Option<f32>,
	pub scroll: Option<bool>,
	pub singleclick: Option<bool>,
	pub doubleclick: Option<bool>,
	pub middleclick: Option<bool>,
	pub rightclick: Option<bool>,
	pub singlehold: Option<bool>,
	pub singlerelease: Option<bool>,

	#[serde(rename = "sendAck")]
	pub send_ack: Option<bool>,
}

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum MousepadSpecialKey {
	Backspace = 1,
	Tab = 2,
	DpadLeft = 4,
	DpadUp = 5,
	DpadRight = 6,
	DpadDown = 7,
	PageUp = 8,
	PageDown = 9,
	Home = 10,
	End = 11,
	Enter = 12,
	Delete = 13,
	Escape = 14,
	SysRq = 15,
	ScrollLock = 16,
	F1 = 21,
	F2 = 22,
	F3 = 23,
	F4 = 24,
	F5 = 25,
	F6 = 26,
	F7 = 27,
	F8 = 28,
	F9 = 29,
	F10 = 30,
	F11 = 31,
	F12 = 32,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct MousepadKeyboardState {
	pub state: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MousepadEcho {
	pub key: Option<String>,
	#[serde(rename = "specialKey")]
	pub special_key: Option<MousepadSpecialKey>,
	pub alt: Option<bool>,
	pub ctrl: Option<bool>,
	pub shift: Option<bool>,

	pub dx: Option<f32>,
	pub dy: Option<f32>,
	pub scroll: Option<bool>,
	pub singleclick: Option<bool>,
	pub doubleclick: Option<bool>,
	pub middleclick: Option<bool>,
	pub rightclick: Option<bool>,
	pub singlehold: Option<bool>,
	pub singlerelease: Option<bool>,

	#[serde(rename = "isAck")]
	pub is_ack: bool,
}

derive_type!(MousepadRequest, "kdeconnect.mousepad.request");
derive_type!(MousepadKeyboardState, "kdeconnect.mousepad.keyboardstate");
derive_type!(MousepadEcho, "kdeconnect.mousepad.echo");
