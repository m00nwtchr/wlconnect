use std::{collections::HashMap, path::PathBuf, pin::Pin, str::FromStr, sync::Arc};

use kdeconnect::{
	config::FsConfig,
	device::DeviceHandler,
	packets::{
		self, Battery, ConnectivityReport, MousepadEcho, MousepadKeyboardState, MousepadRequest,
		MprisPlayer, MprisRequestAction, Ping, Presenter, RunCommandItem, ShareRequestFile,
		ShareRequestUpdate, SystemVolume, SystemVolumeRequest, SystemVolumeStream, Telephony,
	},
};
use log::info;
use tokio::io::AsyncRead;
use tokio_stream::StreamExt;

struct MyDeviceHandler;

#[async_trait::async_trait]
impl DeviceHandler for MyDeviceHandler {
	async fn handle_ping(&mut self, packet: Ping) {
		info!("recieved ping packet: {:#?}", packet);
	}
	async fn handle_pair_status_change(&mut self, pair_status: bool) {
		// todo!();
	}
	async fn handle_battery(&mut self, packet: Battery) {
		// todo!();
	}
	async fn handle_clipboard_content(&mut self, content: String) {
		// todo!();
	}
	async fn handle_find_phone(&mut self) {
		// todo!();
	}
	async fn handle_connectivity_report(&mut self, packet: ConnectivityReport) {
		// todo!();
	}
	async fn handle_presenter(&mut self, packet: Presenter) {
		// todo!();
	}
	async fn handle_system_volume(&mut self, packet: SystemVolume) {
		// todo!();
	}
	async fn handle_system_volume_request(&mut self, packet: SystemVolumeRequest) {
		// todo!();
	}
	async fn handle_multi_file_share(&mut self, packet: ShareRequestUpdate) {
		// todo!();
	}
	async fn handle_file_share(
		&mut self,
		packet: ShareRequestFile,
		size: i64,
		data: Pin<Box<dyn AsyncRead + Sync + Send>>,
	) {
		// todo!();
	}
	async fn handle_url_share(&mut self, url: String) {
		// todo!();
	}
	async fn handle_text_share(&mut self, text: String) {
		// todo!();
	}
	async fn handle_mpris_player_list(&mut self, list: Vec<String>) {
		// todo!();
	}
	async fn handle_mpris_player_info(&mut self, player: MprisPlayer) {
		// todo!();
	}
	async fn handle_mpris_player_album_art(
		&mut self,
		player: String,
		art: Pin<Box<dyn AsyncRead + Sync + Send>>,
	) {
		// todo!();
	}
	async fn handle_mpris_player_action(&mut self, action: MprisRequestAction) {
		// todo!();
	}
	async fn handle_mousepad_request(&mut self, action: MousepadRequest) {
		// todo!();
	}
	async fn handle_mousepad_keyboard_state(&mut self, state: MousepadKeyboardState) {
		// todo!();
	}
	async fn handle_mousepad_echo(&mut self, echo: MousepadEcho) {
		// todo!();
	}
	async fn handle_command_list(&mut self, command_list: HashMap<String, RunCommandItem>) {
		// todo!();
	}
	async fn handle_command_request(&mut self, command_id: String) {
		// todo!();
	}
	async fn handle_telephony(&mut self, packet: Telephony) {
		// todo!();
	}
	async fn handle_telephony_mute_request(&mut self) {
		// todo!();
	}

	async fn handle_pairing_request(&mut self) -> bool {
		info!("Pairing");
		true
	}

	async fn get_battery(&mut self) -> Battery {
		Battery {
			charge: 69,
			is_charging: true,
			under_threshold: false,
		}
	}
	async fn get_clipboard_content(&mut self) -> String {
		String::new()
	}
	async fn get_connectivity_report(&mut self) -> ConnectivityReport {
		ConnectivityReport {
			signal_strengths: HashMap::new(),
		}
	}
	async fn get_system_volume(&mut self) -> Vec<SystemVolumeStream> {
		vec![]
	}
	async fn get_mpris_player_list(&mut self) -> Vec<String> {
		vec![]
	}
	async fn get_mpris_player(&mut self, player: String) -> Option<MprisPlayer> {
		None
	}
	async fn get_command_list(&mut self) -> HashMap<String, RunCommandItem> {
		HashMap::new()
	}

	async fn handle_exit(&mut self) {
		info!("Exit");
		// todo!();
	}
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	env_logger::init();

	let config_provider = Arc::new(
		FsConfig::new(
			PathBuf::from_str("./config")?,
			"server_cert".into(),
			"server_keypair".into(),
			"devices".into(),
		)
		.await?,
	);

	let (connect, client, mut device_stream) = kdeconnect::KdeConnect::new(
		"123_id".to_owned(),
		"m00n - Laptop".to_owned(),
		packets::DeviceType::Laptop,
		vec![
			packets::Battery::TYPE.to_string(),
			packets::MousepadRequest::TYPE.to_string(),
		],
		vec![kdeconnect::packets::BatteryRequest::TYPE.to_string()],
		config_provider.clone(),
	)
	.await?;

	tokio::spawn(async move { info!("server ret {:?}", connect.start_server().await) });

	info!("discovering");

	let mut abc = Vec::new();

	while let Some((mut device, client)) = device_stream.next().await {
		if let Ok(key) = device.get_verification_key().await {
			info!(
				"new device discovered: id {:?} name {:?} type {:?}",
				device.config.id, device.config.name, device.config.device_type
			);

			let handler = Box::new(MyDeviceHandler);

			tokio::spawn(async move {
				info!("handler task exited: {:?}", device.task(handler).await);
			});
			abc.push(client);
		}
	}

	Ok(())
}
