mod config;
use toml::de::Error;

use crate::config::{Config};
use std::fs;
use std::process::{exit, Stdio};
use url::Url;
use std::process::Command;
use flexi_logger::{Duplicate, FileSpec, WriteMode};
use log::{error, warn, debug, LevelFilter, info};

fn main() {
    let home = std::env::var("HOME").unwrap();
    let default_folder = format!("{home}/.config/simple-browser-chooser");
    configure_logging(&default_folder);
    let config = build_config(&default_folder);
    let arg_url = get_treated_parameter();
    let mut parsed_url = Url::parse(&arg_url).unwrap();

    if parsed_url.domain().is_some() && parsed_url.domain().unwrap() == "www.google.com" && parsed_url.path().ends_with("url") {
        let qps: std::collections::HashMap<_, _> = parsed_url.query_pairs().into_owned().collect();
        if qps.contains_key("url") {
            let new_uri = qps.get("url").expect("url parameter is expected");
            parsed_url = Url::parse(new_uri.as_str()).unwrap();
        }
    }

    let the_profile = config.get_profile_by_domain(&parsed_url);
    let cmd = the_profile.exec;

    let mut uri = parsed_url.to_string();
    if uri.contains("%20") {
        uri = format!("'{}'", parsed_url);
    }
    debug!("Uri for configured browser is: {}", uri);

    let mut cmd_builder = Command::new(cmd);
    if the_profile.args.is_some() {
        cmd_builder.args(the_profile.args.unwrap());
    }
    cmd_builder.arg(uri);
    let code = cmd_builder
        .status().unwrap();
    exit(code.code().unwrap());
}


fn validate(toml_str: &str) -> Result<Config, Error> {
    toml::from_str(toml_str)
}

fn configure_logging(default_folder: &str) {
    let log_file = format!("{default_folder}/sbclog.log");
    let _logger = flexi_logger::Logger::try_with_env().unwrap()
        .log_to_file(FileSpec::try_from(log_file).unwrap()).print_message()
        .duplicate_to_stdout(Duplicate::Info)
        .write_mode(WriteMode::Direct)
        .start().unwrap();
}

fn build_config(default_folder: &str) -> Config {
    let custom_config_path = std::env::var("SIMPLE_BROWSER_CONFIG");
    let file_path = custom_config_path
        .unwrap_or_else(|_e1| {
            warn!("Using $HOME/.config/simple-browser-chooser/configuration.toml as config location");
            format!("{default_folder}/configuration.toml").to_string()
        });

    let toml_string = match fs::read_to_string(file_path.clone()) {
        Ok(toml_string) => toml_string,
        Err(e) => {
            error!("Could not read configuration file [{}]: {}", file_path.to_string(), e);
            exit(2);
        }
    };

    match validate(&toml_string) {
        Ok(resp) => resp,
        Err(e) => {
            error!("Failed to parse config: {}", e);
            exit(2);
        }
    }
}

fn get_treated_parameter() -> String {
    let mut arg_url = std::env::args().nth(1)
        .unwrap_or_else(|| "https://duckduckgo.com/".to_string());
    debug!("Received arg uri: {}", arg_url);

    arg_url = arg_url.replace("%20", " ");

    if !arg_url.starts_with("http://")
        && !arg_url.starts_with("https://")
        && !arg_url.starts_with("file://") {
        arg_url = format!("https://{}", arg_url);
    }

    debug!("Treated arg uri: {}", arg_url);

    arg_url
}