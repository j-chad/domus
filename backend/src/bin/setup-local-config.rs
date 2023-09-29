use pasetors::keys::{AsymmetricKeyPair, Generate};
use pasetors::paserk::FormatAsPaserk;
use pasetors::version4::V4;
use serde::{Deserialize, Serialize};
use std::fs;

const LOCAL_CONFIG_FILE: &str = "config/local.toml";

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Auth {
    private_key: String,
    public_key: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct App {
    host: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Database {
    url: String,
    max_pool_size: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Settings {
    app: Option<App>,
    database: Option<Database>,
    auth: Option<Auth>,
}

fn main() {
    let mut settings = get_settings();

    add_auth_keys(&mut settings);

    write_settings(&settings);
}

fn get_settings() -> Settings {
    // create the file if it doesn't exist
    if fs::metadata(LOCAL_CONFIG_FILE).is_err() {
        fs::write(LOCAL_CONFIG_FILE, "").unwrap();
    }

    let contents = fs::read_to_string(LOCAL_CONFIG_FILE).unwrap();

    //deserialize the file into a Settings struct
    toml::from_str::<Settings>(&contents).unwrap()
}

fn write_settings(settings: &Settings) {
    let contents = toml::to_string_pretty(settings).unwrap();

    fs::write(LOCAL_CONFIG_FILE, contents).unwrap();
}

fn add_auth_keys(settings: &mut Settings) {
    let sk = AsymmetricKeyPair::<V4>::generate().unwrap();

    let mut public = String::new();
    sk.public.fmt(&mut public).unwrap();

    let mut private = String::new();
    sk.secret.fmt(&mut private).unwrap();

    settings.auth = Some(Auth {
        private_key: private,
        public_key: public,
    });
}
