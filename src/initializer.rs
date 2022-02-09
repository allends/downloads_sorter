use std::fs;
use std::env;
use serde::Deserialize;
use std::fs::ReadDir;
use std::path::PathBuf;
use config::{ Config, File, FileFormat};
use fs_extra::dir::create_all;

#[derive(Debug, Deserialize)]
struct SortingConfig {
  homework: String,
}

pub fn setup() -> (PathBuf, PathBuf, Vec<Vec<String>>, ReadDir, Vec<String> ) {
  // SECTION ONE: SET UP OS SPECIFIC STUFF
  println!("{} mode", env::consts::OS);
  let downloads_path = dirs::download_dir().unwrap();
  load_config();
  update_config();

  // CONFIGURATION
  let home_directory = dirs::home_dir().unwrap();
  let archive_base_directory = home_directory.as_path().join("Archives");

  // SECTION TWO: MAKE THE FOLDERS FOR EACH CATEGORY
  let categories: Vec<String> = vec!["App", "Archive", "Audio", "Book", "Doc", "Font", "Image", "Text", "Video", "Other"].into_iter().map(|name| String::from(name)).collect();

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

  (downloads_path, archive_base_directory, sorted_list, file_paths, categories )
}

fn update_config() {
  let mut settings = Config::new();
  let _ = settings.set("Homework", "hw-homework-problem");
  // println!("{:?}", settings);
}

fn load_config() {
  let config_path = dirs::home_dir().unwrap().join(".config").join("downloads_sorter.toml");
  let mut settings = Config::default();
  settings.merge(File::new(config_path.to_str().unwrap(), FileFormat::Toml)).unwrap();
  println!("{:?}", settings);

  let config: SortingConfig = settings.try_into().unwrap();
  println!("{:?}", config);

}