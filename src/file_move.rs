use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use fs_extra::file::move_file;
use std::path::PathBuf;

pub fn move_to_archive(downloads_path: &PathBuf, archive_base_directory: PathBuf, sorted_list: Vec<Vec<String>>, categories: Vec<String>, file_count: u64
) {
  let bar = ProgressBar::new(file_count);
  bar.set_style(ProgressStyle::default_bar()
    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
    .progress_chars("##-"));
  let options = fs_extra::file::CopyOptions::new();

  let mut failed_moves: Vec<String> = Vec::new();

  // Loop over every category
  for (index, list) in sorted_list.iter().enumerate() {
    for file in list {
      let destination_folder = archive_base_directory.join(&categories[index]).join(file);
      let destination_folder_path = destination_folder.as_path();
      let origin_folder = downloads_path.join(file);
      let origin_folder_path = origin_folder.as_path();
      let move_result = move_file(&origin_folder_path, &destination_folder_path, &options);
      match move_result {
        Ok (_) => bar.inc(1),
        Err (e) => {
          let file_and_failure = format!("{} {}", file.to_string(), e);
          failed_moves.push(file_and_failure);
        },
      }
    }
  }
  bar.finish_at_current_pos();
  if failed_moves.len() > 0 {
    println!("Had issues moving the following files:");
    for failed_move in failed_moves {
      println!("{}", failed_move);
    }
  } else {
    println!("Mission accomplished, sorted and moved {} of {} files!", bar.position(), file_count);
  }
}