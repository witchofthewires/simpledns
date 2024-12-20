use std::error::Error;
use std::fs;
use std::io::ErrorKind;
use yaml_rust::YamlLoader;
use std::path::Path;

use crate::log_info;

extern crate shellexpand;

#[derive(Clone, Debug)]
pub struct DnsSettings {
  pub listening_port: u16,
  pub remote_lookup_port: u16,
  pub database_file: String,
  pub thread_count: u32,
  pub use_udp: bool,
  pub use_tcp: bool,
}

impl DnsSettings {

  pub fn load_from_file(filename: String) -> Result<Self, Box<dyn Error>> {

    let error_str = "Aw man, there was an issue while opening the config file '{".to_owned() + filename.as_str() + "}' :(";
    let contents = fs::read_to_string(shellexpand::full(filename.as_str()).unwrap().to_string())
                   .expect(&error_str);
    log_info!("Loaded from config file '{}'...", filename.as_str());

    let yaml_files = &YamlLoader::load_from_str(contents.as_str())?;
    let config_settings_option = &yaml_files.get(0);
    match config_settings_option {
      Some(config_settings) => {
        let listening_port = match config_settings["listening-port"].as_i64() {
          Some(x) => x as u16,
          None => 53,
        };
        let remote_lookup_port = match config_settings["remote-lookup-port"].as_i64() {
          Some(x) => x as u16,
          None => 42069,
        };
        let thread_count = match config_settings["thread-count"].as_i64() {
          Some(x) => x as u32,
          None => 1, // TODO is this the best default?
        };
        let use_udp = match config_settings["use-udp"].as_bool() {
          Some(x) => x,
          None => true,
        };
        let use_tcp = match config_settings["use-tcp"].as_bool() {
          Some(x) => x,
          None => false, // TODO should be set true when this functionality is working properly
        };

        let database_file = shellexpand::full(
            config_settings["database-file"]
              .as_str()
              .unwrap_or_else(|| "~/.config/simpledns/simpledns.sqlite.db")
          )
          .unwrap()
          .to_string();

        Ok(DnsSettings {
          listening_port,
          remote_lookup_port,
          database_file,
          thread_count,
          use_udp,
          use_tcp,
        })
      }
      None => Err(Box::new(std::io::Error::new(
        ErrorKind::Other,
        "Parsing the config file lead to no yaml documents :(",
      ))),
    }
  }

  pub fn load_default() -> Result<Self, Box<dyn Error>> {
    let filenames = ["./dns.config.yaml", "~/.config/simpledns/dns.config.yaml", "/etc/simpledns/dns.config.yaml"];
    let mut config_file = "";
    for filename in filenames {    
      if Path::new(filename).exists() { config_file = filename; break; }
    }
    if config_file == "" { panic!("No valid config file given"); }
    Self::load_from_file(String::from(config_file))
  }
}
