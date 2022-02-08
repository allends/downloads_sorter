use std::path::PathBuf;
use std::env;
use std::fs;
use std::fs::ReadDir;
use fs_extra::dir::create_all;

pub fn setup() -> (PathBuf, PathBuf, Vec<Vec<PathBuf>>, ReadDir, Vec<String> ) {
  // SECTION ONE: SET UP OS SPECIFIC STUFF
  println!("{} mode", env::consts::OS);
  let downloads_path = dirs::download_dir().unwrap();

  // TODO: MAKE THIS CHANGE BASED ON OS
  let archive_base_buf = dirs::home_dir().unwrap();
  let archive_base_directory = archive_base_buf.as_path().join("Archives");

  // SECTION TWO: MAKE THE FOLDERS FOR EACH CATEGORY
  let categories: Vec<String> = vec!["App", "Archive", "Audio", "Book", "Doc", "Font", "Image", "Text", "Video", "Other"].into_iter().map(|name| String::from(name)).collect();

  let mut sorted_list: Vec<Vec<PathBuf>> = Vec::new();
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