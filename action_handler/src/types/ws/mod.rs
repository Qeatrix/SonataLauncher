use serde::{Serialize, Deserialize};
use tide_websockets::WebSocketConnection;

pub async fn send_ws_msg(ws: &WebSocketConnection, msg: serde_json::Value) -> Result<(), String> {
    // let msg_in_json = serde_json::to_string(&msg).unwrap();

    ws.send_json(&msg).await.map_err(|e| {
        println!("WebSocket message sending error: {}", e);
        e.to_string()
    })
}

// Info
#[derive(Serialize, Deserialize, Debug)]
pub struct InfoMessage {
    pub message: String,
    pub message_id: String,
    pub message_type: String,
    pub timestamp: String,
}


// Error
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessage {
    pub message: String,
    pub message_id: String,
    pub message_type: String,
    pub timestamp: String,
    pub details: ErrorDetails,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorDetails {
    pub reason: String,
    pub suggestions: Vec<String>,
}


// Progress
#[derive(Serialize, Deserialize, Debug)]
pub struct ProgressMessage {
    pub message_id: String,
    pub timestamp: String,
    pub data: ProgressData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProgressData {
    pub stage: String,
    pub determinable: bool,
    pub progress: Option<usize>,
    pub max: usize,
    pub status: String,
    pub target_type: String,
    pub target: ProgressTarget,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ProgressTarget {
    File {
        status: String,
        name: String,
        size_bytes: u64,
    },
    Dir {
        status: String,
        path: String,
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProgressTargetsList {
    pub message_id: String,
    pub message_type: String,
    pub timestamp: String,
    pub ids_list: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProgressFinishMessage {
    pub message_id: String,
    pub message_type: String,
    pub timestamp: String,
    pub data: ProgressFinishData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProgressFinishData {
    pub stage: String,
    pub status: String,
}