use std::time::Duration;
use lapin::{options::*, BasicProperties, Connection, ConnectionProperties, Channel};
use lapin::types::ReplyCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoteDTO {
    uuid: String,
    client_id: String,
    closed_at: u128,
}

impl RemoteDTO {
    pub fn new(uuid: String, client_id: String, closed_at: u128) -> Self {
        RemoteDTO { uuid, client_id, closed_at }
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
