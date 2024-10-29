mod config;
use toml::de::Error;

use crate::config::{Config};
use std::fs;
use std::process::exit;
use url::Url;
use std::process::Command;
use log::{error, warn};


fn main() {
    pretty_env_logger::init();

    let home = std::env::var("HOME").unwrap();
    let custom_config_path = std::env::var("SIMPLE_BROWSER_CONFIG");
    let file_path = custom_config_path
        .unwrap_or_else( |_e1| {
            warn!("Using $HOME/.config/simple-browser-chooser/configuration.toml as config location");
            format!("{home}/.config/simple-browser-chooser/configuration.toml").to_string()
        });

    let toml_string = match fs::read_to_string(file_path.clone()) {
        Ok(toml_string) => toml_string,
        Err(e) => {
            error!("Could not read configuration file [{}]: {}", file_path.to_string(), e);
            exit(2);
        }
    };

    let config = match validate(&toml_string) {
        Ok(resp) => resp,
        Err(e) => {
            error!("Failed to parse config: {}", e);
            exit(2);
        }
    };

    let mut arg_url = std::env::args().nth(1)
        .unwrap_or_else(|| "https://duckduckgo.com/".to_string());

    if !arg_url.starts_with("http://")
        && !arg_url.starts_with("https://")
        && !arg_url.starts_with("file://") {
        arg_url = format!("https://{}", arg_url);
    }

    let parsed_url = Url::parse(arg_url.as_str()).unwrap();
    let uri = parsed_url.to_string();
    let the_profile = config.get_profile_by_domain(parsed_url);
    let cmd = the_profile.exec;


    let code = Command::new(cmd)
        .args(the_profile.args.unwrap_or_else(Vec::new))
        .arg(uri)
        .status().unwrap();
    exit(code.code().unwrap());
}


fn validate(toml_str: &str) -> Result<Config, Error> {
    toml::from_str(toml_str)
}