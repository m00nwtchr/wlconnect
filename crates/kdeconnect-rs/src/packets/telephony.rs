#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TelephonyEvent {
	MissedCall,
	Ringing,
	Talking,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Telephony {
	pub event: TelephonyEvent,
	pub contact_name: Option<String>,
	pub phone_number: Option<String>,
	pub phone_thumbnail: Option<String>,
	pub is_cancel: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TelephonyRequestMute {}

derive_type!(Telephony, "kdeconnect.telephony");
derive_type!(TelephonyRequestMute, "kdeconnect.telephony.request_mute");
