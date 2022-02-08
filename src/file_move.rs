use indicatif::ProgressBar;
use fs_extra::move_items;
use std::path::PathBuf;

pub fn move_to_archive( archive_base_directory: PathBuf, sorted_list: Vec<Vec<PathBuf>>, categories: Vec<String>
) {
  let bar = ProgressBar::new(9);
  let mut options = fs_extra::dir::CopyOptions::new();
  options.skip_exist = true;
  for (index, list) in sorted_list.iter().enumerate() {
      bar.inc(1);
      let destination_folder = archive_base_directory.join(&categories[index]);
      let move_result = move_items(&list, &destination_folder, &options);
      match move_result {
          Ok(_) => continue,
          Err(e) => println!("Something went terribly wrong while moving {} files :'( INFO: {}", categories[index], e),
      }
      println!();
  }
  bar.finish();
}