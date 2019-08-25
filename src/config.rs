use std::fs;

use serde_derive::Deserialize;
use toml;

/*
============================================ TOML Types ============================================
*/
#[derive(Debug, Deserialize)]
struct TomlConfig {
    ptp: Option<TomlPtp>,
}

#[derive(Debug, Deserialize)]
struct TomlPtp {
    username: Option<String>,
    password: Option<String>,
    passkey: Option<String>,
}

#[derive(Debug)]
pub struct PtpKeys {
    username: String,
    password: String,
    passkey: String,
}
impl PtpKeys {
    pub fn new(username: String, password: String, passkey: String) -> PtpKeys {
        PtpKeys {
            username,
            password,
            passkey,
        }
    }
}

fn read_config_file(config_filename: String) -> Result<TomlConfig, toml::de::Error> {
    let file_contents = fs::read_to_string(config_filename)
        .expect("Something went wrong with reading the file!");
    toml::from_str(&file_contents)
}

pub fn get_config_from_file(config_filename: String) -> PtpKeys {
    let config_from_file = read_config_file(config_filename);

    let config = config_from_file.expect("Problem reading the config!");
    let ptp_keys = config.ptp.expect("Problem with the PTP fields in the config!");

    PtpKeys::new(
        ptp_keys.username.expect("Error with the username!"),
        ptp_keys.password.expect("Errow with the password!"),
        ptp_keys.passkey.expect("Error with the passkey!"),
    )
}

pub fn get_config() -> PtpKeys {
    get_config_from_file("api.toml.key".to_string())
}
