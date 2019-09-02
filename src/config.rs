use std::fs;

use serde_derive::Deserialize;
use toml;

/*
============================================ TOML Types ============================================
*/
#[derive(Debug, Deserialize)]
struct TomlConfig {
    ptp: Option<TomlPtp>,
    prefs: Option<UserPrefs>,
}

#[derive(Debug, Deserialize)]
struct TomlPtp {
    username: Option<String>,
    password: Option<String>,
    pass_key: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserPrefs {
    pub download_dir: Option<String>,
}
impl UserPrefs {
    pub fn new(download_dir: Option<String>) -> UserPrefs {
        UserPrefs { download_dir }
    }
}

#[derive(Debug)]
pub struct PtpKeys {
    pub username: String,
    pub password: String,
    pub pass_key: String,
}
impl PtpKeys {
    pub fn new(username: String, password: String, pass_key: String) -> PtpKeys {
        PtpKeys {
            username,
            password,
            pass_key,
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub ptp: PtpKeys,
    pub prefs: UserPrefs,
}
impl Config {
    pub fn new(ptp: PtpKeys, prefs: UserPrefs) -> Config {
        Config { ptp, prefs }
    }
}

fn read_config_file(config_filename: String) -> Result<TomlConfig, toml::de::Error> {
    let file_contents =
        fs::read_to_string(config_filename).expect("Something went wrong with reading the file!");
    toml::from_str(&file_contents)
}

pub fn get_config_from_file(config_filename: String) -> Config {
    let config_from_file = read_config_file(config_filename);

    let config = config_from_file.expect("Problem reading the config!");
    let ptp_keys = config
        .ptp
        .expect("Problem with the PTP fields in the config!");
    let keys = PtpKeys::new(
        ptp_keys.username.expect("Error with the username!"),
        ptp_keys.password.expect("Errow with the password!"),
        ptp_keys.pass_key.expect("Error with the pass_key!"),
    );

    let prefs = config
        .prefs
        .expect("Problem with the Prefs fields in the config!");
    let user_prefs = UserPrefs::new(prefs.download_dir);

    Config::new(keys, user_prefs)
}

pub fn get_config() -> Config {
    get_config_from_file("api.toml.key".to_string())
}
