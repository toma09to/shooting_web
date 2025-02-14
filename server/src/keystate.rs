use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyState {
    #[serde(rename = "ArrowLeft")]
    pub left: bool,
    #[serde(rename = "ArrowRight")]
    pub right: bool,
    #[serde(rename = "ArrowUp")]
    pub up: bool,
    #[serde(rename = " ")]
    pub space: bool,
}
