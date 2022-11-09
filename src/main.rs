use color_eyre::Report;
use control_packets::{ControlPacketType, Encodable};
use disconnect_packet::DisconnectPacket;
use ping_packets::{PingReqPacket, PingRespPacket};
use publish_packet::PublishPacket;
use std::time::Duration;
pub(crate) use std::{
    io::{self, BufRead, Write},
    net::TcpStream,
};

use tokio::{
    signal,
    time::{interval_at, Instant},
};
use tracing::{debug, error, info, warn};
use tracing_subscriber::EnvFilter;

use crate::conn_ack_packet::ConnAck;

pub mod conn_ack_packet;
pub mod connect_packet;
pub mod control_packets;
pub mod disconnect_packet;
pub mod ping_packets;
pub mod publish_packet;

pub struct MyQuteKittyClient {
    client_id: String,
    server_address: Option<String>,
    tcp_stream: Option<TcpStream>,
}

impl Clone for MyQuteKittyClient {
    fn clone(&self) -> Self {
        let server_address_clone = match &self.server_address {
            Some(address) => Some(address.to_string()),
            None => None,
        };

        let tcp_stream_clone = match &self.tcp_stream {
            Some(stream) => {
                let stream_clone = stream.try_clone();
                let x = match stream_clone {
                    Ok(clone) => Some(clone),
                    Err(_) => None,
                };
                x
            }
            None => None,
        };

        MyQuteKittyClient {
            client_id: self.client_id.to_string(),
            server_address: server_address_clone,
            tcp_stream: tcp_stream_clone,
        }
    }
}

impl MyQuteKittyClient {
    pub fn new(client_id: &str) -> Self {
        return MyQuteKittyClient {
            client_id: client_id.to_owned(),
            server_address: None,
            tcp_stream: None,
        };
    }

    pub fn read_packet(&mut self) -> Result<(), std::io::Error> {
        if let Some(stream) = &mut self.tcp_stream {
            let mut reader = io::BufReader::new(stream);

            let received: Vec<u8> = reader.fill_buf()?.to_vec();
            if received.len() > 0 {
                let packet_type: u8 = (received[0] & 0xf0) >> 4;
                match packet_type.into() {
                    ControlPacketType::ConnAck => {
                        let conn_ack_packet = ConnAck::from(received.as_slice());
                        info!("received a {:?}", conn_ack_packet);
                    }
                    ControlPacketType::Connect => todo!(),
                    ControlPacketType::Publish => todo!(),
                    ControlPacketType::PubAck => info!("Received a PubAck"),
                    ControlPacketType::PubRec => todo!(),
                    ControlPacketType::PubRel => todo!(),
                    ControlPacketType::PubComp => todo!(),
                    ControlPacketType::Subscribe => todo!(),
                    ControlPacketType::SubAck => info!("Received a SubAck"),
                    ControlPacketType::Unsubscribe => todo!(),
                    ControlPacketType::UnsubAck => info!("Received an UnsubAck"),
                    ControlPacketType::PingReq => info!("Received a PingReq"),
                    ControlPacketType::PingResp => {
                        let ping_resp_packet = PingRespPacket::from(received.as_slice());
                        info!("received a {:?}", ping_resp_packet);
                    }
                    ControlPacketType::Disconnect => todo!(),
                    _ => warn!("Received an unknown control packet!"),
                }
            }
        }
        Ok(())
    }

    pub fn connect(&mut self, address: String) -> Result<(), std::io::Error> {
        self.server_address = Some(address);
        match TcpStream::connect(self.server_address.as_ref().unwrap()) {
            Ok(mut stream) => {
                let connect_packet_bytes = connect_packet::Builder::new()
                    .client_id(&self.client_id)
                    .build()
                    .unwrap()
                    .encode();
                match stream.write_all(&connect_packet_bytes) {
                    Ok(_) => {
                        self.tcp_stream = Some(stream);
                        return self.read_packet();
                    }
                    Err(error) => {
                        return Err(error);
                    }
                }
            }
            Err(error) => {
                return Err(error);
            }
        }
    }

    pub fn disconnect(&mut self) -> Result<(), std::io::Error> {
        let disconnect_packet_bytes = DisconnectPacket::new().encode();
        if let Some(stream) = &mut self.tcp_stream {
            match stream.write_all(&disconnect_packet_bytes) {
                Ok(_) => return Ok(()),
                Err(error) => return Err(error),
            }
        }

        return Ok(());
    }

    pub fn subscribe(&self, _topic: &str) -> Result<(), std::io::Error> {
        return Ok(());
    }

    pub fn unsubscribe(&self, _topic: &str) -> Result<(), std::io::Error> {
        return Ok(());
    }

    pub fn publish(&mut self, topic: &str, payload: &str) -> Result<(), std::io::Error> {
        let publish_packet_bytes =
            PublishPacket::new(0b0000_0000.into(), topic, payload.as_bytes()).encode();

        if let Some(stream) = &mut self.tcp_stream {
            match stream.write_all(&publish_packet_bytes) {
                Ok(_) => return Ok(()),
                Err(error) => return Err(error),
            }
        }

        return Ok(());
    }

    pub fn ping(&mut self) -> Result<(), std::io::Error> {
        let ping_req_packet_bytes = PingReqPacket::new().encode();
        if let Some(stream) = &mut self.tcp_stream {
            match stream.write_all(&ping_req_packet_bytes) {
                Ok(_) => {
                    return self.read_packet();
                }
                Err(error) => return Err(error),
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;
    info!("My Qute kiTTy v0.0.1");
    info!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⣿⡷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀");
    info!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⡿⠋⠈⠻⣮⣳⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀");
    info!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣴⣾⡿⠋⠀⠀⠀⠀⠙⣿⣿⣤⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀");
    info!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⣶⣿⡿⠟⠛⠉⠀⠀⠀⠀⠀⠀⠀⠈⠛⠛⠿⠿⣿⣷⣶⣤⣄⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀");
    info!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣴⣾⡿⠟⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠛⠻⠿⣿⣶⣦⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀");
    info!("⠀⠀⠀⣀⣠⣤⣤⣀⡀⠀⠀⣀⣴⣿⡿⠛⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⠿⣿⣷⣦⣄⡀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣤⣄⠀⠀");
    info!("⢀⣤⣾⡿⠟⠛⠛⢿⣿⣶⣾⣿⠟⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⠿⣿⣷⣦⣀⣀⣤⣶⣿⡿⠿⢿⣿⡀⠀");
    info!("⣿⣿⠏⠀⢰⡆⠀⠀⠉⢿⣿⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠻⢿⡿⠟⠋⠁⠀⠀⢸⣿⠇⠀");
    info!("⣿⡟⠀⣀⠈⣀⡀⠒⠃⠀⠙⣿⡆⠀⠀⠀⠀⠀⠀⠀⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⠇⠀");
    info!("⣿⡇⠀⠛⢠⡋⢙⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⣿⣿⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀");
    info!("⣿⣧⠀⠀⠀⠓⠛⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⠛⠋⠀⠀⢸⣧⣤⣤⣶⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⣿⡿⠀⠀");
    info!("⣿⣿⣤⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠉⠻⣷⣶⣶⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣿⣿⠁⠀⠀");
    info!("⠈⠛⠻⠿⢿⣿⣷⣶⣦⣤⣄⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣴⣿⣷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⣿⡏⠀⠀⠀");
    info!("⠀⠀⠀⠀⠀⠀⠀⠉⠙⠛⠻⠿⢿⣿⣷⣶⣦⣤⣄⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠿⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢿⣿⡄⠀⠀");
    info!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠙⠛⠻⠿⢿⣿⣷⣶⣦⣤⣄⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢿⣿⡄⠀");
    info!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠛⠛⠿⠿⣿⣷⣶⣶⣤⣤⣀⡀⠀⠀⠀⢀⣴⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢿⡿⣄");
    info!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠛⠛⠿⠿⣿⣷⣶⡿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣿⣹");
    info!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣀⠀⠀⠀⠀⠀⠀⢸⣧");
    info!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⣿⣆⠀⠀⠀⠀⠀⠀⢀⣀⣠⣤⣶⣾⣿⣿⣿⣿⣤⣄⣀⡀⠀⠀⠀⣿");
    info!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⢿⣻⣷⣶⣾⣿⣿⡿⢯⣛⣛⡋⠁⠀⠀⠉⠙⠛⠛⠿⣿⣿⡷⣶⣿");

    let mut mqtt_client = MyQuteKittyClient::new("mqutekitty-client");
    let server_address = String::from("127.0.0.1:1883");
    mqtt_client.connect(server_address)?;
    let mut mqtt_client_clone = mqtt_client.clone();

    let (timer_notification_channel_tx, timer_notification_channel_rx) = std::sync::mpsc::channel();
    let instant = Instant::now();
    let mut timer = interval_at(instant, Duration::from_secs(5));

    // We have to perioically ping the MQTT server. Currently that operation is blocking (i.e. doesn't use async io (..I know)).
    // We can't spawn it in a tokio task isn't a good practice, because we would end up blocking the event loop.
    // So, we'll just use a normal std thread, which will loop and wait on a channel (timer_notification_channel).
    // Then, we'll have a very light weight async tokio task (timed_event_task), which will async wait on a tokio interval and engage the
    // timer notification channel on every timer tick. This will keep the tokio task event loop happy, because the processing in the tokio task is minimal.
    let ping_thread = std::thread::Builder::new()
        .name("ping_thread".to_string())
        .spawn(move || loop {
            match timer_notification_channel_rx.recv() {
                Ok(_) => {
                    debug!("TNC received OK");
                }
                Err(error) => {
                    error!("TNC errored out! {}", error);
                    break;
                }
            }

            match mqtt_client.ping() {
                Ok(_) => {
                    debug!("Ping OK");
                }
                Err(error) => {
                    error!("Error pinging MQTT server! {}", error);
                    break;
                }
            }
        });

    let timed_event_task = tokio::spawn(async move {
        loop {
            timer.tick().await;
            match timer_notification_channel_tx.send(0) {
                Ok(_) => {
                    // life's good
                    debug!("TNC send OK");
                }
                Err(error) => {
                    error!("Error sending on TNC! {}", error);
                }
            };
        }
    });

    match mqtt_client_clone.publish("myqutekitty/test", "first message"){
        Ok(_) => debug!("Pub OK"),
        Err(error) => error!("Error publishing! {:?}", error)
    }

    tokio::select! {
        _ = signal::ctrl_c() => {
            mqtt_client_clone.disconnect()?;
            warn!("Exiting..");
        }
        _ = timed_event_task => {info!("Pinged..")}
    };

    info!("Meow..?!");

    drop(ping_thread);

    Ok(())
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
