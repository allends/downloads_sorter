extern crate toml;

use crate::config::{load_config, update_config, Config};
use std::fs;
use std::env;

use std::fs::ReadDir;
use std::path::{PathBuf, Path};

use fs_extra::dir::create_all;



pub fn setup() -> (PathBuf, PathBuf, Vec<Vec<String>>, ReadDir, Vec<String>, Config ) {
  // SECTION ONE: SET UP OS SPECIFIC STUFF
  println!("{} mode", env::consts::OS);
  let config = load_config();
  let downloads_path = dirs::download_dir().unwrap_or(PathBuf::from(&config.sortingdir));
  if !Path::new(&downloads_path).is_dir() || downloads_path.to_str().unwrap().eq("none") {
    println!("There was a problem with the sorting folder (where files are being read from)! Check the downloads_sorter.toml to make sure this is correct!");
  }
  update_config();

  // CONFIGURATION
  let archive_base_directory = match config.archivedir.as_str() {
    "none" => {
      match dirs::home_dir() {
        Some(env_home_dir) => env_home_dir.as_path().join("Archives"),
        None => {
          println!("There was a problem with the sorting folder (where files are being read from)! Check the downloads_sorter.toml to make sure this is correct!");
          std::process::exit(0)
        },
      }
    },
    _ => {
      if !Path::new(&config.archivedir).is_dir() {
        println!("There was a problem with the sorting folder (where files are being read from)! Check the downloads_sorter.toml to make sure this is correct!");
        std::process::exit(0)
      }
      PathBuf::from(&config.archivedir)
    }
  };

  // SECTION TWO: MAKE THE FOLDERS FOR EACH CATEGORY
  let categories: Vec<String> = config.folderconfig.iter().map(|rule| rule.name.clone()).collect();

  let mut sorted_list: Vec<Vec<String>> = Vec::new();
  for (index, category) in categories.iter().enumerate() {
      sorted_list.push(Vec::new());
      let category_path = archive_base_directory.join(category);
      let create_check_result = create_all(&category_path, false);
      match create_check_result {
          Ok(_) => print!("{}: ✅ ", categories[index]),
          Err(e) => println!("Oh no! {}", e),
      }
      assert!(category_path.exists());
  }
  println!();
  let file_paths = fs::read_dir(downloads_path.as_path()).unwrap();
  println!("All systems check! Time for takeoff 🚀");

  (downloads_path, archive_base_directory, sorted_list, file_paths, categories, config)
}

