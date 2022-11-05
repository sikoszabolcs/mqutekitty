use std::time::Duration;
use futures::future::abortable;
use tokio::time::interval;
pub(crate) use std::{
    io::{self, BufRead, Write},
    net::TcpStream,
};
use tokio::signal;
use control_packets::{ControlPacketType, Encodable};
use disconnect_packet::DisconnectPacket;
use ping_packets::{PingReqPacket, PingRespPacket};

pub mod conn_ack_packet;
pub mod connect_packet;
pub mod control_packets;
pub mod disconnect_packet;
pub mod ping_packets;

pub struct MyQuteKittyClient {
    client_id: String,
    server_address: Option<String>,
    tcp_stream: Option<TcpStream>,
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
            let received: Vec<u8> = reader.fill_buf().unwrap().to_vec();
            if received.len() > 0 {
                let packet_type: u8 = (received[0] & 0xf0) >> 4;
                match packet_type.into() {
                    ControlPacketType::ConnAck => {
                        let conn_ack_packet: Vec<u8> = received.into();
                        println!("received a {:?}", conn_ack_packet);
                    }
                    ControlPacketType::Connect => todo!(),
                    ControlPacketType::Publish => todo!(),
                    ControlPacketType::PubAck => println!("Received a PubAck"),
                    ControlPacketType::PubRec => todo!(),
                    ControlPacketType::PubRel => todo!(),
                    ControlPacketType::PubComp => todo!(),
                    ControlPacketType::Subscribe => todo!(),
                    ControlPacketType::SubAck => println!("Received a SubAck"),
                    ControlPacketType::Unsubscribe => todo!(),
                    ControlPacketType::UnsubAck => println!("Received an UnsubAck"),
                    ControlPacketType::PingReq => println!("Received a PingReq"),
                    ControlPacketType::PingResp => {
                        let ping_resp_packet = PingRespPacket::from(received.as_slice());
                        println!("received a {:?}", ping_resp_packet);
                    }
                    ControlPacketType::Disconnect => todo!(),
                    _ => println!("Received an unknown control packet!"),
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

    pub fn publish(&self, _topic: &str, _payload: &str) -> Result<(), std::io::Error> {
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
async fn main() -> io::Result<()> {
    println!("My Qute kiTTy v0.0.1");
    println!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⣿⡷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀");
    println!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⡿⠋⠈⠻⣮⣳⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀");
    println!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣴⣾⡿⠋⠀⠀⠀⠀⠙⣿⣿⣤⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀");
    println!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⣶⣿⡿⠟⠛⠉⠀⠀⠀⠀⠀⠀⠀⠈⠛⠛⠿⠿⣿⣷⣶⣤⣄⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀");
    println!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣴⣾⡿⠟⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠛⠻⠿⣿⣶⣦⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀");
    println!("⠀⠀⠀⣀⣠⣤⣤⣀⡀⠀⠀⣀⣴⣿⡿⠛⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⠿⣿⣷⣦⣄⡀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣤⣄⠀⠀");
    println!("⢀⣤⣾⡿⠟⠛⠛⢿⣿⣶⣾⣿⠟⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⠿⣿⣷⣦⣀⣀⣤⣶⣿⡿⠿⢿⣿⡀⠀");
    println!("⣿⣿⠏⠀⢰⡆⠀⠀⠉⢿⣿⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠻⢿⡿⠟⠋⠁⠀⠀⢸⣿⠇⠀");
    println!("⣿⡟⠀⣀⠈⣀⡀⠒⠃⠀⠙⣿⡆⠀⠀⠀⠀⠀⠀⠀⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⠇⠀");
    println!("⣿⡇⠀⠛⢠⡋⢙⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⣿⣿⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀");
    println!("⣿⣧⠀⠀⠀⠓⠛⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⠛⠋⠀⠀⢸⣧⣤⣤⣶⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⣿⡿⠀⠀");
    println!("⣿⣿⣤⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠉⠻⣷⣶⣶⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣿⣿⠁⠀⠀");
    println!("⠈⠛⠻⠿⢿⣿⣷⣶⣦⣤⣄⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣴⣿⣷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⣿⡏⠀⠀⠀");
    println!("⠀⠀⠀⠀⠀⠀⠀⠉⠙⠛⠻⠿⢿⣿⣷⣶⣦⣤⣄⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠿⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢿⣿⡄⠀⠀");
    println!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠙⠛⠻⠿⢿⣿⣷⣶⣦⣤⣄⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢿⣿⡄⠀");
    println!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠛⠛⠿⠿⣿⣷⣶⣶⣤⣤⣀⡀⠀⠀⠀⢀⣴⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢿⡿⣄");
    println!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠛⠛⠿⠿⣿⣷⣶⡿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣿⣹");
    println!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣀⠀⠀⠀⠀⠀⠀⢸⣧");
    println!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⣿⣆⠀⠀⠀⠀⠀⠀⢀⣀⣠⣤⣶⣾⣿⣿⣿⣿⣤⣄⣀⡀⠀⠀⠀⣿");
    println!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⢿⣻⣷⣶⣾⣿⣿⡿⢯⣛⣛⡋⠁⠀⠀⠉⠙⠛⠛⠿⣿⣿⡷⣶⣿");
    
    let mut mqtt_client = MyQuteKittyClient::new("mqutekitty-client");
    let server_address = String::from("127.0.0.1:1883");
    mqtt_client.connect(server_address)?;

    let (job, handle) = abortable(ping(mqtt_client));
    tokio::spawn(job);

    match signal::ctrl_c().await {
        Ok(()) => {
            println!("Got out!");
            handle.abort();
            mqtt_client.disconnect();
        }
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            handle.abort();
        }
    }

    Ok(())
}

async fn ping(mut mqtt_client: MyQuteKittyClient) {
    let mut interval = interval(Duration::from_secs(5));
    loop {
        interval.tick().await;
        mqtt_client.ping();
    }
}