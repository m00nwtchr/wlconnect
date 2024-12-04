use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::derive_type;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectivityReport {
	pub signal_strengths: HashMap<String, ConnectivityReportSignal>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectivityReportSignal {
	pub network_type: ConnectivityReportNetworkType,
	pub signal_strength: i32,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum ConnectivityReportNetworkType {
	#[serde(rename = "GSM")]
	Gsm,
	#[serde(rename = "CDMA")]
	Cdma,
	#[serde(rename = "iDEN")]
	Iden,
	#[serde(rename = "UMTS")]
	Umts,
	#[serde(rename = "CDMA2000")]
	Cdma2000,
	#[serde(rename = "EDGE")]
	Edge,
	#[serde(rename = "GPRS")]
	Gprs,
	#[serde(rename = "HSPA")]
	Hspa,
	#[serde(rename = "LTE")]
	Lte,
	#[serde(rename = "5G")]
	FiveG,
	#[serde(rename = "Unknown")]
	Unknown,
}

impl Display for ConnectivityReportNetworkType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use ConnectivityReportNetworkType as C;
		match self {
			C::Gsm => write!(f, "GSM"),
			C::Cdma => write!(f, "CDMA"),
			C::Iden => write!(f, "iDEN"),
			C::Umts => write!(f, "UMTS"),
			C::Cdma2000 => write!(f, "CDMA2000"),
			C::Edge => write!(f, "EDGE"),
			C::Gprs => write!(f, "GPRS"),
			C::Hspa => write!(f, "HSPA"),
			C::Lte => write!(f, "LTE"),
			C::FiveG => write!(f, "5G"),
			C::Unknown => write!(f, "Unknown"),
		}
	}
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct ConnectivityReportRequest {}

derive_type!(ConnectivityReport, "kdeconnect.connectivity_report");
derive_type!(
	ConnectivityReportRequest,
	"kdeconnect.connectivity_report.request"
);
