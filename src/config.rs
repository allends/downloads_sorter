use serde::Deserialize;
use std::process::exit;
use std::fs;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Config {
    pub archivedir: String,
    pub sortingdir: String,
    pub folderconfig: Vec<FolderConfig>,
}

#[derive(Debug, Deserialize, Default)]
#[allow(dead_code)]
pub struct FolderConfig {
    pub name: String,
    pub keywords: Vec<String>,
    pub matchertypes: Vec<String>,
}

impl Default for Config {
  fn default() -> Self {
    Config {
      archivedir: "none".to_string(),
      sortingdir: "none".to_string(),
      folderconfig: vec![
        FolderConfig {
          name: "".to_string(),
          keywords: vec![],
          matchertypes: vec![],
        }
      ]
    }
  }
}

pub fn update_config() {
  // TODO let the user change the config from the command line
}

pub fn load_config() -> Config {
  let config_path = dirs::home_dir().unwrap().join(".config").join("downloads_sorter.toml");

  let config_contents = fs::read_to_string(&config_path).unwrap_or("".to_string());
  let config: Config = match toml::from_str(&config_contents) {
    Ok(value) => value,
    Err(err) => {
      println!("Config Error: {}", err);
      exit(0)
    },
  };
  config
}