use lapin::types::ReplyCode;
use lapin::{options::*, BasicProperties, Channel, Connection, ConnectionProperties};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;

static ADDR: Lazy<String> = Lazy::new(|| {
    let host = std::env::var("RABBITMQ_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = std::env::var("RABBITMQ_PORT").unwrap_or_else(|_| "5672".into());
    let user = std::env::var("RABBITMQ_USERNAME").unwrap_or_else(|_| "guest".into());
    let password = std::env::var("RABBITMQ_PASSWORD").unwrap_or_else(|_| "guest".into());
    format!("amqp://{}:{}@{}:{}", user, password, host, port)
});

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoteDTO {
    msg: Value,
    event_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoteCloseDTO {
    connection_id: String,
    closed_at: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoteConnectDTO {
    connection_id: String,
    public_key: String,
    connected_at: u64,
    client_id: String,
    client_ip: String,
    client_port: String,
}

impl RemoteDTO {
    pub fn new(msg: Value, event_type: String) -> Self {
        RemoteDTO {
            msg: msg,
            event_type: event_type, // Set the appropriate message type string
        }
    }
}

impl RemoteCloseDTO {
    pub fn new(connection_id: String, closed_at: u64) -> Self {
        RemoteCloseDTO {
            connection_id,
            closed_at,
        }
    }
}

impl RemoteConnectDTO {
    pub fn new(
        connection_id: String,
        client_id: String,
        public_key: String,
        connected_at: u64,
        client_ip: String,
        client_port: String,
    ) -> Self {
        RemoteConnectDTO {
            connection_id,
            client_id,
            public_key,
            connected_at,
            client_ip,
            client_port,
        }
    }
}

pub async fn send_close(dto: RemoteCloseDTO) {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    let v = serde_json::to_value(&dto).unwrap();

    send(v, format!("close")).await;
}

pub async fn send_connect(dto: RemoteConnectDTO) {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    let v = serde_json::to_value(&dto).unwrap();

    send(v, format!("connect")).await;
}

async fn send(v: Value, event_type: String) {
    let dto = RemoteDTO::new(v, event_type);
    let msg = serde_json::to_value(&dto).unwrap();

    let conn = get_connection(&ADDR).await;

    let channel_a = get_channel(conn).await;
    channel_a
        .basic_publish(
            "amq.direct",
            "zerox.extitg.remote.event",
            BasicPublishOptions::default(),
            msg.to_string().as_ref(),
            BasicProperties::default(),
        )
        .await
        .expect("Failed to publish message");

    channel_a
        .close(ReplyCode::default(), "good")
        .await
        .expect("Failed to close Channel");
}

async fn get_connection(addr: &str) -> Connection {
    println!("Connecting to: {}", addr);

    return Connection::connect(addr, ConnectionProperties::default())
        .await
        .expect(&format!("fail to get connection {}", addr));
}

async fn get_channel(connection: Connection) -> Channel {
    return connection
        .create_channel()
        .await
        .expect("fail to get channel");
}
