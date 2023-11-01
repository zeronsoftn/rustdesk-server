use lapin::{options::*, BasicProperties, Connection, ConnectionProperties, Channel};
use lapin::types::ReplyCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoteDTO {
    msg: MsgType,
    msg_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
enum MsgType {
    Close(RemoteCloseDTO),
    Connect(RemoteConnectDTO),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoteCloseDTO {
    connection_id: String,
    closed_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoteConnectDTO {
    connection_id: String,
    client_id: String,
    public_key: String,
    connected_at: u64,
    ip: String,
    port: String,
}

impl RemoteDTO {
    pub fn new_close(dto: RemoteCloseDTO) -> Self {
        RemoteDTO {
            msg: MsgType::Close(dto),
            msg_type: "close".to_string(), // Set the appropriate message type string
        }
    }

    pub fn new_connect(dto: RemoteConnectDTO) -> Self {
        RemoteDTO {
            msg: MsgType::Connect(dto),
            msg_type: "connect".to_string(), // Set the appropriate message type string
        }
    }
}


impl RemoteCloseDTO {
    pub fn new(connection_id: String, closed_at: u64) -> Self {
        RemoteCloseDTO { connection_id, closed_at }
    }
}

impl RemoteConnectDTO {
    pub fn new(connection_id: String, client_id: String, public_key: String, connected_at: u64, ip: String, port: String) -> Self {
        RemoteConnectDTO { connection_id, client_id, public_key, connected_at, ip, port }
    }
}

pub async fn send(dto: RemoteDTO) {

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    let v = serde_json::to_value(&dto).unwrap();

    let addr = std::env::var("RABBITMQ_HOST").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());
    let conn = get_connection(&addr).await;

    let channel_a = get_channel(conn).await;
    channel_a
        .basic_publish(
            "amq.direct",
            "zeromon.extitg.remote.event",
            BasicPublishOptions::default(),
            v.to_string().as_ref(),
            BasicProperties::default(),
        ).await.expect("Failed to publish message");

    channel_a.close(ReplyCode::default(), "good").await.expect("Failed to close Channel");
}

async fn get_connection(addr: &str) -> Connection {
    return Connection::connect(
        addr,
        ConnectionProperties::default(),
    ).await.expect(&format!("fail to get connection {}", addr));
}

async fn get_channel(connection: Connection) -> Channel {
    return connection.create_channel().await.expect("fail to get channel");
}
