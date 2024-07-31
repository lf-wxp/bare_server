use std::{
  collections::HashMap,
  fs::{read_dir, read_to_string},
  io::Result,
  sync::OnceLock,
};

use clap::Parser;

use crate::params::Params;

static mut CONFIG: OnceLock<HashMap<String, String>> = OnceLock::new();

fn parse_config() -> Result<HashMap<String, String>> {
  let config_path = Params::parse().config;
  let mut params = HashMap::new();
  for entry in read_dir(config_path)? {
    let entry = entry?;
    let path = entry.path();
    let name = entry.file_name().to_string_lossy().to_string();
    if path.is_file() {
      let content = read_to_string(&path)?.replace('\n', "");
      params.insert(name, content);
    }
  }
  Ok(params)
}

pub fn set_config() {
  unsafe {
    let config = parse_config().unwrap_or(HashMap::<String, String>::default());
    let _ = CONFIG.set(config);
  }
}

pub fn get_config(key: &str) -> String {
  unsafe {
    CONFIG.get().map_or("".to_string(), |config| {
      config.get(key).cloned().unwrap_or("".to_string())
    })
  }
}
