use std::io;

use thiserror::Error;
use tokio::sync::{mpsc, oneshot};

#[derive(Error, Debug)]
pub enum KdeConnectError {
	#[error(transparent)]
	Io(#[from] io::Error),
	#[error(transparent)]
	Mdns(#[from] mdns_sd::Error),
	#[error(transparent)]
	Rcgen(#[from] rcgen::Error),
	#[error(transparent)]
	Rustls(#[from] tokio_rustls::rustls::Error),
	#[error(transparent)]
	InvalidDnsName(#[from] tokio_rustls::rustls::pki_types::InvalidDnsNameError),
	#[error(transparent)]
	SerdeJson(#[from] serde_json::Error),
	#[error(transparent)]
	X509(#[from] x509_parser::nom::Err<x509_parser::error::X509Error>),
	#[error("Channel send error")]
	ChannelSendError,
	#[error("Channel recieve error")]
	ChannelRecvError,
	#[error("No peer certificates")]
	NoPeerCerts,
	#[error("Server task already started")]
	ServerAlreadyStarted,
	#[error("Failed to convert OsString to str")]
	OsStringConversionError,
	#[error("Failed to find port for payload transfer")]
	NoPayloadTransferPortFound,
	#[error("No filename")]
	NoFileName,
	#[error("Other")]
	Other,

	#[error("Device rejected pair")]
	DeviceRejectedPair,
	#[error("Already paired")]
	DeviceAlreadyPaired,
}

impl<T> From<mpsc::error::SendError<T>> for KdeConnectError {
	fn from(_: mpsc::error::SendError<T>) -> Self {
		Self::ChannelSendError
	}
}

impl From<oneshot::error::RecvError> for KdeConnectError {
	fn from(_: oneshot::error::RecvError) -> Self {
		Self::ChannelRecvError
	}
}
