use moonlight::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "serde")]
pub enum Config {
    Get,
    SetOrReplace(String),
    New(String),
    Del,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "serde")]
pub enum UpMsg {
    Connect(String, u32),
    Config(String, Config),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "serde")]
pub enum DownMsg {
    ConnectResult(Result<String, String>),
    ConfigResult(Result<String, String>),
}
