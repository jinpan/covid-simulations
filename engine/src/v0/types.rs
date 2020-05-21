use serde::Serialize;

#[derive(Copy, Clone, Serialize, Debug)]
pub enum Mask {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "n95")]
    N95,
}
